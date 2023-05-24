use pyo3::prelude::*;

pub mod example {
    use pyo3::prelude::*;
    pub fn a1(py: Python<'_>) -> PyResult<()> {
        let m = py.import("example")?;
        let a1 = m.getattr("a1")?;
        a1.call0()?;
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
        Ok(())
    })
}
