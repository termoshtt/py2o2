pub fn async_query<'py>(
    py: ::pyo3::Python<'py>,
    on_success: impl Fn((i64,)) -> () + Send + 'static,
    on_error: impl Fn((i64, ::pyo3::Py<::pyo3::PyAny>)) -> () + Send + 'static,
) -> ::pyo3::PyResult<()> {
    let on_success = ::py2o2_runtime::as_pycfunc(py, on_success)?;
    let on_error = ::py2o2_runtime::as_pycfunc(py, on_error)?;
    let _ = py
        .import("callable")?
        .getattr("async_query")?
        .call((on_success, on_error), None)?;
    Ok(())
}
pub fn caller<'py>(
    py: ::pyo3::Python<'py>,
    f: impl Fn((i64, f64)) -> f64 + Send + 'static,
) -> ::pyo3::PyResult<()> {
    let f = ::py2o2_runtime::as_pycfunc(py, f)?;
    let _ = py.import("callable")?.getattr("caller")?.call((f,), None)?;
    Ok(())
}
pub fn ellipsis_callable<'py>(
    py: ::pyo3::Python<'py>,
    f: impl Fn((::pyo3::Py<::pyo3::PyAny>,)) -> () + Send + 'static,
) -> ::pyo3::PyResult<()> {
    let f = ::py2o2_runtime::as_pycfunc(py, f)?;
    let _ = py
        .import("callable")?
        .getattr("ellipsis_callable")?
        .call((f,), None)?;
    Ok(())
}
pub fn feeder<'py>(
    py: ::pyo3::Python<'py>,
    get_next_item: impl Fn() -> ::pyo3::Py<::pyo3::types::PyString> + Send + 'static,
) -> ::pyo3::PyResult<()> {
    let get_next_item = ::py2o2_runtime::as_pycfunc(py, move |_input: [usize; 0]| get_next_item())?;
    let _ = py
        .import("callable")?
        .getattr("feeder")?
        .call((get_next_item,), None)?;
    Ok(())
}
