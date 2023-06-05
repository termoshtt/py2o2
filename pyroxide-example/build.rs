use std::{fs, path::Path};

fn main() {
    // Add a path where `example.py` exists
    std::env::set_var("PYTHONPATH", env!("CARGO_MANIFEST_DIR"));

    let generated = pyroxide::generate("example").unwrap();
    let out_dir = std::env::var("OUT_DIR").unwrap();
    fs::write(Path::new(&out_dir).join("example.rs"), generated).unwrap();
}
