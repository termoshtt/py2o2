pub fn _dims_getter<'py>(py: ::pyo3::Python<'py>, _self: ()) -> ::pyo3::PyResult<()> {
    let _ = py
        .import("ast")?
        .getattr("_dims_getter")?
        .call((_self,), None)?;
    Ok(())
}
pub fn _dims_setter<'py>(py: ::pyo3::Python<'py>, _self: (), value: ()) -> ::pyo3::PyResult<()> {
    let _ = py
        .import("ast")?
        .getattr("_dims_setter")?
        .call((_self, value), None)?;
    Ok(())
}
pub fn _getter<'py>(py: ::pyo3::Python<'py>, _self: ()) -> ::pyo3::PyResult<()> {
    let _ = py.import("ast")?.getattr("_getter")?.call((_self,), None)?;
    Ok(())
}
pub fn _new<'py>(py: ::pyo3::Python<'py>, cls: (), args: (), kwargs: ()) -> ::pyo3::PyResult<()> {
    let _ = py
        .import("ast")?
        .getattr("_new")?
        .call((cls, args, kwargs), None)?;
    Ok(())
}
pub fn _pad_whitespace<'py>(py: ::pyo3::Python<'py>, source: ()) -> ::pyo3::PyResult<()> {
    let _ = py
        .import("ast")?
        .getattr("_pad_whitespace")?
        .call((source,), None)?;
    Ok(())
}
pub fn _setter<'py>(py: ::pyo3::Python<'py>, _self: (), value: ()) -> ::pyo3::PyResult<()> {
    let _ = py
        .import("ast")?
        .getattr("_setter")?
        .call((_self, value), None)?;
    Ok(())
}
pub fn _simple_enum<'py>(
    py: ::pyo3::Python<'py>,
    etype: (),
    boundary: (),
    use_args: (),
) -> ::pyo3::PyResult<()> {
    let _ = py
        .import("ast")?
        .getattr("_simple_enum")?
        .call((etype, boundary, use_args), None)?;
    Ok(())
}
pub fn _splitlines_no_ff<'py>(py: ::pyo3::Python<'py>, source: ()) -> ::pyo3::PyResult<()> {
    let _ = py
        .import("ast")?
        .getattr("_splitlines_no_ff")?
        .call((source,), None)?;
    Ok(())
}
pub fn contextmanager<'py>(py: ::pyo3::Python<'py>, func: ()) -> ::pyo3::PyResult<()> {
    let _ = py
        .import("ast")?
        .getattr("contextmanager")?
        .call((func,), None)?;
    Ok(())
}
pub fn copy_location<'py>(
    py: ::pyo3::Python<'py>,
    new_node: (),
    old_node: (),
) -> ::pyo3::PyResult<()> {
    let _ = py
        .import("ast")?
        .getattr("copy_location")?
        .call((new_node, old_node), None)?;
    Ok(())
}
pub fn dump<'py>(
    py: ::pyo3::Python<'py>,
    node: (),
    annotate_fields: (),
    include_attributes: (),
    indent: (),
) -> ::pyo3::PyResult<()> {
    let _ = py
        .import("ast")?
        .getattr("dump")?
        .call((node, annotate_fields, include_attributes, indent), None)?;
    Ok(())
}
pub fn fix_missing_locations<'py>(py: ::pyo3::Python<'py>, node: ()) -> ::pyo3::PyResult<()> {
    let _ = py
        .import("ast")?
        .getattr("fix_missing_locations")?
        .call((node,), None)?;
    Ok(())
}
pub fn get_docstring<'py>(py: ::pyo3::Python<'py>, node: (), clean: ()) -> ::pyo3::PyResult<()> {
    let _ = py
        .import("ast")?
        .getattr("get_docstring")?
        .call((node, clean), None)?;
    Ok(())
}
pub fn get_source_segment<'py>(
    py: ::pyo3::Python<'py>,
    source: (),
    node: (),
    padded: (),
) -> ::pyo3::PyResult<()> {
    let _ = py
        .import("ast")?
        .getattr("get_source_segment")?
        .call((source, node, padded), None)?;
    Ok(())
}
pub fn increment_lineno<'py>(py: ::pyo3::Python<'py>, node: (), n: ()) -> ::pyo3::PyResult<()> {
    let _ = py
        .import("ast")?
        .getattr("increment_lineno")?
        .call((node, n), None)?;
    Ok(())
}
pub fn iter_child_nodes<'py>(py: ::pyo3::Python<'py>, node: ()) -> ::pyo3::PyResult<()> {
    let _ = py
        .import("ast")?
        .getattr("iter_child_nodes")?
        .call((node,), None)?;
    Ok(())
}
pub fn iter_fields<'py>(py: ::pyo3::Python<'py>, node: ()) -> ::pyo3::PyResult<()> {
    let _ = py
        .import("ast")?
        .getattr("iter_fields")?
        .call((node,), None)?;
    Ok(())
}
pub fn literal_eval<'py>(py: ::pyo3::Python<'py>, node_or_string: ()) -> ::pyo3::PyResult<()> {
    let _ = py
        .import("ast")?
        .getattr("literal_eval")?
        .call((node_or_string,), None)?;
    Ok(())
}
pub fn main<'py>(py: ::pyo3::Python<'py>) -> ::pyo3::PyResult<()> {
    let _ = py.import("ast")?.getattr("main")?.call((), None)?;
    Ok(())
}
pub fn parse<'py>(
    py: ::pyo3::Python<'py>,
    source: (),
    filename: (),
    mode: (),
    type_comments: (),
    feature_version: (),
) -> ::pyo3::PyResult<()> {
    let _ = py.import("ast")?.getattr("parse")?.call(
        (source, filename, mode, type_comments, feature_version),
        None,
    )?;
    Ok(())
}
pub fn unparse<'py>(py: ::pyo3::Python<'py>, ast_obj: ()) -> ::pyo3::PyResult<()> {
    let _ = py
        .import("ast")?
        .getattr("unparse")?
        .call((ast_obj,), None)?;
    Ok(())
}
pub fn walk<'py>(py: ::pyo3::Python<'py>, node: ()) -> ::pyo3::PyResult<()> {
    let _ = py.import("ast")?.getattr("walk")?.call((node,), None)?;
    Ok(())
}
