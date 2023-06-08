use anyhow::{Context, Result};
use clap::Parser;
use std::path::Path;

#[derive(Parser, Debug)]
struct Cli {
    python_module_name_or_path: String,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let path = Path::new(&cli.python_module_name_or_path);
    if path.is_file() {
        let python_root = path.canonicalize().unwrap().parent().unwrap().to_owned();
        let name = path
            .file_name()
            .unwrap()
            .to_str()
            .context("Non UTF-8 filename")?;
        println!(
            "{}",
            pyroxide::generate(
                &name
                    .strip_suffix(".py")
                    .context("Input must be Python script")?,
                Some(&python_root)
            )?
        );
    } else {
        println!(
            "{}",
            pyroxide::generate(&cli.python_module_name_or_path, None::<&Path>)?
        );
    }
    Ok(())
}
