pub use pyo3;

use pyo3::{
    conversion::FromPyObject,
    exceptions::PyTypeError,
    type_object::PyTypeInfo,
    types::{PyFloat, PyLong, PyString},
    PyAny, PyResult,
};

pub trait AsPyType {
    fn is_type_of(obj: &PyAny) -> bool;
}

impl AsPyType for i32 {
    fn is_type_of(obj: &PyAny) -> bool {
        PyLong::is_type_of(obj)
    }
}
impl AsPyType for i64 {
    fn is_type_of(obj: &PyAny) -> bool {
        PyLong::is_type_of(obj)
    }
}

impl AsPyType for f32 {
    fn is_type_of(obj: &PyAny) -> bool {
        PyFloat::is_type_of(obj)
    }
}
impl AsPyType for f64 {
    fn is_type_of(obj: &PyAny) -> bool {
        PyFloat::is_type_of(obj)
    }
}

impl AsPyType for &PyString {
    fn is_type_of(obj: &PyAny) -> bool {
        PyString::is_type_of(obj)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Enum2<T1, T2> {
    Item1(T1),
    Item2(T2),
}

impl<'s, T1, T2> FromPyObject<'s> for Enum2<T1, T2>
where
    T1: AsPyType + FromPyObject<'s>,
    T2: AsPyType + FromPyObject<'s>,
{
    fn extract(ob: &'s PyAny) -> PyResult<Self> {
        if T1::is_type_of(ob) {
            return Ok(Enum2::Item1(ob.extract()?));
        }
        if T2::is_type_of(ob) {
            return Ok(Enum2::Item2(ob.extract()?));
        }
        Err(PyTypeError::new_err("Type mismatch"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use pyo3::{IntoPy, Py, Python};

    #[test]
    fn convert() -> Result<()> {
        Python::with_gil(|py| -> Result<()> {
            let v1: i32 = 42;
            let v2: f32 = 2.123;

            let p1: Py<PyAny> = v1.into_py(py);
            let e: Enum2<i32, f32> = p1.extract(py)?;
            assert_eq!(e, Enum2::Item1(v1));

            let p2: Py<PyAny> = v2.into_py(py);
            let e: Enum2<i32, f32> = p2.extract(py)?;
            assert_eq!(e, Enum2::Item2(v2));

            let p3: Py<PyAny> = "test".into_py(py);
            assert!(p3.extract::<Enum2<i32, f32>>(py).is_err());

            Ok(())
        })?;
        Ok(())
    }
}
