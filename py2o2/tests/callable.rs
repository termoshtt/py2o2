use anyhow::Result;
use py2o2::{codegen::*, inspect::*};

const PYTHON_ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../python/");
const TARGET: &str = "callable";

#[test]
fn inspect() -> Result<()> {
    std::env::set_var("PYTHONPATH", PYTHON_ROOT);
    let json = get_inspect_json(TARGET)?;
    insta::assert_snapshot!(json, @r###"
    {
        "functions": {
            "async_query": {
                "name": "async_query",
                "parameters": [
                    {
                        "name": "on_success",
                        "type": {
                            "kind": "callable",
                            "args": [
                                {
                                    "kind": "primitive",
                                    "name": "int"
                                }
                            ],
                            "return": {
                                "kind": "none"
                            }
                        }
                    },
                    {
                        "name": "on_error",
                        "type": {
                            "kind": "callable",
                            "args": [
                                {
                                    "kind": "primitive",
                                    "name": "int"
                                },
                                {
                                    "kind": "exception"
                                }
                            ],
                            "return": {
                                "kind": "none"
                            }
                        }
                    }
                ],
                "return": {
                    "kind": "none"
                }
            },
            "caller": {
                "name": "caller",
                "parameters": [
                    {
                        "name": "f",
                        "type": {
                            "kind": "callable",
                            "args": [
                                {
                                    "kind": "primitive",
                                    "name": "int"
                                },
                                {
                                    "kind": "primitive",
                                    "name": "float"
                                }
                            ],
                            "return": {
                                "kind": "primitive",
                                "name": "float"
                            }
                        }
                    }
                ],
                "return": {
                    "kind": "none"
                }
            },
            "ellipsis_callable": {
                "name": "ellipsis_callable",
                "parameters": [
                    {
                        "name": "f",
                        "type": {
                            "kind": "callable",
                            "args": [
                                {
                                    "kind": "ellipsis"
                                }
                            ],
                            "return": {
                                "kind": "none"
                            }
                        }
                    }
                ],
                "return": {
                    "kind": "none"
                }
            },
            "feeder": {
                "name": "feeder",
                "parameters": [
                    {
                        "name": "get_next_item",
                        "type": {
                            "kind": "callable",
                            "args": [],
                            "return": {
                                "kind": "primitive",
                                "name": "str"
                            }
                        }
                    }
                ],
                "return": {
                    "kind": "none"
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
            "async_query": Function {
                name: "async_query",
                parameters: [
                    Parameter {
                        name: "on_success",
                        type: Callable {
                            args: [
                                Primitive(
                                    Int,
                                ),
                            ],
                            return: None,
                        },
                    },
                    Parameter {
                        name: "on_error",
                        type: Callable {
                            args: [
                                Primitive(
                                    Int,
                                ),
                                Exception,
                            ],
                            return: None,
                        },
                    },
                ],
                return: None,
            },
            "caller": Function {
                name: "caller",
                parameters: [
                    Parameter {
                        name: "f",
                        type: Callable {
                            args: [
                                Primitive(
                                    Int,
                                ),
                                Primitive(
                                    Float,
                                ),
                            ],
                            return: Primitive(
                                Float,
                            ),
                        },
                    },
                ],
                return: None,
            },
            "ellipsis_callable": Function {
                name: "ellipsis_callable",
                parameters: [
                    Parameter {
                        name: "f",
                        type: Callable {
                            args: [
                                Ellipsis,
                            ],
                            return: None,
                        },
                    },
                ],
                return: None,
            },
            "feeder": Function {
                name: "feeder",
                parameters: [
                    Parameter {
                        name: "get_next_item",
                        type: Callable {
                            args: [],
                            return: Primitive(
                                Str,
                            ),
                        },
                    },
                ],
                return: None,
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
    pub fn async_query<'py>(
        py: ::pyo3::Python<'py>,
        on_success: impl Fn((i64,)) -> () + Send + 'static,
        on_error: impl Fn((i64, ::pyo3::Py<::pyo3::PyAny>)) -> () + Send + 'static,
    ) -> ::pyo3::PyResult<()> {
        let on_success = ::py2o2_runtime::as_pycfunc(py, on_success)?;
        let on_error = ::py2o2_runtime::as_pycfunc(py, on_error)?;
        let _ = py
            .import("callable")?
            .getattr("async_query")?
            .call((on_success, on_error), None)?;
        Ok(())
    }
    pub fn caller<'py>(
        py: ::pyo3::Python<'py>,
        f: impl Fn((i64, f64)) -> f64 + Send + 'static,
    ) -> ::pyo3::PyResult<()> {
        let f = ::py2o2_runtime::as_pycfunc(py, f)?;
        let _ = py.import("callable")?.getattr("caller")?.call((f,), None)?;
        Ok(())
    }
    pub fn ellipsis_callable<'py>(
        py: ::pyo3::Python<'py>,
        f: impl Fn((::pyo3::Py<::pyo3::PyAny>,)) -> () + Send + 'static,
    ) -> ::pyo3::PyResult<()> {
        let f = ::py2o2_runtime::as_pycfunc(py, f)?;
        let _ = py.import("callable")?.getattr("ellipsis_callable")?.call((f,), None)?;
        Ok(())
    }
    pub fn feeder<'py>(
        py: ::pyo3::Python<'py>,
        get_next_item: impl Fn() -> ::pyo3::Py<::pyo3::types::PyString> + Send + 'static,
    ) -> ::pyo3::PyResult<()> {
        let get_next_item = ::py2o2_runtime::as_pycfunc(
            py,
            move |_input: [usize; 0]| get_next_item(),
        )?;
        let _ = py.import("callable")?.getattr("feeder")?.call((get_next_item,), None)?;
        Ok(())
    }
    "###);

    insta::assert_snapshot!(generate(TARGET, &interface, false)?, @r###"
    pub mod callable {
        pub fn async_query<'py>(
            py: ::pyo3::Python<'py>,
            on_success: impl Fn((i64,)) -> () + Send + 'static,
            on_error: impl Fn((i64, ::pyo3::Py<::pyo3::PyAny>)) -> () + Send + 'static,
        ) -> ::pyo3::PyResult<()> {
            let on_success = ::py2o2_runtime::as_pycfunc(py, on_success)?;
            let on_error = ::py2o2_runtime::as_pycfunc(py, on_error)?;
            let _ = py
                .import("callable")?
                .getattr("async_query")?
                .call((on_success, on_error), None)?;
            Ok(())
        }
        pub fn caller<'py>(
            py: ::pyo3::Python<'py>,
            f: impl Fn((i64, f64)) -> f64 + Send + 'static,
        ) -> ::pyo3::PyResult<()> {
            let f = ::py2o2_runtime::as_pycfunc(py, f)?;
            let _ = py.import("callable")?.getattr("caller")?.call((f,), None)?;
            Ok(())
        }
        pub fn ellipsis_callable<'py>(
            py: ::pyo3::Python<'py>,
            f: impl Fn((::pyo3::Py<::pyo3::PyAny>,)) -> () + Send + 'static,
        ) -> ::pyo3::PyResult<()> {
            let f = ::py2o2_runtime::as_pycfunc(py, f)?;
            let _ = py.import("callable")?.getattr("ellipsis_callable")?.call((f,), None)?;
            Ok(())
        }
        pub fn feeder<'py>(
            py: ::pyo3::Python<'py>,
            get_next_item: impl Fn() -> ::pyo3::Py<::pyo3::types::PyString> + Send + 'static,
        ) -> ::pyo3::PyResult<()> {
            let get_next_item = ::py2o2_runtime::as_pycfunc(
                py,
                move |_input: [usize; 0]| get_next_item(),
            )?;
            let _ = py.import("callable")?.getattr("feeder")?.call((get_next_item,), None)?;
            Ok(())
        }
    }
    "###);
    Ok(())
}
