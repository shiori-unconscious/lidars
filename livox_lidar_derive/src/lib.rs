extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{self, Data, DeriveInput};

#[proc_macro_derive(CheckStatus)]
pub fn check_status_derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let Data::Struct(ds) = ast.data else {
        panic!("Trait CheckStatus derive must be use on struct");
    };
    if ds
        .fields
        .iter()
        .any(|f| f.ident.as_ref().is_some_and(|name| name == "ret_code"))
    {
        panic!("Trait CheckStatus needs struct field 'ret_code'");
    }
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
        panic!("Trait Len derive must be use on struct");
    };
    let len = ds.fields.iter().fold(0u16, |accumulate,f|{
        accumulate + std::mem::size_of::<{ f.ty }>() as u16
    });
    let gen = quote! {
        impl Len for #id {
            fn len() -> u16 {
                #len
            }
        }
    };
    gen.into()
}
