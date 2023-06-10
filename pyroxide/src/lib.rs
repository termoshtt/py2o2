pub mod codegen;
pub mod wit;

use anyhow::Result;

pub fn generate(python_module_name: &str, bare: bool) -> Result<String> {
    let (_wit, path) = wit::witgen(python_module_name)?;
    let wit = wit::parse(&path)?;
    let generated = codegen::generate_from_wit(&wit, bare)?;
    Ok(generated)
}
