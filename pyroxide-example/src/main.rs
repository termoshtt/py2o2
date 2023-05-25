use pyo3::prelude::*;

pub mod example {
    use pyo3::{prelude::*, types::PyString};

    pub fn a1(py: Python<'_>) -> PyResult<()> {
        let m = py.import("example")?;
        let a1 = m.getattr("a1")?;
        a1.call((), None)?;
        Ok(())
    }

    pub fn a2(py: Python<'_>, x: i64) -> PyResult<()> {
        let m = py.import("example")?;
        let a2 = m.getattr("a2")?;
        a2.call((x,), None)?;
        Ok(())
    }

    pub fn a3(py: Python<'_>, y: &str, z: f32) -> PyResult<()> {
        let m = py.import("example")?;
        let a3 = m.getattr("a3")?;
        a3.call((y, z), None)?;
        Ok(())
    }

    pub fn a4(py: Python<'_>) -> PyResult<i64> {
        let m = py.import("example")?;
        let a4 = m.getattr("a4")?;
        let out = a4.call((), None)?.extract()?;
        Ok(out)
    }

    pub fn a5(py: Python<'_>, x: i64) -> PyResult<&PyString> {
        let m = py.import("example")?;
        let a5 = m.getattr("a5")?;
        let out = a5.call((x,), None)?.extract()?;
        Ok(out)
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
        example::a3(py, "homhom", 3.1415)?;

        // With return values
        let a4_out = example::a4(py)?;
        dbg!(a4_out);
        let a5_out = example::a5(py, 33)?;
        dbg!(a5_out);
        Ok(())
    })
}
