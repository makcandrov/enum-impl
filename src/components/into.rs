use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::Fields;

use crate::attr::ClassicAttribute;

pub fn expand_into(
    enum_ident: &Ident,
    variant_ident: &Ident,
    variant_name_snake_case: &str,
    params: &ClassicAttribute,
    fields: &Fields,
) -> TokenStream {
    let function_name = params.rename.clone().unwrap_or(Ident::new(
        &format!("into_{variant_name_snake_case}"),
        Span::call_site(),
    ));

    let (ty, destruct, ret) = match fields {
        Fields::Named(named_fields) => {
            let len = named_fields.named.len();

            let mut ty = TokenStream::default();
            let mut destruct = TokenStream::default();
            let mut ret = TokenStream::default();

            let mut i = 0;

            for field in &named_fields.named {
                let var_ident = field.ident.as_ref().unwrap();

                let field_ty = &field.ty;

                if i == len - 1 {
                    ty.extend(quote! { #field_ty });
                    destruct.extend(quote! { #var_ident });
                    ret.extend(quote! { #var_ident });
                } else {
                    ty.extend(quote! { #field_ty, });
                    destruct.extend(quote! { #var_ident, });
                    ret.extend(quote! { #var_ident, });

                    i += 1;
                }
            }

            (
                if len == 1 {
                    quote! { #ty }
                } else {
                    quote! { ( #ty ) }
                },
                quote! { { #destruct } },
                if len == 1 {
                    quote! { #ret }
                } else {
                    quote! { ( #ret ) }
                },
            )
        },
        Fields::Unnamed(unnamed_fields) => {
            let len = unnamed_fields.unnamed.len();

            let mut ty = TokenStream::default();
            let mut destruct = TokenStream::default();
            let mut ret = TokenStream::default();

            let mut i = 0;

            for field in &unnamed_fields.unnamed {
                assert!(field.ident.is_none());

                let var_ident = Ident::new(&format!("arg{i}"), Span::call_site());

                let field_ty = &field.ty;

                if i == len - 1 {
                    ty.extend(quote! { #field_ty });
                    destruct.extend(quote! { #var_ident });
                    ret.extend(quote! { #var_ident });
                } else {
                    ty.extend(quote! { #field_ty, });
                    destruct.extend(quote! { #var_ident, });
                    ret.extend(quote! { #var_ident, });

                    i += 1;
                }
            }

            (
                if len == 1 {
                    quote! { #ty }
                } else {
                    quote! { ( #ty ) }
                },
                quote! { ( #destruct ) },
                if len == 1 {
                    quote! { #ret }
                } else {
                    quote! { ( #ret ) }
                },
            )
        },
        Fields::Unit => (quote! { () }, quote! {}, quote! { () }),
    };

    let keyword = if params.public {
        quote! { pub }
    } else {
        quote! {}
    };

    let documentation = format!(
        "Converts into the associated data if it is the [`{}::{}`] variant. Otherwise, returns `None`.",
        enum_ident, variant_ident
    );

    quote! {
        #[doc = #documentation]
        #keyword fn #function_name(self) -> Option<#ty> {
            match self {
                Self::#variant_ident #destruct => Some(#ret),
                _ => None,
            }
        }
    }
}
