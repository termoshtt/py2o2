use anyhow::Result;
use std::{fs, path::Path};

const PYTHON_ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../python/");

fn main() -> Result<()>{
    std::env::set_var("PYTHONPATH", PYTHON_ROOT);
    let testing_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../pyroxide-testing/src");
    for module in ["example", "type_aliases"] {
        let code = pyroxide::generate(module, true)?;
        fs::write(&testing_root.join(format!("{}.rs", module)), code)?;
    }
    Ok(())
}
