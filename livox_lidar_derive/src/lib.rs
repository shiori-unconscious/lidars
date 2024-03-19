extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Data, DeriveInput};

#[proc_macro_derive(CheckStatus)]
pub fn check_status_derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let Data::Struct(ds) = ast.data else {
        panic!("Trait CheckStatus derive must be use on struct");
    };
    if !ds
        .fields
        .iter()
        .any(|f| f.ident.as_ref().is_some_and(|name| name == "ret_code"))
    {
        panic!("Trait CheckStatus needs struct field 'ret_code'");
    }
    let gen = quote! {
        impl CheckStatus for #name {
            fn check_status(&self) -> Result<()> {
                if self.ret_code == 0u8 {
                    Ok(())
                }
                else {
                    Err(anyhow!("{} failed on checking status code ❌", stringify!(#name)))
                }
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
    let size_sum = TokenStream2::from_iter(ds.fields.iter().map(|f| {
        let ty = &f.ty;
        quote! {
            std::mem::size_of::<#ty>(),
        }
    }));
    let gen = quote! {
        impl Len for #id {
            fn len() -> u16 {
                [#size_sum].iter().fold(0,|s,x|x+s) as u16
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(GetCmd)]
pub fn cmd_derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let id = &ast.ident;
    let Data::Struct(ds) = ast.data else {
        panic!("Trait GetCmd derive must be use on struct");
    };

    let Some(field) = ds.fields.into_iter().enumerate().find_map(|(idx, f)|{
        if let Some(field_id) = f.ident {
            if field_id == "cmd" {
                Some(quote!(#field_id))
            }
            else{
                None
            }
        }
        else {
            let idx_str = syn::Index::from(idx);
            Some(quote!(#idx_str))
        }
    }) else {
        panic!("Trait GetCmd needs struct field 'cmd'");
    };
   
    let gen = quote! {
        impl GetCmd for #id {
            fn cmd(&self) -> Cmd {
                self.#field
            }
        }
    };
    gen.into()
}