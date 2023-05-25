use proc_macro::TokenStream;
use proc_macro_error::*;
use quote::quote;

#[proc_macro_error]
#[proc_macro]
pub fn import(_input: TokenStream) -> TokenStream {
    quote! {}.into()
}
