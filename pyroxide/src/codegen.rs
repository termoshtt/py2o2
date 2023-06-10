use anyhow::Result;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;

fn format(tt: TokenStream2) -> String {
    prettyplease::unparse(&syn::parse_file(&tt.to_string()).unwrap())
}

fn as_input_type(
    ty: &wit_parser::Type,
    definitions: &id_arena::Arena<wit_parser::TypeDef>,
) -> syn::Type {
    match ty {
        wit_parser::Type::S64 => syn::parse_quote!(i64),
        wit_parser::Type::U64 => syn::parse_quote!(u64),
        wit_parser::Type::Float64 => syn::parse_quote!(f64),
        wit_parser::Type::Float32 => syn::parse_quote!(f32),
        wit_parser::Type::String => syn::parse_quote!(&str),
        wit_parser::Type::Id(id) => {
            use wit_parser::TypeDefKind;
            let def = &definitions[*id];
            match def.kind {
                TypeDefKind::List(_) => {
                    syn::parse_quote!(&::pyo3::types::PyList)
                }
                _ => unimplemented!("Type definition = {:?}", def),
            }
        }
        _ => unimplemented!("ty = {:?}", ty),
    }
}

fn as_output_type(
    ty: &wit_parser::Type,
    definitions: &id_arena::Arena<wit_parser::TypeDef>,
) -> syn::Type {
    match ty {
        wit_parser::Type::S64 => syn::parse_quote!(i64),
        wit_parser::Type::U64 => syn::parse_quote!(u64),
        wit_parser::Type::Float64 => syn::parse_quote!(f64),
        wit_parser::Type::Float32 => syn::parse_quote!(f32),
        wit_parser::Type::String => syn::parse_quote!(&'py ::pyo3::types::PyString),
        wit_parser::Type::Id(id) => {
            use wit_parser::TypeDefKind;
            let def = &definitions[*id];
            match def.kind {
                TypeDefKind::List(_) => {
                    syn::parse_quote!(&'py ::pyo3::types::PyList)
                }
                _ => unimplemented!("Type definition = {:?}", def),
            }
        }
        _ => unimplemented!("ty = {:?}", ty),
    }
}

pub fn generate_function(
    module_name: &str,
    name: &str,
    f: &wit_parser::Function,
    definitions: &id_arena::Arena<wit_parser::TypeDef>,
) -> Result<TokenStream2> {
    let ident = syn::Ident::new(name, Span::call_site());
    let param_names: Vec<_> = f
        .params
        .iter()
        .map(|(name, _)| syn::Ident::new(name, Span::call_site()))
        .collect();
    let param_types: Vec<syn::Type> = f
        .params
        .iter()
        .map(|(_, ty)| as_input_type(ty, &definitions))
        .collect();
    let input_tt = quote!(#(#param_names: #param_types),*);

    let output = match &f.results {
        wit_parser::Results::Named(params) => {
            let params: Vec<syn::Type> = params
                .iter()
                .map(|(_, ty)| as_output_type(ty, &definitions))
                .collect();
            syn::parse_quote!((#(#params),*))
        }
        wit_parser::Results::Anon(ty) => as_output_type(ty, &definitions),
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

    Ok(quote! {
        pub fn #ident<'py>(py: ::pyo3::Python<'py>, #input_tt) -> ::pyo3::PyResult<#output> {
            #inner_tt
        }
    })
}

pub fn generate_from_wit(wit: wit_parser::Resolve, bare: bool) -> Result<String> {
    let mut tt = Vec::new();
    for (_id, interface) in &wit.interfaces {
        let module_name = interface.name.as_ref().unwrap().replace('-', "_");
        let f_tt = interface
            .functions
            .iter()
            .map(|(name, f)| generate_function(&module_name, name, f, &wit.types))
            .collect::<Result<Vec<_>>>()?;
        if !bare {
            let module_ident = syn::Ident::new(&module_name, Span::call_site());
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
    }
    Ok(format(quote! { #(#tt)* }))
}
