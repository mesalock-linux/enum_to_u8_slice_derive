extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;
extern crate proc_macro2;

use proc_macro::TokenStream;
use syn::{punctuated::Punctuated, token::Comma, DeriveInput, LitByteStr, Variant};

#[proc_macro_derive(EnumToU8)]
pub fn enum_to_u8(input: TokenStream) -> TokenStream {
    //let s = input.to_string();
    let ast = syn::parse_macro_input!(input as DeriveInput);

    let name = &ast.ident;
    if let syn::Data::Enum(data) = ast.data {
        impl_enum_to_u8(name, data.variants)
    } else {
        panic!("Only work for enum");
    }
}

fn impl_enum_to_u8(name: &syn::Ident, body: Punctuated<Variant, Comma>) -> TokenStream {
    let get_items: Vec<_> = body
        .iter()
        .map(|field| {
            let ident = field.ident.clone();
            let mut name_c_bytes = ident.to_string().as_bytes().to_vec();
            name_c_bytes.push(0);
            let lbs: LitByteStr = LitByteStr::new(&name_c_bytes, ident.span());
            quote!(
                #name::#ident => #lbs
            )
        })
        .collect();
    quote!(
        impl #name{
            fn enum_to_u8(&self) -> &'static [u8] {
                match *self{
                    #(#get_items),*
                }
            }
        }
    )
    .into()
}
