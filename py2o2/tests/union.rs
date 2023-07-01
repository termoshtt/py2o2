use anyhow::Result;
use py2o2::{codegen::*, inspect::*};

const PYTHON_ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../python/");
const TARGET: &str = "union";

#[test]
fn inspect() -> Result<()> {
    std::env::set_var("PYTHONPATH", PYTHON_ROOT);
    let json = get_inspect_json(TARGET)?;
    insta::assert_snapshot!(json, @r###"
    {
        "functions": {
            "f_new": {
                "name": "f_new",
                "parameters": [
                    {
                        "name": "a",
                        "type": {
                            "kind": "union",
                            "args": [
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
                    }
                ],
                "return": {
                    "kind": "union",
                    "args": [
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
            "f_old": {
                "name": "f_old",
                "parameters": [
                    {
                        "name": "a",
                        "type": {
                            "kind": "union",
                            "args": [
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
                    }
                ],
                "return": {
                    "kind": "union",
                    "args": [
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
            }
        },
        "type_definitions": {}
    }
    "###);

    let interface = Interface::from_json(&json)?;
    insta::assert_debug_snapshot!(interface, @r###"
    Interface {
        functions: {
            "f_new": Function {
                name: "f_new",
                parameters: [
                    Parameter {
                        name: "a",
                        type: Union {
                            args: [
                                Primitive(
                                    Int,
                                ),
                                Primitive(
                                    Str,
                                ),
                            ],
                        },
                    },
                ],
                return: Union {
                    args: [
                        Primitive(
                            Int,
                        ),
                        Primitive(
                            Str,
                        ),
                    ],
                },
            },
            "f_old": Function {
                name: "f_old",
                parameters: [
                    Parameter {
                        name: "a",
                        type: Union {
                            args: [
                                Primitive(
                                    Int,
                                ),
                                Primitive(
                                    Str,
                                ),
                            ],
                        },
                    },
                ],
                return: Union {
                    args: [
                        Primitive(
                            Int,
                        ),
                        Primitive(
                            Str,
                        ),
                    ],
                },
            },
        },
        type_definitions: {},
    }
    "###);

    Ok(())
}

#[test]
fn codegen() -> Result<()> {
    std::env::set_var("PYTHONPATH", PYTHON_ROOT);
    let interface = Interface::from_py_module(TARGET)?;
    insta::assert_snapshot!(generate(TARGET, &interface, true)?, @r###"
    pub trait Union5d6b010906f780ce: ::pyo3::conversion::IntoPy<::pyo3::PyObject> {}
    impl Union5d6b010906f780ce for i64 {}
    impl Union5d6b010906f780ce for &str {}
    pub fn f_new<'py>(
        py: ::pyo3::Python<'py>,
        a: impl Union5d6b010906f780ce,
    ) -> ::pyo3::PyResult<::py2o2_runtime::Enum2<i64, ::pyo3::Py<::pyo3::types::PyString>>> {
        let result = py.import("union")?.getattr("f_new")?.call((a,), None)?;
        Ok(result.extract()?)
    }
    pub fn f_old<'py>(
        py: ::pyo3::Python<'py>,
        a: impl Union5d6b010906f780ce,
    ) -> ::pyo3::PyResult<::py2o2_runtime::Enum2<i64, ::pyo3::Py<::pyo3::types::PyString>>> {
        let result = py.import("union")?.getattr("f_old")?.call((a,), None)?;
        Ok(result.extract()?)
    }
    "###);

    insta::assert_snapshot!(generate(TARGET, &interface, false)?, @r###"
    pub mod union {
        pub trait Union5d6b010906f780ce: ::pyo3::conversion::IntoPy<::pyo3::PyObject> {}
        impl Union5d6b010906f780ce for i64 {}
        impl Union5d6b010906f780ce for &str {}
        pub fn f_new<'py>(
            py: ::pyo3::Python<'py>,
            a: impl Union5d6b010906f780ce,
        ) -> ::pyo3::PyResult<
            ::py2o2_runtime::Enum2<i64, ::pyo3::Py<::pyo3::types::PyString>>,
        > {
            let result = py.import("union")?.getattr("f_new")?.call((a,), None)?;
            Ok(result.extract()?)
        }
        pub fn f_old<'py>(
            py: ::pyo3::Python<'py>,
            a: impl Union5d6b010906f780ce,
        ) -> ::pyo3::PyResult<
            ::py2o2_runtime::Enum2<i64, ::pyo3::Py<::pyo3::types::PyString>>,
        > {
            let result = py.import("union")?.getattr("f_old")?.call((a,), None)?;
            Ok(result.extract()?)
        }
    }
    "###);
    Ok(())
}
