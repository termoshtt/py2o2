use std::{fs, path::Path};

const PYTHON_ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../python/");

fn main() {
    std::env::set_var("PYTHONPATH", PYTHON_ROOT);
    let generated = pyroxide::generate("example").unwrap();
    let out_dir = std::env::var("OUT_DIR").unwrap();
    fs::write(Path::new(&out_dir).join("example.rs"), generated).unwrap();
}
