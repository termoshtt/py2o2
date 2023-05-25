use pyo3::prelude::*;

pub mod example {
    use pyo3::prelude::*;
    pub fn a1(py: Python<'_>) -> PyResult<()> {
        let m = py.import("example")?;
        let a1 = m.getattr("a1")?;
        a1.call0()?;
        Ok(())
    }

    pub fn a2(py: Python<'_>, x: i64) -> PyResult<()> {
        let m = py.import("example")?;
        let a2 = m.getattr("a2")?;
        a2.call1((x,))?;
        Ok(())
    }

    pub fn a3(py: Python<'_>, y: &str, z: f32) -> PyResult<()> {
        let m = py.import("example")?;
        let a3 = m.getattr("a3")?;
        a3.call((y, z), None)?;
        Ok(())
    }
}

fn main() -> PyResult<()> {
    // Add a path where `example.py` exists
    let project_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap();
    std::env::set_var("PYTHONPATH", project_root);

    Python::with_gil(|py| {
        example::a1(py)?;
        example::a2(py, 57)?;
        example::a3(py, "homhom", 3.1415)?;
        Ok(())
    })
}
