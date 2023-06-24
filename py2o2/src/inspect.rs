use anyhow::Result;
use pyo3::{types::PyModule, PyResult, Python};
use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "kind")]
pub enum Type {
    Primitive(Primitive),
    Tuple {
        tags: Vec<Type>,
    },
    List {
        inner: Vec<Type>,
    },
    Dict {
        inner: Vec<Type>,
    },
    UserDefined {
        module: String,
        name: String,
        supertype: Box<Type>,
    },
    Union {
        args: Vec<Type>,
    },
    None,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
#[serde(rename_all = "lowercase")]
#[serde(tag = "name")]
pub enum Primitive {
    Int,
    Float,
    Str,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
pub struct Parameter {
    pub name: String,
    pub r#type: Type,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
pub struct Function {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub r#return: Type,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
pub struct TypeDefinition {
    pub name: String,
    pub module: String,
    pub supertype: Type,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Interface {
    pub functions: BTreeMap<String, Function>,
    pub type_definitions: BTreeMap<String, TypeDefinition>,
}

impl Interface {
    pub fn from_json(json: &str) -> Result<Self> {
        Ok(serde_json::from_str(json)?)
    }

    pub fn from_py_module(target: &str) -> Result<Self> {
        let json = get_inspect_json(target)?;
        Self::from_json(&json)
    }
}

pub fn get_inspect_json(target: &str) -> Result<String> {
    const PY: &str = include_str!("../../inspect_module.py");
    let json = Python::with_gil(|py: Python<'_>| -> PyResult<String> {
        let module = PyModule::from_code(py, PY, "", "")?;
        let f = module.getattr("inspect_module")?;
        let json = f.call1((target,))?.extract()?;
        Ok(json)
    })?;
    Ok(json)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn deserialize_type() -> Result<()> {
        let ty: Type = serde_json::from_str(r#"{"kind": "none"}"#)?;
        assert_eq!(ty, Type::None);

        let ty: Type = serde_json::from_str(r#"{"kind": "primitive", "name": "int"}"#)?;
        assert_eq!(ty, Type::Primitive(Primitive::Int));

        let ty: Type = serde_json::from_str(
            r#"{"kind": "list", "inner": [{"kind": "primitive", "name": "int"}]}"#,
        )?;
        assert_eq!(
            ty,
            Type::List {
                inner: vec![Type::Primitive(Primitive::Int)]
            }
        );
        Ok(())
    }

    #[test]
    fn deserialize_parameter() -> Result<()> {
        let p: Parameter =
            serde_json::from_str(r#"{"name": "x", "type": {"kind": "primitive", "name": "int"}}"#)?;
        assert_eq!(
            p,
            Parameter {
                name: "x".to_string(),
                r#type: Type::Primitive(Primitive::Int)
            }
        );
        Ok(())
    }
}
