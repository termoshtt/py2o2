use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::path::Path;

#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Serialize Python interface into JSON
    Inspect { python_module_name_or_path: String },

    /// Generate Rust code for Python module
    Codegen {
        python_module_name_or_path: String,
        /// Generate Rust code without creating modules
        #[arg(short, long, default_value_t = false)]
        bare: bool,
    },
}

fn seek_py_module(python_module_name_or_path: &str) -> Result<String> {
    let path = Path::new(&python_module_name_or_path);
    if path.exists() {
        std::env::set_var("PYTHONPATH", path.canonicalize().unwrap().parent().unwrap());
        let name = path
            .file_name()
            .unwrap()
            .to_str()
            .context("Non UTF-8 filename")?;
        Ok(if let Some(inner) = name.strip_suffix(".py") {
            inner.to_string()
        } else {
            name.to_string()
        })
    } else {
        Ok(python_module_name_or_path.to_string())
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Codegen {
            python_module_name_or_path,
            bare,
        } => {
            let pymod = seek_py_module(&python_module_name_or_path)?;
            println!("{}", py2o2::generate(&pymod, bare)?);
        }
        Command::Inspect {
            python_module_name_or_path,
        } => {
            let pymod = seek_py_module(&python_module_name_or_path)?;
            println!("{}", py2o2::inspect(&pymod)?);
        }
    }
    Ok(())
}
