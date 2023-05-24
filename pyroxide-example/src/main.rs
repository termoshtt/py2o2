use pyo3::prelude::*;

pub mod example {
    use pyo3::prelude::*;

    fn load_pymodule(py: Python<'_>) -> PyResult<&PyModule> {
        // Append current directory `.` to sys.path
        let sys = py.import("sys")?;
        let paths = sys.getattr("path")?;
        paths.call_method("append", ("",), None)?;

        py.import("example")
    }

    pub fn a1(py: Python<'_>) -> PyResult<()> {
        let m = load_pymodule(py)?;
        let a1 = m.getattr("a1")?;
        a1.call0()?;
        Ok(())
    }
}

fn main() -> PyResult<()> {
    Python::with_gil(|py| {
        example::a1(py)?;
        Ok(())
    })
}
