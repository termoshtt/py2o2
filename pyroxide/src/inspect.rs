use anyhow::Result;
use pyo3::{types::PyModule, PyResult, Python};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Type {
    Primitive(Primitive),
    Tuple(Vec<Type>),
    List(Vec<Type>),
    None,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Primitive {
    Int,
    Float,
    Str,
}

#[derive(Debug, Clone, PartialEq)]
struct Function {
    name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Interface {
    functions: HashMap<String, Function>,
}

pub fn inspect(target: &str) -> Result<Interface> {
    const PY: &str = include_str!("../../inspect_module.py");
    let _json_str = Python::with_gil(|py: Python<'_>| -> PyResult<String> {
        let module = PyModule::from_code(py, PY, "", "")?;
        let f = module.getattr("inspect_module")?;
        let json_str = f.call1((target,))?.extract()?;
        Ok(json_str)
    })?;

    todo!()
}
