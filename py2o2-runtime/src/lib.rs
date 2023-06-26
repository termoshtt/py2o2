pub use pyo3;

use pyo3::{
    conversion::{FromPyObject, IntoPy},
    types::PyType,
    Py, PyAny, PyResult, Python,
};
use std::{any::TypeId, collections::BTreeMap};

#[derive(Debug, PartialEq, Clone)]
pub enum Enum2<T1, T2> {
    Item1(T1),
    Item2(T2),
}

static mut TYPE_MAPPING: BTreeMap<TypeId, Py<PyType>> = BTreeMap::new();

impl<'s, T1, T2> FromPyObject<'s> for Enum2<T1, T2>
where
    T1: 'static + FromPyObject<'s> + IntoPy<Py<PyAny>> + Default,
    T2: 'static + FromPyObject<'s> + IntoPy<Py<PyAny>> + Default,
{
    fn extract(ob: &'s PyAny) -> PyResult<Self> {
        let ty = ob.get_type();

        let t1_ty = unsafe { TYPE_MAPPING.entry(TypeId::of::<T1>()) }.or_insert_with(|| {
            Python::with_gil(|py| {
                let value = T1::default().into_py(py);
                value.as_ref(py).get_type().extract().unwrap()
            })
        });
        if ty.is(t1_ty) {
            return Ok(Enum2::Item1(T1::extract(ob)?));
        }

        let t2_ty = unsafe { TYPE_MAPPING.entry(TypeId::of::<T2>()) }.or_insert_with(|| {
            Python::with_gil(|py| {
                let value = T2::default().into_py(py);
                value.as_ref(py).get_type().extract().unwrap()
            })
        });
        if ty.is(t2_ty) {
            return Ok(Enum2::Item2(T2::extract(ob)?));
        }

        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

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

            Ok(())
        })?;
        Ok(())
    }
}
