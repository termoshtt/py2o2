pub fn async_query<'py>(
    py: ::pyo3::Python<'py>,
    on_success: &::pyo3::types::PyCFunction,
    on_error: &::pyo3::types::PyCFunction,
) -> ::pyo3::PyResult<()> {
    let _ = py
        .import("callable")?
        .getattr("async_query")?
        .call((on_success, on_error), None)?;
    Ok(())
}
pub fn caller<'py>(
    py: ::pyo3::Python<'py>,
    f: &::pyo3::types::PyCFunction,
) -> ::pyo3::PyResult<()> {
    let _ = py.import("callable")?.getattr("caller")?.call((f,), None)?;
    Ok(())
}
pub fn ellipsis_callable<'py>(
    py: ::pyo3::Python<'py>,
    f: &::pyo3::types::PyCFunction,
) -> ::pyo3::PyResult<()> {
    let _ = py
        .import("callable")?
        .getattr("ellipsis_callable")?
        .call((f,), None)?;
    Ok(())
}
pub fn feeder<'py>(
    py: ::pyo3::Python<'py>,
    get_next_item: &::pyo3::types::PyCFunction,
) -> ::pyo3::PyResult<()> {
    let _ = py
        .import("callable")?
        .getattr("feeder")?
        .call((get_next_item,), None)?;
    Ok(())
}
