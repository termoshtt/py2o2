use anyhow::Result;
use py2o2::{codegen::*, inspect::*};

const PYTHON_ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../python/");
const TARGET: &str = "example";

#[test]
fn inspect() -> Result<()> {
    std::env::set_var("PYTHONPATH", PYTHON_ROOT);
    let json = get_inspect_json(TARGET)?;
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
        },
        "type_definitions": {},
        "unions": {}
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
        type_definitions: {},
        unions: {},
    }
    "###);

    Ok(())
}

#[test]
fn codegen() -> Result<()> {
    std::env::set_var("PYTHONPATH", PYTHON_ROOT);
    let interface = Interface::from_py_module(TARGET)?;
    insta::assert_snapshot!(generate(TARGET, &interface, true)?, @r###"
    pub fn a1<'py>(py: ::pyo3::Python<'py>) -> ::pyo3::PyResult<()> {
        let _ = py.import("example")?.getattr("a1")?.call((), None)?;
        Ok(())
    }
    pub fn a2<'py>(py: ::pyo3::Python<'py>, x: i64) -> ::pyo3::PyResult<()> {
        let _ = py.import("example")?.getattr("a2")?.call((x,), None)?;
        Ok(())
    }
    pub fn a3<'py>(py: ::pyo3::Python<'py>, y: &str, z: f64) -> ::pyo3::PyResult<()> {
        let _ = py.import("example")?.getattr("a3")?.call((y, z), None)?;
        Ok(())
    }
    pub fn a4<'py>(py: ::pyo3::Python<'py>) -> ::pyo3::PyResult<i64> {
        let result = py.import("example")?.getattr("a4")?.call((), None)?;
        Ok(result.extract()?)
    }
    pub fn a5<'py>(
        py: ::pyo3::Python<'py>,
        x: i64,
    ) -> ::pyo3::PyResult<&'py ::pyo3::types::PyString> {
        let result = py.import("example")?.getattr("a5")?.call((x,), None)?;
        Ok(result.extract()?)
    }
    pub fn a6<'py>(
        py: ::pyo3::Python<'py>,
    ) -> ::pyo3::PyResult<(i64, &'py ::pyo3::types::PyString)> {
        let result = py.import("example")?.getattr("a6")?.call((), None)?;
        Ok(result.extract()?)
    }
    pub fn a7<'py>(
        py: ::pyo3::Python<'py>,
        x: i64,
    ) -> ::pyo3::PyResult<(i64, &'py ::pyo3::types::PyString, f64)> {
        let result = py.import("example")?.getattr("a7")?.call((x,), None)?;
        Ok(result.extract()?)
    }
    "###);

    insta::assert_snapshot!(generate(TARGET, &interface, false)?, @r###"
    pub mod example {
        pub fn a1<'py>(py: ::pyo3::Python<'py>) -> ::pyo3::PyResult<()> {
            let _ = py.import("example")?.getattr("a1")?.call((), None)?;
            Ok(())
        }
        pub fn a2<'py>(py: ::pyo3::Python<'py>, x: i64) -> ::pyo3::PyResult<()> {
            let _ = py.import("example")?.getattr("a2")?.call((x,), None)?;
            Ok(())
        }
        pub fn a3<'py>(py: ::pyo3::Python<'py>, y: &str, z: f64) -> ::pyo3::PyResult<()> {
            let _ = py.import("example")?.getattr("a3")?.call((y, z), None)?;
            Ok(())
        }
        pub fn a4<'py>(py: ::pyo3::Python<'py>) -> ::pyo3::PyResult<i64> {
            let result = py.import("example")?.getattr("a4")?.call((), None)?;
            Ok(result.extract()?)
        }
        pub fn a5<'py>(
            py: ::pyo3::Python<'py>,
            x: i64,
        ) -> ::pyo3::PyResult<&'py ::pyo3::types::PyString> {
            let result = py.import("example")?.getattr("a5")?.call((x,), None)?;
            Ok(result.extract()?)
        }
        pub fn a6<'py>(
            py: ::pyo3::Python<'py>,
        ) -> ::pyo3::PyResult<(i64, &'py ::pyo3::types::PyString)> {
            let result = py.import("example")?.getattr("a6")?.call((), None)?;
            Ok(result.extract()?)
        }
        pub fn a7<'py>(
            py: ::pyo3::Python<'py>,
            x: i64,
        ) -> ::pyo3::PyResult<(i64, &'py ::pyo3::types::PyString, f64)> {
            let result = py.import("example")?.getattr("a7")?.call((x,), None)?;
            Ok(result.extract()?)
        }
    }
    "###);
    Ok(())
}
