use anyhow::Result;
use std::{fs, path::Path};

const PYTHON_ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../python/");

fn main() -> Result<()> {
    std::env::set_var("PYTHONPATH", PYTHON_ROOT);
    for m in ["example"] {
        let generated = pyroxide::generate(m)?;
        let out_dir = std::env::var("OUT_DIR")?;
        fs::write(Path::new(&out_dir).join(format!("{}.rs", m)), generated)?;
    }
    Ok(())
}
