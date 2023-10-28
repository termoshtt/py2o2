use pyo3::{exceptions::PyTypeError, prelude::*, type_object::PyTypeInfo, types::*, PyNativeType};
use std::fmt;

pub struct Module(Py<PyAny>);
unsafe impl PyNativeType for Module {}
unsafe impl PyTypeInfo for Module {
    const NAME: &'static str = "Module";
    const MODULE: Option<&'static str> = Some("ast");
    type AsRefTarget = Self;
    fn type_object_raw(py: Python<'_>) -> *mut pyo3::ffi::PyTypeObject {
        let typ: &PyType = py
            .import("ast")
            .expect("ast module must exists")
            .getattr("Module")
            .expect("ast.Module not found")
            .extract()
            .expect("ast.Module must be a type");
        typ.as_type_ptr()
    }
}

impl fmt::Debug for Module {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Py({})", self.0.to_string())
    }
}

impl<'source> FromPyObject<'source> for Module {
    fn extract(inner: &'source PyAny) -> PyResult<Self> {
        if Module::is_exact_type_of(inner) {
            Ok(Module(inner.into()))
        } else {
            Err(PyTypeError::new_err("Not a ast.Module"))
        }
    }
}

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
