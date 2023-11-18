use py2o2_runtime::{import_pytype, PyTypeInfoUser};
use pyo3::{exceptions::*, prelude::*, types::*};

import_pytype!(ast.Module);

impl<'py> Module<'py> {
    pub fn body(&self) -> PyResult<Vec<Statements<'py>>> {
        let statments: &PyList = self.0.getattr("body")?.extract()?;
        statments
            .iter()
            .map(|st| st.extract())
            .collect::<PyResult<_>>()
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

import_pytype!(ast.ImportFrom);
import_pytype!(ast.Assign);

#[derive(Debug)]
pub enum Statements<'py> {
    ImportFrom(ImportFrom<'py>),
    FunctionDef(FunctionDef<'py>),
    Assign(Assign<'py>),
}

impl<'py> FromPyObject<'py> for Statements<'py> {
    fn extract(ob: &'py PyAny) -> PyResult<Self> {
        if let Ok(import_from) = ob.extract() {
            Ok(Statements::ImportFrom(import_from))
        } else if let Ok(function_def) = ob.extract() {
            Ok(Statements::FunctionDef(function_def))
        } else if let Ok(assign) = ob.extract() {
            Ok(Statements::Assign(assign))
        } else {
            Err(PyTypeError::new_err(format!(
                "Expected a statement, {}",
                ob.get_type(),
            )))
        }
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
            if let Statements::FunctionDef(foo) = &body[0] {
                assert_eq!(foo.name()?, "foo");
            } else {
                panic!()
            }
            Ok(())
        })
        .unwrap();
    }

    #[test]
    fn test_parse_callable() {
        Python::with_gil(|py| -> PyResult<()> {
            let m = parse(py, include_str!("../../python/callable.py"))?;
            let body = m.body()?;
            dbg!(body);
            Ok(())
        })
        .unwrap();
    }

    #[test]
    fn test_parse_example() {
        Python::with_gil(|py| -> PyResult<()> {
            let m = parse(py, include_str!("../../python/example.py"))?;
            let body = m.body()?;
            dbg!(body);
            Ok(())
        })
        .unwrap();
    }

    #[test]
    fn test_parse_type_aliases() {
        Python::with_gil(|py| -> PyResult<()> {
            let m = parse(py, include_str!("../../python/type_aliases.py"))?;
            let body = m.body()?;
            dbg!(body);
            Ok(())
        })
        .unwrap();
    }

    #[test]
    fn test_parse_union() {
        Python::with_gil(|py| -> PyResult<()> {
            let m = parse(py, include_str!("../../python/union.py"))?;
            let body = m.body()?;
            dbg!(body);
            Ok(())
        })
        .unwrap();
    }
}
