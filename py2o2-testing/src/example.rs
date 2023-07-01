pub fn a1<'py>(py: ::pyo3::Python<'py>) -> ::pyo3::PyResult<()> {
    let _ = py.import("example")?.getattr("a1")?.call((), None)?;
    Ok(())
}
pub fn a2<'py>(py: ::pyo3::Python<'py>, x: i64) -> ::pyo3::PyResult<()> {
    let _ = py.import("example")?.getattr("a2")?.call((x,), None)?;
    Ok(())
}
pub fn a3<'py>(py: ::pyo3::Python<'py>, y: &str, z: f64) -> ::pyo3::PyResult<()> {
    let _ = py.import("example")?.getattr("a3")?.call((y, z), None)?;
    Ok(())
}
pub fn a4<'py>(py: ::pyo3::Python<'py>) -> ::pyo3::PyResult<i64> {
    let result = py.import("example")?.getattr("a4")?.call((), None)?;
    Ok(result.extract()?)
}
pub fn a5<'py>(
    py: ::pyo3::Python<'py>,
    x: i64,
) -> ::pyo3::PyResult<::pyo3::Py<::pyo3::types::PyString>> {
    let result = py.import("example")?.getattr("a5")?.call((x,), None)?;
    Ok(result.extract()?)
}
pub fn a6<'py>(
    py: ::pyo3::Python<'py>,
) -> ::pyo3::PyResult<(i64, ::pyo3::Py<::pyo3::types::PyString>)> {
    let result = py.import("example")?.getattr("a6")?.call((), None)?;
    Ok(result.extract()?)
}
pub fn a7<'py>(
    py: ::pyo3::Python<'py>,
    x: i64,
) -> ::pyo3::PyResult<(i64, ::pyo3::Py<::pyo3::types::PyString>, f64)> {
    let result = py.import("example")?.getattr("a7")?.call((x,), None)?;
    Ok(result.extract()?)
}
