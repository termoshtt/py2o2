pub mod codegen;
pub mod wit;

use anyhow::Result;
use std::{
    fs,
    path::{Path, PathBuf},
};

/// Generate PyO3-based Rust binding
pub fn generate(
    python_module_name: &str,
    python_sys_path: Option<impl AsRef<Path>>,
) -> Result<String> {
    let (_wit, path) = wit::witgen(python_module_name, python_sys_path)?;
    let interfaces = wit::parse(&path)?;
    let generated = codegen::generate_from_wit(interfaces)?;
    Ok(generated)
}

/// Generate binding within `build.rs`
pub fn generate_on_out_dir(
    python_module_name: &str,
    python_sys_path: Option<impl AsRef<Path>>,
) -> Result<PathBuf> {
    let generated = generate(python_module_name, python_sys_path)?;
    let path = Path::new(&std::env::var("OUT_DIR")?).join(format!("{}.rs", python_module_name));
    fs::write(&path, generated)?;
    Ok(path)
}

#[macro_export]
macro_rules! import {
    ($target:ident) => {
        include!(concat!(env!("OUT_DIR"), "/", stringify!($target), ".rs"));
    };
}
