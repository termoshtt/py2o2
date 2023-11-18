use py2o2_runtime::{import_pytype, PyTypeInfoUser};
use pyo3::{prelude::*, types::*};

import_pytype!(ast.Module);

impl Module {
    pub fn body<'py>(&'py self, py: Python<'py>) -> PyResult<&'py PyList> {
        self.0.as_ref(py).getattr("body")?.extract()
    }
}

import_pytype!(ast.Expression);

import_pytype!(ast.arguments);

import_pytype!(ast.FunctionDef);

impl FunctionDef {
    pub fn name<'py>(&'py self, py: Python<'py>) -> PyResult<&'py str> {
        self.0.as_ref(py).getattr("name")?.extract()
    }

    pub fn args<'py>(&'py self, py: Python<'py>) -> PyResult<arguments> {
        self.0.as_ref(py).getattr("args")?.extract()
    }

    pub fn returns<'py>(&'py self, py: Python<'py>) -> PyResult<&'py PyAny> {
        self.0.as_ref(py).getattr("returns")
    }
}

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
        Python::with_gil(|py| -> PyResult<()> {
            let m = parse(py, "def foo(): pass")?;
            let body = m.body(py)?;
            let elem = body.get_item(0)?;
            let foo = elem.extract::<FunctionDef>()?;
            dbg!(foo.name(py)?, foo.args(py)?, foo.returns(py)?);
            Ok(())
        })
        .unwrap();
    }
}
