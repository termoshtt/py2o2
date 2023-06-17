pub mod codegen;
pub mod inspect;

use anyhow::Result;

pub fn generate(python_module_name: &str, bare: bool) -> Result<String> {
    let interface = inspect::Interface::from_py_module(python_module_name)?;
    let generated = codegen::generate(python_module_name, &interface, bare)?;
    Ok(generated)
}
