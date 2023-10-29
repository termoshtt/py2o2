use pyo3::{exceptions::PyTypeError, prelude::*, types::*};
use std::fmt;

/// Types defined in Python user land (not in Python's runtime)
///
/// Different from [`pyo3::type_object::PyTypeInfo`],
/// - this is safe trait since this does not require `AsRefTarget`
/// - returns `PyResult<PyType>` to represent a case where given Python type does not exist.
pub trait PyTypeInfoUser<const N: usize> {
    const NAME: &'static str;
    const MODULE: [&'static str; N];
    fn type_object(py: Python<'_>) -> PyResult<&PyType>;

    fn is_type_of(value: &PyAny) -> PyResult<bool> {
        let py = value.py();
        let ty = Self::type_object(py)?;
        value.is_instance(ty)
    }

    fn is_exact_type_of(value: &PyAny) -> PyResult<bool> {
        let py = value.py();
        let ty = Self::type_object(py)?;
        Ok(value.is_exact_instance(ty))
    }
}

macro_rules! import_pytype {
    ($pymodule:ident . $pytype:ident) => {
        pub struct $pytype(Py<PyAny>);

        impl PyTypeInfoUser<1> for $pytype {
            const NAME: &'static str = stringify!($pytype);
            const MODULE: [&'static str; 1] = [stringify!($pymodule)];
            fn type_object(py: Python<'_>) -> PyResult<&PyType> {
                let module = py.import(stringify!($pymodule))?;
                let ty = module.getattr(stringify!($pytype))?;
                ty.extract()
            }
        }

        impl ::std::fmt::Debug for $pytype {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "Py({})", self.0.to_string())
            }
        }

        impl ::pyo3::FromPyObject<'_> for $pytype {
            fn extract(inner: &PyAny) -> PyResult<Self> {
                if Module::is_exact_type_of(inner)? {
                    Ok($pytype(inner.into()))
                } else {
                    Err(PyTypeError::new_err(format!(
                        "Not a {}.{}",
                        stringify!($pymodule),
                        stringify!($pytype)
                    )))
                }
            }
        }
    };
}

import_pytype!(ast.Module);
import_pytype!(ast.Expression);

pub fn parse(py: Python<'_>, input: &str) -> PyResult<Module> {
    let ast = py.import("ast")?;
    let parse = ast.getattr("parse")?;
    let parsed = parse.call1((input,))?;
    parsed.extract()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        Python::with_gil(|py| {
            let m = parse(py, "def foo(): pass").unwrap();
            dbg!(m);
        });
        panic!()
    }
}
