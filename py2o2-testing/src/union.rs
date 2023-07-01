pub trait Union5d6b010906f780ce: ::pyo3::conversion::IntoPy<::pyo3::PyObject> {}
impl Union5d6b010906f780ce for i64 {}
impl Union5d6b010906f780ce for &str {}
pub fn f_new<'py>(
    py: ::pyo3::Python<'py>,
    a: impl Union5d6b010906f780ce,
) -> ::pyo3::PyResult<::py2o2_runtime::Enum2<i64, ::pyo3::Py<::pyo3::types::PyString>>> {
    let result = py.import("union")?.getattr("f_new")?.call((a,), None)?;
    Ok(result.extract()?)
}
pub fn f_old<'py>(
    py: ::pyo3::Python<'py>,
    a: impl Union5d6b010906f780ce,
) -> ::pyo3::PyResult<::py2o2_runtime::Enum2<i64, ::pyo3::Py<::pyo3::types::PyString>>> {
    let result = py.import("union")?.getattr("f_old")?.call((a,), None)?;
    Ok(result.extract()?)
}
