//! Binding to the Python's `importlib`

use py2o2_runtime::import_pytype;
use pyo3::prelude::*;

import_pytype!(importlib.ModuleSpec);

pub fn find_spec<'py>(py: Python<'py>, _name: &str) -> Option<ModuleSpec<'py>> {
    let _importlib_util = py.import("importlib.util").unwrap();

    todo!()
}
