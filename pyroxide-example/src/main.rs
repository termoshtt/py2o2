use pyo3::prelude::*;

pyroxide::import!(example);

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
