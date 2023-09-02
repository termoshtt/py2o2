use crate::inspect::*;
use anyhow::Result;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{format_ident, quote};
use std::collections::{hash_map::DefaultHasher, BTreeMap};
use std::hash::{Hash, Hasher};

fn format(tt: TokenStream2) -> String {
    let parsed = &syn::parse_file(&tt.to_string()).unwrap();
    prettyplease::unparse(parsed)
}

fn union_trait_ident(args: &[Type]) -> syn::Ident {
    let mut s = DefaultHasher::new();
    for arg in args {
        arg.hash(&mut s);
    }
    let hash = s.finish();
    quote::format_ident!("Union{:x}", hash)
}

fn callable_trait(args: &[Type], ret: &Type) -> TokenStream2 {
    let args = if args.is_empty() {
        quote! {}
    } else {
        let args: Vec<_> = args.iter().map(as_input_type).collect();
        quote! {(#(#args,)*)}
    };
    let out = as_output_type(ret);
    quote!(Fn(#args) -> #out + Send + 'static)
}

pub fn as_input_type(ty: &Type) -> syn::Type {
    match ty {
        Type::Primitive(Primitive::Int) => syn::parse_quote!(i64),
        Type::Primitive(Primitive::Float) => syn::parse_quote!(f64),
        Type::Primitive(Primitive::Str) => syn::parse_quote!(&str),
        Type::None => syn::parse_quote!(()),
        Type::Ellipsis | Type::Exception => syn::parse_quote!(::pyo3::Py<::pyo3::PyAny>),
        Type::Tuple { tags } => {
            let tags: Vec<syn::Type> = tags.iter().map(as_input_type).collect();
            syn::parse_quote! { (#(#tags),*) }
        }
        Type::List { .. } => syn::parse_quote! { &::pyo3::types::PyList },
        Type::Dict { .. } => syn::parse_quote! { &::pyo3::types::PyDict },
        Type::UserDefined { name, .. } => {
            let ty = syn::Ident::new(name, Span::call_site());
            syn::parse_quote!(#ty)
        }
        Type::Union { args } => {
            let ident = union_trait_ident(args);
            syn::parse_quote!(impl #ident)
        }
        Type::Callable { args, r#return } => {
            let t = callable_trait(args, r#return);
            syn::parse_quote!(impl #t)
        }
    }
}

pub fn as_output_type(ty: &Type) -> syn::Type {
    match ty {
        Type::Primitive(Primitive::Int) => syn::parse_quote!(i64),
        Type::Primitive(Primitive::Float) => syn::parse_quote!(f64),
        Type::Primitive(Primitive::Str) => syn::parse_quote!(::pyo3::Py<::pyo3::types::PyString>),
        Type::None => syn::parse_quote!(()),
        Type::Ellipsis | Type::Exception => syn::parse_quote!(::pyo3::Py<::pyo3::PyAny>),
        Type::Tuple { tags } => {
            let tags: Vec<syn::Type> = tags.iter().map(as_output_type).collect();
            syn::parse_quote! { (#(#tags),*) }
        }
        Type::List { .. } => syn::parse_quote!(::pyo3::Py<::pyo3::types::PyList>),
        Type::Dict { .. } => syn::parse_quote!(::pyo3::Py<::pyo3::types::PyDict>),
        Type::UserDefined { name, .. } => {
            let ty = syn::Ident::new(name, Span::call_site());
            syn::parse_quote!(#ty)
        }
        Type::Union { args } => {
            let n = args.len();
            let enum_ = quote::format_ident!("Enum{}", n);
            let out: Vec<_> = args.iter().map(as_output_type).collect();
            syn::parse_quote!( ::py2o2_runtime::#enum_ <#(#out),*>)
        }
        Type::Callable { args, r#return } => {
            let t = callable_trait(args, r#return);
            syn::parse_quote!(Box<#t>)
        }
    }
}

pub fn as_ident(name: &str) -> syn::Ident {
    const RUST_KEYWORDS: &[&str] = &["self"];
    for keyword in RUST_KEYWORDS {
        if name == *keyword {
            return format_ident!("_{}", name);
        }
    }
    syn::Ident::new(name, Span::call_site())
}

pub fn generate_function(module_name: &str, f: &Function) -> Result<TokenStream2> {
    let name = &f.name;
    let ident = syn::Ident::new(name, Span::call_site());
    let param_names: Vec<_> = f.parameters.iter().map(|p| as_ident(&p.name)).collect();
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
    let convert_callable: Vec<TokenStream2> = f
        .parameters
        .iter()
        .flat_map(|p| match &p.r#type {
            Type::Callable { args, .. } => {
                let ident = syn::Ident::new(&p.name, Span::call_site());
                Some(if args.is_empty() {
                    quote! {
                        let #ident = ::py2o2_runtime::as_pycfunc(py, move |_input: [usize; 0]| #ident())?;
                    }
                } else {
                    quote! {
                        let #ident = ::py2o2_runtime::as_pycfunc(py, #ident)?;
                    }
                })
            }
            _ => None,
        })
        .collect();

    Ok(quote! {
        pub fn #ident<'py>(py: ::pyo3::Python<'py>, #input_tt) -> ::pyo3::PyResult<#output> {
            #(#convert_callable)*
            #inner_tt
        }
    })
}

pub fn generate_type_definitions(typedef: &TypeDefinition) -> Result<TokenStream2> {
    let TypeDefinition {
        name, supertype, ..
    } = typedef;
    let inner = as_output_type(supertype);
    let name = syn::Ident::new(name, Span::call_site());
    Ok(quote! {
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct #name(pub #inner);

        impl ::pyo3::conversion::IntoPy<::pyo3::PyObject> for #name {
            fn into_py(self, py: ::pyo3::Python<'_>) -> ::pyo3::PyObject {
                self.0.into_py(py)
            }
        }
    })
}

pub fn generate_union_traits(interface: &Interface) -> Result<TokenStream2> {
    let mut traits: BTreeMap<syn::Ident, TokenStream2> = BTreeMap::new();
    for f in interface.functions.values() {
        for p in &f.parameters {
            match &p.r#type {
                Type::Union { args } => {
                    let trait_ident = union_trait_ident(args);
                    let args: Vec<_> = args
                        .iter()
                        .map(|ty| match ty {
                            Type::Primitive(_) => as_input_type(ty),
                            _ => unimplemented!(),
                        })
                        .collect();
                    traits.entry(trait_ident.clone()).or_insert(quote! {
                        pub trait #trait_ident: ::pyo3::conversion::IntoPy<::pyo3::PyObject> {}
                        #(
                        impl #trait_ident for #args {}
                        )*
                    });
                }
                _ => continue,
            }
        }
    }
    let traits: Vec<_> = traits.values().collect();
    Ok(quote! { #(#traits)* })
}

pub fn generate(module_name: &str, interface: &Interface, bare: bool) -> Result<String> {
    let mut tt = Vec::new();
    let f_tt = interface
        .functions
        .values()
        .map(|f| generate_function(module_name, f))
        .collect::<Result<Vec<_>>>()?;
    let typedef_tt = interface
        .newtypes
        .values()
        .map(generate_type_definitions)
        .collect::<Result<Vec<_>>>()?;
    let union_traits = generate_union_traits(interface)?;
    if !bare {
        let module_ident = syn::Ident::new(module_name, Span::call_site());
        tt.push(quote! {
            pub mod #module_ident {
                #(#typedef_tt)*
                #union_traits
                #(#f_tt)*
            }
        })
    } else {
        tt.push(quote! {
            #(#typedef_tt)*
            #union_traits
            #(#f_tt)*
        })
    }
    Ok(format(quote! { #(#tt)* }))
}
