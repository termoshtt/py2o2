use crate::inspect::*;
use anyhow::Result;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;

fn format(tt: TokenStream2) -> String {
    prettyplease::unparse(&syn::parse_file(&tt.to_string()).unwrap())
}

pub fn as_input_type(ty: &Type) -> syn::Type {
    match ty {
        Type::Primitive(Primitive::Int) => syn::parse_quote!(i64),
        Type::Primitive(Primitive::Float) => syn::parse_quote!(f64),
        Type::Primitive(Primitive::Str) => syn::parse_quote!(&str),
        Type::None => syn::parse_quote!(()),
        Type::Tuple { tags } => {
            let tags: Vec<syn::Type> = tags.iter().map(as_input_type).collect();
            syn::parse_quote! { (#(#tags),*) }
        }
        Type::List { .. } => syn::parse_quote! { &::pyo3::types::PyList },
    }
}

pub fn as_output_type(ty: &Type) -> syn::Type {
    match ty {
        Type::Primitive(Primitive::Int) => syn::parse_quote!(i64),
        Type::Primitive(Primitive::Float) => syn::parse_quote!(f64),
        Type::Primitive(Primitive::Str) => syn::parse_quote!(&'py ::pyo3::types::PyString),
        Type::None => syn::parse_quote!(()),
        Type::Tuple { tags } => {
            let tags: Vec<syn::Type> = tags.iter().map(as_output_type).collect();
            syn::parse_quote! { (#(#tags),*) }
        }
        Type::List { .. } => {
            syn::parse_quote! { &'py ::pyo3::types::PyList }
        }
    }
}

pub fn generate_function(module_name: &str, f: &Function) -> Result<TokenStream2> {
    let name = &f.name;
    let ident = syn::Ident::new(name, Span::call_site());
    let param_names: Vec<_> = f
        .parameters
        .iter()
        .map(|p| syn::Ident::new(&p.name, Span::call_site()))
        .collect();
    let param_types: Vec<syn::Type> = f
        .parameters
        .iter()
        .map(|p| as_input_type(&p.r#type))
        .collect();
    let input_tt = quote!(#(#param_names: #param_types),*);

    let output = as_output_type(&f.r#return);

    let call_tt = quote! {
        py.import(#module_name)?.getattr(#name)?.call((#(#param_names,)*), None)?
    };
    let inner_tt = if matches!(&f.r#return, Type::None) {
        quote! {
            let _ = #call_tt;
            Ok(())
        }
    } else {
        quote! {
            let result = #call_tt;
            Ok(result.extract()?)
        }
    };

    Ok(quote! {
        pub fn #ident<'py>(py: ::pyo3::Python<'py>, #input_tt) -> ::pyo3::PyResult<#output> {
            #inner_tt
        }
    })
}

pub fn generate(module_name: &str, interface: &Interface, bare: bool) -> Result<String> {
    let mut tt = Vec::new();
    let f_tt = interface
        .functions
        .values()
        .map(|f| generate_function(module_name, f))
        .collect::<Result<Vec<_>>>()?;
    if !bare {
        let module_ident = syn::Ident::new(module_name, Span::call_site());
        tt.push(quote! {
            pub mod #module_ident {
                #(#f_tt)*
            }
        })
    } else {
        tt.push(quote! {
            #(#f_tt)*
        })
    }
    Ok(format(quote! { #(#tt)* }))
}

#[cfg(test)]
mod test {
    use super::*;
    const PYTHON_ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../python/");

    #[test]
    fn example() -> Result<()> {
        std::env::set_var("PYTHONPATH", PYTHON_ROOT);
        let interface = Interface::from_py_module("example")?;
        insta::assert_snapshot!(generate("example", &interface, true)?, @r###"
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
        ) -> ::pyo3::PyResult<&'py ::pyo3::types::PyString> {
            let result = py.import("example")?.getattr("a5")?.call((x,), None)?;
            Ok(result.extract()?)
        }
        pub fn a6<'py>(
            py: ::pyo3::Python<'py>,
        ) -> ::pyo3::PyResult<(i64, &'py ::pyo3::types::PyString)> {
            let result = py.import("example")?.getattr("a6")?.call((), None)?;
            Ok(result.extract()?)
        }
        pub fn a7<'py>(
            py: ::pyo3::Python<'py>,
            x: i64,
        ) -> ::pyo3::PyResult<(i64, &'py ::pyo3::types::PyString, f64)> {
            let result = py.import("example")?.getattr("a7")?.call((x,), None)?;
            Ok(result.extract()?)
        }
        "###);

        insta::assert_snapshot!(generate("example", &interface, false)?, @r###"
        pub mod example {
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
            ) -> ::pyo3::PyResult<&'py ::pyo3::types::PyString> {
                let result = py.import("example")?.getattr("a5")?.call((x,), None)?;
                Ok(result.extract()?)
            }
            pub fn a6<'py>(
                py: ::pyo3::Python<'py>,
            ) -> ::pyo3::PyResult<(i64, &'py ::pyo3::types::PyString)> {
                let result = py.import("example")?.getattr("a6")?.call((), None)?;
                Ok(result.extract()?)
            }
            pub fn a7<'py>(
                py: ::pyo3::Python<'py>,
                x: i64,
            ) -> ::pyo3::PyResult<(i64, &'py ::pyo3::types::PyString, f64)> {
                let result = py.import("example")?.getattr("a7")?.call((x,), None)?;
                Ok(result.extract()?)
            }
        }
        "###);
        Ok(())
    }

    #[test]
    fn type_aliases() -> Result<()> {
        std::env::set_var("PYTHONPATH", PYTHON_ROOT);
        let interface = Interface::from_py_module("type_aliases")?;
        insta::assert_snapshot!(generate("type_aliases", &interface, true)?, @r###"
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
        "###);

        insta::assert_snapshot!(generate("example", &interface, false)?, @r###"
        pub mod example {
            pub fn scale<'py>(
                py: ::pyo3::Python<'py>,
                scalar: f64,
                vector: &::pyo3::types::PyList,
            ) -> ::pyo3::PyResult<&'py ::pyo3::types::PyList> {
                let result = py
                    .import("example")?
                    .getattr("scale")?
                    .call((scalar, vector), None)?;
                Ok(result.extract()?)
            }
        }
        "###);
        Ok(())
    }
}
