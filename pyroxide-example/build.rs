use std::{fs, path::Path};

fn main() {
    // Add a path where `example.py` exists
    let project_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap();
    std::env::set_var("PYTHONPATH", project_root);

    let generated = pyroxide::generate("example").unwrap();
    let out_dir = std::env::var("OUT_DIR").unwrap();
    fs::write(Path::new(&out_dir).join("example.rs"), generated).unwrap();
}
