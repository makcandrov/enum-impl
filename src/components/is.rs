use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::Fields;

use crate::attr::ClassicAttribute;

pub fn expand_is(
    enum_ident: &Ident,
    variant_ident: &Ident,
    variant_name_snake_case: &str,
    params: &ClassicAttribute,
    fields: &Fields,
) -> TokenStream {
    let function_name = params
        .rename
        .clone()
        .unwrap_or(Ident::new(&format!("is_{variant_name_snake_case}"), Span::call_site()));

    let destruct = match fields {
        Fields::Named(_) => quote! { { .. } },
        Fields::Unnamed(_) => quote! { ( .. ) },
        Fields::Unit => quote! {},
    };

    let keyword = if params.public {
        quote! { pub }
    } else {
        quote! {}
    };

    let documentation = format!(
        "Returns `true` if it is the [`{}::{}`] variant. Otherwise, returns `false`.",
        enum_ident, variant_ident
    );

    quote! {
        #[doc = #documentation]
        #keyword fn #function_name(&self) -> bool {
            match self {
                Self::#variant_ident #destruct => true,
                _ => false,
            }
        }
    }
}
