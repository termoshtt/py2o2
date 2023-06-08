use anyhow::{Context, Result};
use pyo3::{prelude::*, types::PyString};
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

/// Parse WIT file, and resolve its definitions
pub fn parse(path: impl AsRef<Path>) -> Result<wit_parser::Resolve> {
    let unresolved = wit_parser::UnresolvedPackage::parse_file(path.as_ref())?;
    let mut wit = wit_parser::Resolve::new();
    wit.push(unresolved, &HashMap::new())?;
    Ok(wit)
}

/// Create WIT from Python module
///
/// Since WIT must be accompanied with a path,
/// generated WIT string will be saved cache ditectory.
///
pub fn witgen(
    target: &str,
    python_sys_path: Option<impl AsRef<Path>>,
) -> Result<(String, PathBuf)> {
    const WITGEN_PY: &str = include_str!("../../witgen.py");
    if let Some(path) = python_sys_path {
        std::env::set_var("PYTHONPATH", path.as_ref());
    }
    let wit = Python::with_gil(|py| -> PyResult<String> {
        let m = PyModule::from_code(py, WITGEN_PY, "", "")?;
        let f = m.getattr("witgen")?;
        let wit: &PyString = f.call1((target,))?.extract()?;
        Ok(wit.to_string())
    })?;
    let out_dir = dirs::cache_dir()
        .context("Cannot create cache directory")?
        .join("pyroxide");
    fs::create_dir_all(&out_dir)?;
    let path = Path::new(&out_dir).join(format!("{}.wit", target.replace('_', "-")));
    fs::write(&path, &wit)?;
    Ok((wit, path))
}
