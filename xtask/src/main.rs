use anyhow::Result;
use std::{fs, path::Path, process::Command};

const PYTHON_ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../python/");

fn main() -> Result<()> {
    std::env::set_var("PYTHONPATH", PYTHON_ROOT);
    let testing_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../py2o2-testing/src");
    for module in ["example", "type_aliases"] {
        let code = py2o2::generate(module, true)?;
        fs::write(&testing_root.join(format!("{}.rs", module)), code)?;
    }

    let st = Command::new("cargo").arg("fmt").arg("--all").status()?;
    assert!(st.success());
    Ok(())
}
