use anyhow::Result;
use pyo3::{prelude::*, types::PyString};
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

pub fn save_wit(target: &str) -> Result<PathBuf> {
    let wit = witgen(target)?;
    let out_dir = dirs::cache_dir().unwrap().join("pyroxide");
    fs::create_dir_all(&out_dir)?;
    let path = Path::new(&out_dir).join(format!("{}.wit", target));
    fs::write(&path, wit)?;
    Ok(path)
}

pub fn get_interfaces(path: &Path) -> Result<Vec<wit_parser::Interface>> {
    let unresolved = wit_parser::UnresolvedPackage::parse_file(path)?;
    let mut wit = wit_parser::Resolve::new();
    wit.push(unresolved, &HashMap::new())?;
    Ok(wit
        .interfaces
        .into_iter()
        .map(|(_id, contents)| contents)
        .collect())
}

pub fn witgen(target: &str) -> Result<String> {
    const WITGEN_PY: &str = include_str!("../../witgen.py");
    Ok(Python::with_gil(|py| -> PyResult<String> {
        let m = PyModule::from_code(py, WITGEN_PY, "", "")?;
        let f = m.getattr("witgen")?;
        let wit: &PyString = f.call1((target,))?.extract()?;
        Ok(wit.to_string())
    })?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wit_parser() {
        let test_wit = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../test.wit");
        let interfaces = get_interfaces(&test_wit).unwrap();
        dbg!(interfaces);
    }

    #[test]
    fn test_witgen_example() {
        // Add a path where `example.py` exists
        let project_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap();
        std::env::set_var("PYTHONPATH", project_root);

        let wit = witgen("example").unwrap();
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
}
