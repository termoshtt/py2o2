use anyhow::Result;
use pyo3::{prelude::*, types::PyString};
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

pub fn parse(path: &Path) -> Result<Vec<wit_parser::Interface>> {
    let unresolved = wit_parser::UnresolvedPackage::parse_file(path)?;
    let mut wit = wit_parser::Resolve::new();
    wit.push(unresolved, &HashMap::new())?;
    Ok(wit
        .interfaces
        .into_iter()
        .map(|(_id, contents)| contents)
        .collect())
}

pub fn witgen(target: &str) -> Result<(String, PathBuf)> {
    const WITGEN_PY: &str = include_str!("../../witgen.py");
    let wit = Python::with_gil(|py| -> PyResult<String> {
        let m = PyModule::from_code(py, WITGEN_PY, "", "")?;
        let f = m.getattr("witgen")?;
        let wit: &PyString = f.call1((target,))?.extract()?;
        Ok(wit.to_string())
    })?;

    let out_dir = dirs::cache_dir().unwrap().join("pyroxide");
    fs::create_dir_all(&out_dir)?;
    let path = Path::new(&out_dir).join(format!("{}.wit", target));
    fs::write(&path, &wit)?;
    Ok((wit, path))
}
