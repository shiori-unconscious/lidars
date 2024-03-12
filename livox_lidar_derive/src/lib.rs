extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{self, Data, DeriveInput};

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
    let Data::Struct(ds) = ast.data else {
        panic!("Len Derive Must be Use on struct");
    };
    let gen = quote! {
        impl Len for #id {
            fn len() -> u16 {
                for f in ds.fields {

                }
                std::mem::size_of<>()
            }
        }
    };
    gen.into()
}
