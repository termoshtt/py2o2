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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Parameter {
    name: String,
    type_: Type,
}

#[derive(Debug, Clone, PartialEq)]
struct Function {
    name: String,
    parameters: Vec<Parameter>,
    return_: Vec<Parameter>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Interface {
    functions: HashMap<String, Function>,
}

pub fn inspect(target: &str) -> Result<String> {
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
    fn example() -> Result<()> {
        std::env::set_var("PYTHONPATH", PYTHON_ROOT);
        let json = inspect("example")?;
        insta::assert_snapshot!(json, @r###"
        {
            "functions": {
                "a1": {
                    "name": "a1",
                    "parameters": [],
                    "return": {
                        "kind": null
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
                        "kind": null
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
                        "kind": null
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
        Ok(())
    }

    #[test]
    fn type_aliases() -> Result<()> {
        std::env::set_var("PYTHONPATH", PYTHON_ROOT);
        let json = inspect("type_aliases")?;
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
        Ok(())
    }
}
