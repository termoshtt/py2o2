use anyhow::Result;
use pyroxide::{codegen, wit};
use std::path::Path;

const PYTHON_ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../python/");

#[test]
fn wit2rust() -> Result<()> {
    let wit = wit::parse(&Path::new(PYTHON_ROOT).join("type-aliases.wit"))?;
    let tt = codegen::generate_from_wit(wit).unwrap();
    insta::assert_snapshot!(tt, @r###"
    pub mod type_aliases {
        pub fn scale<'py>(
            py: ::pyo3::Python<'py>,
            scalar: f64,
            vector: &::pyo3::types::PyList,
        ) -> ::pyo3::PyResult<&'py ::pyo3::types::PyList> {
            let result = py
                .import("type_aliases")?
                .getattr("scale")?
                .call((scalar, vector), None)?;
            Ok(result.extract()?)
        }
    }
    "###);
    Ok(())
}

#[test]
fn py2wit() {
    std::env::set_var("PYTHONPATH", PYTHON_ROOT);
    let (wit, _path) = wit::witgen("type_aliases").unwrap();
    insta::assert_snapshot!(wit, @r###"
    interface type-aliases {
    scale: func(scalar: float64, vector: list<float64>) -> list<float64>
    }
    "###);
}
