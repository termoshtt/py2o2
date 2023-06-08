use anyhow::Result;

const PYTHON_ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../python/");

fn main() -> Result<()> {
    for m in ["example", "type_aliases"] {
        pyroxide::generate_on_out_dir(m, Some(PYTHON_ROOT))?;
    }
    Ok(())
}
