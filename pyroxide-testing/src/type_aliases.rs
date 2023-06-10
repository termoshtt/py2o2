pub mod type_aliases {
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
}

