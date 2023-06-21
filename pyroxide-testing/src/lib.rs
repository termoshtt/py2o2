#![allow(dead_code, unused_imports)]

use anyhow::Result;
use pyo3::{prelude::*, types::*};

pub mod example;
pub mod type_aliases;

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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UserId(pub i64);

#[test]
fn type_aliases() -> Result<()> {
    std::env::set_var("PYTHONPATH", PYTHON_ROOT);

    Python::with_gil(|py| {
        let out = type_aliases::scale(py, 2.0, PyList::new(py, [1.0, 2.0, 3.0]))?;
        dbg!(out);

        let id = UserId(124);
        let out: &PyString = py
            .import("type_aliases")?
            .getattr("get_user_name")?
            .call((id.0,), None)?
            .extract()?;
        assert_eq!(out.to_str()?, "ID = 124");

        Ok(())
    })
}
