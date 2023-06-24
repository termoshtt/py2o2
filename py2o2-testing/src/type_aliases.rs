#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UserId(pub i64);
impl ::pyo3::conversion::IntoPy<::pyo3::PyObject> for UserId {
    fn into_py(self, py: ::pyo3::Python<'_>) -> ::pyo3::PyObject {
        self.0.into_py(py)
    }
}
pub fn broadcast_message<'py>(
    py: ::pyo3::Python<'py>,
    message: &str,
    servers: &::pyo3::types::PyList,
) -> ::pyo3::PyResult<()> {
    let _ = py
        .import("type_aliases")?
        .getattr("broadcast_message")?
        .call((message, servers), None)?;
    Ok(())
}
pub fn get_user_name<'py>(
    py: ::pyo3::Python<'py>,
    user_id: UserId,
) -> ::pyo3::PyResult<&'py ::pyo3::types::PyString> {
    let result = py
        .import("type_aliases")?
        .getattr("get_user_name")?
        .call((user_id,), None)?;
    Ok(result.extract()?)
}
pub fn scale<'py>(
    py: ::pyo3::Python<'py>,
    scalar: f64,
    vector: &::pyo3::types::PyList,
) -> ::pyo3::PyResult<&'py ::pyo3::types::PyList> {
    let result = py
        .import("type_aliases")?
        .getattr("scale")?
        .call((scalar, vector), None)?;
    Ok(result.extract()?)
}
