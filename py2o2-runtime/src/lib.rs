pub use pyo3;
use pyo3::{conversion::*, exceptions::*, prelude::*, type_object::*, types::*};

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

    fn path() -> String {
        let mut name = String::new();
        for module in Self::MODULE.iter() {
            name.push_str(module);
            name.push('.');
        }
        name.push_str(Self::NAME);
        name
    }
}

#[macro_export]
macro_rules! import_pytype {
    ($pymodule:ident . $pytype:ident) => {
        pub struct $pytype(::pyo3::Py<::pyo3::PyAny>);

        impl PyTypeInfoUser<1> for $pytype {
            const NAME: &'static str = stringify!($pytype);
            const MODULE: [&'static str; 1] = [stringify!($pymodule)];
            fn type_object(py: ::pyo3::Python<'_>) -> ::pyo3::PyResult<&::pyo3::types::PyType> {
                let module = py.import(stringify!($pymodule))?;
                let ty = module.getattr(stringify!($pytype))?;
                ty.extract()
            }
        }

        impl ::std::fmt::Debug for $pytype {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(f, "Py({})", self.0.to_string())
            }
        }

        impl ::pyo3::FromPyObject<'_> for $pytype {
            fn extract(inner: &::pyo3::PyAny) -> ::pyo3::PyResult<Self> {
                if Module::is_exact_type_of(inner)? {
                    Ok($pytype(inner.into()))
                } else {
                    Err(::pyo3::exceptions::PyTypeError::new_err(format!(
                        "Not a {}",
                        Self::path()
                    )))
                }
            }
        }
    };
}

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

impl<T: PyTypeInfo> AsPyType for Py<T> {
    fn is_type_of(obj: &PyAny) -> bool {
        T::is_type_of(obj)
    }
}

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
                    let inner: $t = ob.extract()?;
                    return Ok($enum::$item(inner.into()));
                }
                )*
                Err(PyTypeError::new_err("Type mismatch"))
            }
        }
    };
}
define_enum!(Enum2; Item1, Item2; T1, T2);
define_enum!(Enum3; Item1, Item2, Item3; T1, T2, T3);

pub fn as_pycfunc<F, Input, Output>(py: Python<'_>, f: F) -> PyResult<&PyCFunction>
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
