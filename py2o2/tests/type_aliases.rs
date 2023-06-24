use anyhow::Result;
use py2o2::inspect::*;

const PYTHON_ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../python/");

#[test]
fn type_aliases() -> Result<()> {
    std::env::set_var("PYTHONPATH", PYTHON_ROOT);
    let json = get_inspect_json("type_aliases")?;
    insta::assert_snapshot!(json, @r###"
    {
        "functions": {
            "broadcast_message": {
                "name": "broadcast_message",
                "parameters": [
                    {
                        "name": "message",
                        "type": {
                            "kind": "primitive",
                            "name": "str"
                        }
                    },
                    {
                        "name": "servers",
                        "type": {
                            "kind": "list",
                            "inner": [
                                {
                                    "kind": "tuple",
                                    "tags": [
                                        {
                                            "kind": "tuple",
                                            "tags": [
                                                {
                                                    "kind": "primitive",
                                                    "name": "str"
                                                },
                                                {
                                                    "kind": "primitive",
                                                    "name": "int"
                                                }
                                            ]
                                        },
                                        {
                                            "kind": "dict",
                                            "inner": [
                                                {
                                                    "kind": "primitive",
                                                    "name": "str"
                                                },
                                                {
                                                    "kind": "primitive",
                                                    "name": "str"
                                                }
                                            ]
                                        }
                                    ]
                                }
                            ]
                        }
                    }
                ],
                "return": {
                    "kind": "none"
                }
            },
            "get_user_name": {
                "name": "get_user_name",
                "parameters": [
                    {
                        "name": "user_id",
                        "type": {
                            "kind": "user_defined",
                            "module": "type_aliases",
                            "name": "UserId",
                            "supertype": {
                                "kind": "primitive",
                                "name": "int"
                            }
                        }
                    }
                ],
                "return": {
                    "kind": "primitive",
                    "name": "str"
                }
            },
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
        },
        "type_definitions": {
            "UserId": {
                "module": "type_aliases",
                "name": "UserId",
                "supertype": {
                    "kind": "primitive",
                    "name": "int"
                }
            }
        },
        "unions": {}
    }
    "###);

    let interface = Interface::from_json(&json)?;
    insta::assert_debug_snapshot!(interface, @r###"
    Interface {
        functions: {
            "broadcast_message": Function {
                name: "broadcast_message",
                parameters: [
                    Parameter {
                        name: "message",
                        type: Primitive(
                            Str,
                        ),
                    },
                    Parameter {
                        name: "servers",
                        type: List {
                            inner: [
                                Tuple {
                                    tags: [
                                        Tuple {
                                            tags: [
                                                Primitive(
                                                    Str,
                                                ),
                                                Primitive(
                                                    Int,
                                                ),
                                            ],
                                        },
                                        Dict {
                                            inner: [
                                                Primitive(
                                                    Str,
                                                ),
                                                Primitive(
                                                    Str,
                                                ),
                                            ],
                                        },
                                    ],
                                },
                            ],
                        },
                    },
                ],
                return: None,
            },
            "get_user_name": Function {
                name: "get_user_name",
                parameters: [
                    Parameter {
                        name: "user_id",
                        type: UserDefined {
                            module: "type_aliases",
                            name: "UserId",
                            supertype: Primitive(
                                Int,
                            ),
                        },
                    },
                ],
                return: Primitive(
                    Str,
                ),
            },
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
        type_definitions: {
            "UserId": TypeDefinition {
                name: "UserId",
                module: "type_aliases",
                supertype: Primitive(
                    Int,
                ),
            },
        },
        unions: {},
    }
    "###);

    Ok(())
}
