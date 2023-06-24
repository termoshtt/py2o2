use anyhow::Result;
use py2o2::inspect::*;

const PYTHON_ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../python/");

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
