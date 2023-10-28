use pyo3::{exceptions::PyTypeError, prelude::*, type_object::PyTypeInfo, types::*, PyNativeType};
use std::fmt;

macro_rules! import_pytype {
    ($pymodule:literal, $pytype:ident) => {
        pub struct $pytype(Py<PyAny>);

        unsafe impl PyNativeType for $pytype {}
        unsafe impl PyTypeInfo for $pytype {
            const NAME: &'static str = stringify!($pytype);
            const MODULE: Option<&'static str> = Some($pymodule);
            type AsRefTarget = Self;
            fn type_object_raw(py: Python<'_>) -> *mut pyo3::ffi::PyTypeObject {
                let typ: &PyType = py
                    .import($pymodule)
                    .expect(&format!("Python module {} not found", $pymodule))
                    .getattr(stringify!($pytype))
                    .expect(&format!("Python type {} not found", stringify!($pytype)))
                    .extract()
                    .expect(&format!(
                        "{}.{} is not a type",
                        $pymodule,
                        stringify!($pytype)
                    ));
                typ.as_type_ptr()
            }
        }

        impl ::std::fmt::Debug for $pytype {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "Py({})", self.0.to_string())
            }
        }

        impl ::pyo3::FromPyObject<'_> for $pytype {
            fn extract(inner: &PyAny) -> PyResult<Self> {
                if Module::is_exact_type_of(inner) {
                    Ok($pytype(inner.into()))
                } else {
                    Err(PyTypeError::new_err(format!(
                        "Not a {}.{}",
                        $pymodule,
                        stringify!($pytype)
                    )))
                }
            }
        }
    };
}

import_pytype!("ast", Module);
import_pytype!("ast", Expression);

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
