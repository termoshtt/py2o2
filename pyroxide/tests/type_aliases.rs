use anyhow::Result;
use pyroxide::{codegen, wit};

const PYTHON_ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../python/");

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

#[test]
fn wit2rust() -> Result<()> {
    std::env::set_var("PYTHONPATH", PYTHON_ROOT);
    let (_, path) = wit::witgen("type_aliases").unwrap();
    let wit = wit::parse(&path)?;
    insta::assert_snapshot!(codegen::generate_from_wit(&wit, false)?, @r###"
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

    insta::assert_snapshot!(codegen::generate_from_wit(&wit, true)?, @r###"
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
    "###);

    Ok(())
}
