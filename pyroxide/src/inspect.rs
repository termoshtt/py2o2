use anyhow::Result;
use pyo3::{types::PyModule, PyResult, Python};
use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
#[serde(rename_all = "lowercase")]
#[serde(tag = "kind")]
pub enum Type {
    Primitive(Primitive),
    Tuple { tags: Vec<Type> },
    List { inner: Vec<Type> },
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
    name: String,
    r#type: Type,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
struct Function {
    name: String,
    parameters: Vec<Parameter>,
    r#return: Type,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Interface {
    functions: BTreeMap<String, Function>,
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

    const PYTHON_ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../python/");

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

    #[test]
    fn example() -> Result<()> {
        std::env::set_var("PYTHONPATH", PYTHON_ROOT);
        let json = get_inspect_json("example")?;
        insta::assert_snapshot!(json, @r###"
        {
            "functions": {
                "a1": {
                    "name": "a1",
                    "parameters": [],
                    "return": {
                        "kind": "none"
                    }
                },
                "a2": {
                    "name": "a2",
                    "parameters": [
                        {
                            "name": "x",
                            "type": {
                                "kind": "primitive",
                                "name": "int"
                            }
                        }
                    ],
                    "return": {
                        "kind": "none"
                    }
                },
                "a3": {
                    "name": "a3",
                    "parameters": [
                        {
                            "name": "y",
                            "type": {
                                "kind": "primitive",
                                "name": "str"
                            }
                        },
                        {
                            "name": "z",
                            "type": {
                                "kind": "primitive",
                                "name": "float"
                            }
                        }
                    ],
                    "return": {
                        "kind": "none"
                    }
                },
                "a4": {
                    "name": "a4",
                    "parameters": [],
                    "return": {
                        "kind": "primitive",
                        "name": "int"
                    }
                },
                "a5": {
                    "name": "a5",
                    "parameters": [
                        {
                            "name": "x",
                            "type": {
                                "kind": "primitive",
                                "name": "int"
                            }
                        }
                    ],
                    "return": {
                        "kind": "primitive",
                        "name": "str"
                    }
                },
                "a6": {
                    "name": "a6",
                    "parameters": [],
                    "return": {
                        "kind": "tuple",
                        "tags": [
                            {
                                "kind": "primitive",
                                "name": "int"
                            },
                            {
                                "kind": "primitive",
                                "name": "str"
                            }
                        ]
                    }
                },
                "a7": {
                    "name": "a7",
                    "parameters": [
                        {
                            "name": "x",
                            "type": {
                                "kind": "primitive",
                                "name": "int"
                            }
                        }
                    ],
                    "return": {
                        "kind": "tuple",
                        "tags": [
                            {
                                "kind": "primitive",
                                "name": "int"
                            },
                            {
                                "kind": "primitive",
                                "name": "str"
                            },
                            {
                                "kind": "primitive",
                                "name": "float"
                            }
                        ]
                    }
                }
            }
        }
        "###);

        let interface = Interface::from_json(&json)?;
        insta::assert_debug_snapshot!(interface, @r###"
        Interface {
            functions: {
                "a1": Function {
                    name: "a1",
                    parameters: [],
                    return: None,
                },
                "a2": Function {
                    name: "a2",
                    parameters: [
                        Parameter {
                            name: "x",
                            type: Primitive(
                                Int,
                            ),
                        },
                    ],
                    return: None,
                },
                "a3": Function {
                    name: "a3",
                    parameters: [
                        Parameter {
                            name: "y",
                            type: Primitive(
                                Str,
                            ),
                        },
                        Parameter {
                            name: "z",
                            type: Primitive(
                                Float,
                            ),
                        },
                    ],
                    return: None,
                },
                "a4": Function {
                    name: "a4",
                    parameters: [],
                    return: Primitive(
                        Int,
                    ),
                },
                "a5": Function {
                    name: "a5",
                    parameters: [
                        Parameter {
                            name: "x",
                            type: Primitive(
                                Int,
                            ),
                        },
                    ],
                    return: Primitive(
                        Str,
                    ),
                },
                "a6": Function {
                    name: "a6",
                    parameters: [],
                    return: Tuple {
                        tags: [
                            Primitive(
                                Int,
                            ),
                            Primitive(
                                Str,
                            ),
                        ],
                    },
                },
                "a7": Function {
                    name: "a7",
                    parameters: [
                        Parameter {
                            name: "x",
                            type: Primitive(
                                Int,
                            ),
                        },
                    ],
                    return: Tuple {
                        tags: [
                            Primitive(
                                Int,
                            ),
                            Primitive(
                                Str,
                            ),
                            Primitive(
                                Float,
                            ),
                        ],
                    },
                },
            },
        }
        "###);

        Ok(())
    }

    #[test]
    fn type_aliases() -> Result<()> {
        std::env::set_var("PYTHONPATH", PYTHON_ROOT);
        let json = get_inspect_json("type_aliases")?;
        insta::assert_snapshot!(json, @r###"
        {
            "functions": {
                "scale": {
                    "name": "scale",
                    "parameters": [
                        {
                            "name": "scalar",
                            "type": {
                                "kind": "primitive",
                                "name": "float"
                            }
                        },
                        {
                            "name": "vector",
                            "type": {
                                "kind": "list",
                                "inner": [
                                    {
                                        "kind": "primitive",
                                        "name": "float"
                                    }
                                ]
                            }
                        }
                    ],
                    "return": {
                        "kind": "list",
                        "inner": [
                            {
                                "kind": "primitive",
                                "name": "float"
                            }
                        ]
                    }
                }
            }
        }
        "###);

        let interface = Interface::from_json(&json)?;
        insta::assert_debug_snapshot!(interface, @r###"
        Interface {
            functions: {
                "scale": Function {
                    name: "scale",
                    parameters: [
                        Parameter {
                            name: "scalar",
                            type: Primitive(
                                Float,
                            ),
                        },
                        Parameter {
                            name: "vector",
                            type: List {
                                inner: [
                                    Primitive(
                                        Float,
                                    ),
                                ],
                            },
                        },
                    ],
                    return: List {
                        inner: [
                            Primitive(
                                Float,
                            ),
                        ],
                    },
                },
            },
        }
        "###);

        Ok(())
    }
}
