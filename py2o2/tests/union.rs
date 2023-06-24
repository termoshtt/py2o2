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
        "functions": {},
        "type_definitions": {},
        "unions": {
            "union_new": {
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
            },
            "union_old": {
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
    }
    "###);

    let interface = Interface::from_json(&json)?;
    insta::assert_debug_snapshot!(interface, @r###"
    Interface {
        functions: {},
        type_definitions: {},
        unions: {
            "union_new": Union {
                args: [
                    Primitive(
                        Int,
                    ),
                    Primitive(
                        Str,
                    ),
                ],
            },
            "union_old": Union {
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
    }
    "###);

    Ok(())
}

#[test]
fn codegen() -> Result<()> {
    std::env::set_var("PYTHONPATH", PYTHON_ROOT);
    let interface = Interface::from_py_module(TARGET)?;
    insta::assert_snapshot!(generate(TARGET, &interface, true)?, @"");

    insta::assert_snapshot!(generate(TARGET, &interface, false)?, @r###"
    pub mod union {}
    "###);
    Ok(())
}
