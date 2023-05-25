use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use proc_macro_error::*;
use quote::quote;
use std::path::Path;

#[proc_macro_error]
#[proc_macro]
pub fn import(_input: TokenStream) -> TokenStream {
    quote! {}.into()
}

fn generate_from_wit(_wit_path: &Path) -> TokenStream2 {
    quote! {}
}

#[cfg(test)]
mod test {
    use super::*;

    fn format(tt: TokenStream2) -> String {
        prettyplease::unparse(&syn::parse_file(&tt.to_string()).unwrap())
    }

    #[test]
    fn test_wit() {
        let project_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap();
        let tt = generate_from_wit(&project_root.join("test.wit"));
        insta::assert_snapshot!(format(tt), @r###""###);
    }
}
