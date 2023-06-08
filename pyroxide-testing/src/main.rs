use pyo3::prelude::*;

include!(concat!(env!("OUT_DIR"), "/example.rs"));
include!(concat!(env!("OUT_DIR"), "/type_aliases.rs"));

const PYTHON_ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../python/");

fn main() -> PyResult<()> {
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
