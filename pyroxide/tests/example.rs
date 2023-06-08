use anyhow::Result;
use pyroxide::{codegen, wit};
use std::path::Path;

const PYTHON_ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../python/");

#[test]
fn wit2rust() -> Result<()> {
    let wit = wit::parse(&Path::new(PYTHON_ROOT).join("example.wit"))?;
    let tt = codegen::generate_from_wit(wit)?;
    insta::assert_snapshot!(tt, @r###"
        pub mod example {
            use pyo3::{prelude::*, types::PyString};
            pub fn a1<'py>(py: Python<'py>) -> PyResult<()> {
                let _ = py.import("example")?.getattr("a1")?.call((), None)?;
                Ok(())
            }
            pub fn a2<'py>(py: Python<'py>, x: i64) -> PyResult<()> {
                let _ = py.import("example")?.getattr("a2")?.call((x,), None)?;
                Ok(())
            }
            pub fn a3<'py>(py: Python<'py>, y: &str, z: f64) -> PyResult<()> {
                let _ = py.import("example")?.getattr("a3")?.call((y, z), None)?;
                Ok(())
            }
            pub fn a4<'py>(py: Python<'py>) -> PyResult<i64> {
                let result = py.import("example")?.getattr("a4")?.call((), None)?;
                Ok(result.extract()?)
            }
            pub fn a5<'py>(py: Python<'py>, x: i64) -> PyResult<&'py PyString> {
                let result = py.import("example")?.getattr("a5")?.call((x,), None)?;
                Ok(result.extract()?)
            }
            pub fn a6<'py>(py: Python<'py>) -> PyResult<(i64, &'py PyString)> {
                let result = py.import("example")?.getattr("a6")?.call((), None)?;
                Ok(result.extract()?)
            }
            pub fn a7<'py>(py: Python<'py>, x: i64) -> PyResult<(i64, &'py PyString, f64)> {
                let result = py.import("example")?.getattr("a7")?.call((x,), None)?;
                Ok(result.extract()?)
            }
        }
        "###);
    Ok(())
}

#[test]
fn py2wit() {
    std::env::set_var("PYTHONPATH", PYTHON_ROOT);
    let (wit, _path) = wit::witgen("example").unwrap();
    insta::assert_snapshot!(wit, @r###"
    interface example {
    a1: func() 
    a2: func(x: s64) 
    a3: func(y: string, z: float64) 
    a4: func() -> s64
    a5: func(x: s64) -> string
    a6: func() -> (out0: s64, out1: string)
    a7: func(x: s64) -> (out0: s64, out1: string, out2: float64)
    }
    "###);
}