use anyhow::Result;
use pyroxide::{codegen, wit};

const PYTHON_ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../python/");

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
    a6: func() -> tuple<s64, string>
    a7: func(x: s64) -> tuple<s64, string, float64>
    }
    "###);
}

#[test]
fn py2rust() -> Result<()> {
    std::env::set_var("PYTHONPATH", PYTHON_ROOT);
    let (_, path) = wit::witgen("example").unwrap();
    let wit = wit::parse(&path)?;
    insta::assert_snapshot!(codegen::generate_from_wit(&wit, false)?, @r###"
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

    insta::assert_snapshot!(codegen::generate_from_wit(&wit, true)?, @r###"
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

    Ok(())
}
