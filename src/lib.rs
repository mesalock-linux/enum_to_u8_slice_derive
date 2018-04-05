extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(EnumToU8)]
pub fn enum_to_u8(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_derive_input(&s).unwrap();

    let name = &ast.ident;
    if let syn::Body::Enum(body) = ast.body {
        let gen = impl_enum_to_u8(name, body);
        gen.parse().unwrap()
    } else {
        panic!("Only work for enum");
    }
}

fn impl_enum_to_u8(name: &syn::Ident, body: Vec<syn::Variant>) -> quote::Tokens {
    let content = build_content(name, body);
    quote!(
        impl #name{
            fn enum_to_u8(&self) -> &'static [u8] {
                match *self{
                    #content
                }
            }
        }
    )
}

fn build_content(name: &syn::Ident, body: Vec<syn::Variant>) -> syn::Ident {
    body.iter()
        .map(|field| format!("{enum_name}::{field} => b\"{field}\0\"",
                             field = field.ident,
                             enum_name = name))
        .collect::<Vec<String>>()
        .join(",")
        .into()
}
