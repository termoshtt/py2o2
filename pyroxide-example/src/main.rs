use pyo3::prelude::*;

fn main() -> PyResult<()> {
    Python::with_gil(|py| {
        // Append current directory `.` to sys.path
        let sys = py.import("sys")?;
        let paths = sys.getattr("path")?;
        paths.call_method("append", ("",), None)?;

        let ex = py.import("example")?;
        let a1 = ex.getattr("a1")?;
        a1.call0()?;
        Ok(())
    })
}
