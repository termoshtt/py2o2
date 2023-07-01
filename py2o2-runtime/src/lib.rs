pub use pyo3;

use pyo3::{conversion::*, exceptions::*, prelude::*, type_object::*, types::*};

pub trait AsPyType {
    fn is_type_of(obj: &PyAny) -> bool;
}

macro_rules! impl_as_py_type {
    ($ty:ty, $pyty:ty) => {
        impl AsPyType for $ty {
            fn is_type_of(obj: &PyAny) -> bool {
                <$pyty>::is_type_of(obj)
            }
        }
    };
}
impl_as_py_type!(i64, PyLong);
impl_as_py_type!(f64, PyFloat);
impl_as_py_type!(&PyString, PyString);

macro_rules! define_enum {
    ($enum:ident; $($item:ident),* ; $($t:ident),*) => {
        #[derive(Debug, PartialEq, Clone)]
        pub enum $enum<$($t),*> {
            $($item($t)),*
        }

        impl<'s, $($t),*> FromPyObject<'s> for $enum<$($t),*>
        where
            $($t: AsPyType + FromPyObject<'s>),*
        {
            fn extract(ob: &'s PyAny) -> PyResult<Self> {
                $(
                if $t::is_type_of(ob) {
                    return Ok($enum::$item(ob.extract()?));
                }
                )*
                Err(PyTypeError::new_err("Type mismatch"))
            }
        }
    };
}
define_enum!(Enum2; Item1, Item2; T1, T2);
define_enum!(Enum3; Item1, Item2, Item3; T1, T2, T3);

pub fn as_pycfunc<'py, F, Input, Output>(py: Python<'py>, f: F) -> PyResult<&'py PyCFunction>
where
    F: Fn(Input) -> Output + Send + 'static,
    for<'a> Input: FromPyObject<'a>,
    Output: IntoPy<Py<PyAny>>,
{
    PyCFunction::new_closure(
        py,
        None,
        None,
        move |args: &PyTuple, _kwargs: Option<&PyDict>| -> PyResult<Py<PyAny>> {
            let input: Input = args.extract()?;
            let out = f(input);
            Python::with_gil(|py2| Ok(out.into_py(py2)))
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use pyo3::{IntoPy, Py, Python};

    #[test]
    fn convert() -> Result<()> {
        Python::with_gil(|py| -> Result<()> {
            let v1: i64 = 42;
            let v2: f64 = 2.123;

            let p1: Py<PyAny> = v1.into_py(py);
            let e: Enum2<i64, f64> = p1.extract(py)?;
            assert_eq!(e, Enum2::Item1(v1));

            let p2: Py<PyAny> = v2.into_py(py);
            let e: Enum2<i64, f64> = p2.extract(py)?;
            assert_eq!(e, Enum2::Item2(v2));

            let p3: Py<PyAny> = "test".into_py(py);
            assert!(p3.extract::<Enum2<i64, f64>>(py).is_err());

            Ok(())
        })?;
        Ok(())
    }
}
