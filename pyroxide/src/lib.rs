pub mod codegen;
pub mod wit;

use anyhow::Result;

pub fn generate(python_module_name: &str) -> Result<String> {
    let wit = wit::save_wit(python_module_name)?;
    let generated = codegen::generate_from_wit(&wit)?;
    Ok(generated)
}
