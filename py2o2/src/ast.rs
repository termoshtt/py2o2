use py2o2_runtime::{import_pytype, PyTypeInfoUser};
use pyo3::prelude::*;

import_pytype!(ast.Module);
import_pytype!(ast.Expression);

pub fn parse(py: Python<'_>, input: &str) -> PyResult<Module> {
    let ast = py.import("ast")?;
    let parse = ast.getattr("parse")?;
    let parsed = parse.call1((input,))?;
    parsed.extract()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        Python::with_gil(|py| {
            let m = parse(py, "def foo(): pass").unwrap();
            dbg!(m);
        });
    }
}
