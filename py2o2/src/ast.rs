use py2o2_runtime::{import_pytype, PyTypeInfoUser};
use pyo3::{prelude::*, types::*};

import_pytype!(ast.Module);

impl<'py> Module<'py> {
    pub fn body(&self) -> PyResult<&'py PyList> {
        self.0.getattr("body")?.extract()
    }
}

import_pytype!(ast.Expression);

import_pytype!(ast.arguments as Arguments);

import_pytype!(ast.FunctionDef);

impl<'py> FunctionDef<'py> {
    pub fn name(&self) -> PyResult<&'py str> {
        self.0.getattr("name")?.extract()
    }

    pub fn args(&self) -> PyResult<Arguments> {
        self.0.getattr("args")?.extract()
    }

    pub fn returns(&self) -> PyResult<&'py PyAny> {
        self.0.getattr("returns")
    }
}

pub fn parse<'py>(py: Python<'py>, input: &str) -> PyResult<Module<'py>> {
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
            let body = m.body()?;
            let elem = body.get_item(0)?;
            let foo = elem.extract::<FunctionDef>()?;
            dbg!(foo.name()?, foo.args()?, foo.returns()?);
            Ok(())
        })
        .unwrap();
    }
}
