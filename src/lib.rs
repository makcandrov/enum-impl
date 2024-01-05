use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod attr;
mod components;
mod expand;

#[proc_macro_derive(EnumImpl, attributes(enum_impl))]
pub fn derive_enum_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand::derive(&input).into()
}
