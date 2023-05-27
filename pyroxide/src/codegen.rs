use anyhow::Result;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;

fn format(tt: TokenStream2) -> String {
    prettyplease::unparse(&syn::parse_file(&tt.to_string()).unwrap())
}

fn as_input_type(ty: &wit_parser::Type) -> syn::Type {
    match ty {
        wit_parser::Type::S64 => syn::parse_quote!(i64),
        wit_parser::Type::U64 => syn::parse_quote!(u64),
        wit_parser::Type::Float64 => syn::parse_quote!(f64),
        wit_parser::Type::Float32 => syn::parse_quote!(f32),
        wit_parser::Type::String => syn::parse_quote!(&str),
        _ => unimplemented!(),
    }
}

fn as_output_type(ty: &wit_parser::Type) -> syn::Type {
    match ty {
        wit_parser::Type::S64 => syn::parse_quote!(i64),
        wit_parser::Type::U64 => syn::parse_quote!(u64),
        wit_parser::Type::Float64 => syn::parse_quote!(f64),
        wit_parser::Type::Float32 => syn::parse_quote!(f32),
        wit_parser::Type::String => syn::parse_quote!(&'py PyString),
        _ => unimplemented!(),
    }
}

fn as_rust_tuple(params: &[(String, wit_parser::Type)]) -> syn::Type {
    let params: Vec<syn::Type> = params.iter().map(|(_, ty)| as_output_type(ty)).collect();
    syn::parse_quote!((#(#params),*))
}

pub fn generate_from_wit(interfaces: &[wit_parser::Interface]) -> Result<String> {
    let mut tt = Vec::new();
    for interface in interfaces {
        let module_name = interface.name.as_ref().unwrap();
        let module_ident = syn::Ident::new(module_name, Span::call_site());
        let mut f_tt = Vec::new();
        for (name, f) in &interface.functions {
            let ident = syn::Ident::new(&name, Span::call_site());
            let param_names: Vec<_> = f
                .params
                .iter()
                .map(|(name, _)| syn::Ident::new(name, Span::call_site()))
                .collect();
            let param_types: Vec<syn::Type> =
                f.params.iter().map(|(_, ty)| as_input_type(ty)).collect();
            let input_tt = quote!(#(#param_names: #param_types),*);

            let output = match &f.results {
                wit_parser::Results::Named(params) => as_rust_tuple(params),
                wit_parser::Results::Anon(ty) => as_output_type(ty),
            };

            let call_tt = quote! {
                py.import(#module_name)?.getattr(#name)?.call((#(#param_names,)*), None)?
            };
            let inner_tt = if output == syn::parse_quote!(()) {
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

            f_tt.push(quote! {
                pub fn #ident<'py>(py: Python<'py>, #input_tt) -> PyResult<#output> {
                    #inner_tt
                }
            });
        }
        tt.push(quote! {
            pub mod #module_ident {
                use pyo3::{prelude::*, types::PyString};
                #(#f_tt)*
            }
        })
    }
    Ok(format(quote! { #(#tt)* }))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn generate_from_test_wit() {
        let project_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap();
        let interfaces = crate::wit::parse(&project_root.join("test.wit")).unwrap();
        let tt = generate_from_wit(&interfaces).unwrap();
        insta::assert_snapshot!(tt, @r###"
        pub mod example {
            use pyo3::{prelude::*, types::PyString};
            pub fn a1<'py>(py: Python<'py>) -> PyResult<()> {
                let _ = py.import("example")?.getattr("a1")?.call((), None)?;
                Ok(())
            }
            pub fn a2<'py>(py: Python<'py>, x: i64) -> PyResult<()> {
                let _ = py.import("example")?.getattr("a2")?.call((x,), None)?;
                Ok(())
            }
            pub fn a3<'py>(py: Python<'py>, y: &str, z: f64) -> PyResult<()> {
                let _ = py.import("example")?.getattr("a3")?.call((y, z), None)?;
                Ok(())
            }
            pub fn a4<'py>(py: Python<'py>) -> PyResult<i64> {
                let result = py.import("example")?.getattr("a4")?.call((), None)?;
                Ok(result.extract()?)
            }
            pub fn a5<'py>(py: Python<'py>, x: i64) -> PyResult<&'py PyString> {
                let result = py.import("example")?.getattr("a5")?.call((x,), None)?;
                Ok(result.extract()?)
            }
            pub fn a6<'py>(py: Python<'py>) -> PyResult<(i64, &'py PyString)> {
                let result = py.import("example")?.getattr("a6")?.call((), None)?;
                Ok(result.extract()?)
            }
            pub fn a7<'py>(py: Python<'py>, x: i64) -> PyResult<(i64, &'py PyString, f64)> {
                let result = py.import("example")?.getattr("a7")?.call((x,), None)?;
                Ok(result.extract()?)
            }
        }
        "###);
    }
}
