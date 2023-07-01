#![allow(dead_code, unused_imports)]

use anyhow::Result;
use py2o2_runtime::Enum2;
use pyo3::{impl_::pyfunction::wrap_pyfunction_impl, intern, prelude::*, types::*, Python};

pub mod example;
pub mod type_aliases;
pub mod union;

const PYTHON_ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../python/");

#[test]
fn example() -> Result<()> {
    std::env::set_var("PYTHONPATH", PYTHON_ROOT);

    Python::with_gil(|py| {
        // No return value
        example::a1(py)?;
        example::a2(py, 57)?;
        example::a3(py, "homhom", 3.0)?;

        // With return values
        dbg!(example::a4(py)?);
        dbg!(example::a5(py, 33)?);
        dbg!(example::a6(py)?);
        dbg!(example::a7(py, 112)?);
        Ok(())
    })
}

#[test]
fn type_aliases() -> Result<()> {
    std::env::set_var("PYTHONPATH", PYTHON_ROOT);

    Python::with_gil(|py| {
        let out = type_aliases::scale(py, 2.0, PyList::new(py, [1.0, 2.0, 3.0]))?;
        dbg!(out);

        let id = type_aliases::UserId(124);
        let out = type_aliases::get_user_name(py, id)?;
        assert_eq!(out.as_ref(py).to_str()?, "ID = 124");

        Ok(())
    })
}

#[test]
fn union() -> Result<()> {
    std::env::set_var("PYTHONPATH", PYTHON_ROOT);
    Python::with_gil(|py| {
        let out = union::f_new(py, 42)?;
        assert!(matches!(out, Enum2::Item1(_)));

        let out = union::f_new(py, "homhom")?;
        assert!(matches!(out, Enum2::Item2(_)));

        Ok(())
    })
}

fn feeder<'py>(py: Python<'py>, f: &'py PyCFunction) -> PyResult<()> {
    py.import("callable")?.getattr("feeder")?.call((f,), None)?;
    Ok(())
}

fn caller<'py>(py: Python<'py>, f: &'py PyCFunction) -> PyResult<()> {
    py.import("callable")?.getattr("caller")?.call((f,), None)?;
    Ok(())
}

fn as_pycfunc<'py, F, Input, Output>(py: Python<'py>, f: F) -> PyResult<&'py PyCFunction>
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

#[test]
fn callable() -> Result<()> {
    std::env::set_var("PYTHONPATH", PYTHON_ROOT);
    Python::with_gil(|py| {
        let f = as_pycfunc(py, |_input: [usize; 0]| -> String {
            static mut COUNT: usize = 0;
            let current = unsafe {
                COUNT += 1;
                COUNT
            };
            format!("{}", current)
        })?;
        feeder(py, f)?;

        let g = as_pycfunc(py, |(a, b): (i64, f64)| a as f64 * b)?;
        caller(py, g)?;

        Ok(())
    })
}
