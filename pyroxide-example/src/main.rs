use pyo3::prelude::*;

pyroxide::import!(example);

pub mod example {
    use pyo3::{prelude::*, types::PyString};

    pub fn a1(py: Python<'_>) -> PyResult<()> {
        let _ = py.import("example")?.getattr("a1")?.call((), None)?;
        Ok(())
    }

    pub fn a2(py: Python<'_>, x: i64) -> PyResult<()> {
        let _ = py.import("example")?.getattr("a2")?.call((x,), None)?;
        Ok(())
    }

    pub fn a3(py: Python<'_>, y: &str, z: f32) -> PyResult<()> {
        let _ = py.import("example")?.getattr("a3")?.call((y, z), None)?;
        Ok(())
    }

    pub fn a4(py: Python<'_>) -> PyResult<i64> {
        let result = py.import("example")?.getattr("a4")?.call((), None)?;
        Ok(result.extract()?)
    }

    pub fn a5(py: Python<'_>, x: i64) -> PyResult<&PyString> {
        let result = py.import("example")?.getattr("a5")?.call((x,), None)?;
        Ok(result.extract()?)
    }

    pub fn a6(py: Python<'_>) -> PyResult<(i64, &PyString)> {
        let result = py.import("example")?.getattr("a6")?.call((), None)?;
        Ok(result.extract()?)
    }

    pub fn a7(py: Python<'_>, x: i64) -> PyResult<(i64, &PyString, f64)> {
        let result = py.import("example")?.getattr("a7")?.call((x,), None)?;
        Ok(result.extract()?)
    }
}

fn main() -> PyResult<()> {
    // Add a path where `example.py` exists
    let project_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap();
    std::env::set_var("PYTHONPATH", project_root);

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
