use pyroxide::{codegen, wit};
use std::path::Path;

#[test]
fn wit_to_rust() {
    let interfaces =
        wit::parse(&Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/example.wit")).unwrap();
    let tt = codegen::generate_from_wit(&interfaces).unwrap();
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
}

#[test]
fn py_to_wit() {
    let project_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    std::env::set_var("PYTHONPATH", project_root.join("tests"));
    let (wit, _path) = wit::witgen("example").unwrap();
    assert_eq!(wit, include_str!("example.wit").trim());
}
