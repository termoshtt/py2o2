use anyhow::{Context, Result};
use clap::Parser;
use std::path::Path;

#[derive(Parser, Debug)]
struct Cli {
    python_module_name_or_path: String,

    /// Generate Rust code without creating modules
    #[arg(short, long, default_value_t = false)]
    bare: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let path = Path::new(&cli.python_module_name_or_path);
    let module_name = if path.exists() {
        std::env::set_var("PYTHONPATH", path.canonicalize().unwrap().parent().unwrap());
        let name = path
            .file_name()
            .unwrap()
            .to_str()
            .context("Non UTF-8 filename")?;
        if let Some(inner) = name.strip_suffix(".py") {
            inner.to_string()
        } else {
            name.to_string()
        }
    } else {
        cli.python_module_name_or_path
    };
    println!("{}", pyroxide::generate(&module_name, cli.bare)?);
    Ok(())
}
