extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{self, DeriveInput};

#[proc_macro_derive(CheckStatus)]
pub fn check_status_derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl CheckStatus for #name {
            fn check_status(&self) -> bool {
                self.ret_code == 0u8
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(Len)]
pub fn len_derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let id = &ast.ident;
    let mut i = 0;
    let gen = quote! {
        impl Len for #id {
            fn len() -> u16 {
                // for f in
            }
        }
    };
    gen.into()
}
