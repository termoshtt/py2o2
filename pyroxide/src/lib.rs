pub mod codegen2;
pub mod inspect;
pub mod wit;

use anyhow::Result;

pub fn generate(python_module_name: &str, bare: bool) -> Result<String> {
    let interface = inspect::Interface::from_py_module(python_module_name)?;
    let generated = codegen2::generate(python_module_name, &interface, bare)?;
    Ok(generated)
}
