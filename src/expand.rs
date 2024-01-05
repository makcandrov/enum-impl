use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput};

use crate::attr::{EnumImplAttributes, ImplOrClassicAttribute};
use crate::components::{
    expand_as_ref,
    expand_as_ref_mut,
    expand_from_foreign,
    expand_from_local,
    expand_into,
    expand_is,
};

pub fn derive(input: &DeriveInput) -> TokenStream {
    match try_expand(input) {
        Ok(expanded) => expanded,
        Err(err) => {
            let error = err.to_compile_error();
            quote! {
                #error
            }
        },
    }
}

fn try_expand(input: &DeriveInput) -> syn::Result<TokenStream> {
    let enum_ident = &input.ident;
    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let Data::Enum(data_enum) = &input.data else {
        return Err(syn::Error::new_spanned(input, "only enums are supported"));
    };

    let mut expanded = TokenStream::default();
    let mut foreign_impls = TokenStream::default();

    for variant in &data_enum.variants {
        let variant_ident = &variant.ident;
        let variant_name_snake_case = variant_ident.to_string().to_case(Case::Snake);

        let attributes = EnumImplAttributes::new(&variant.attrs)?;

        let fields = &variant.fields;

        if let Some(params) = &attributes.as_ref_mut {
            expanded.extend(expand_as_ref_mut(
                variant_ident,
                &variant_name_snake_case,
                params,
                fields,
            ));
        }
        if let Some(params) = &attributes.as_ref {
            expanded.extend(expand_as_ref(variant_ident, &variant_name_snake_case, params, fields));
        }
        if let Some(params) = &attributes.from {
            match params {
                ImplOrClassicAttribute::Classic(params) => expanded.extend(expand_from_local(
                    variant_ident,
                    &variant_name_snake_case,
                    params,
                    fields,
                )),
                ImplOrClassicAttribute::Impl => foreign_impls.extend(expand_from_foreign(input, variant_ident, fields)),
            }
        }
        if let Some(params) = &attributes.into {
            expanded.extend(expand_into(variant_ident, &variant_name_snake_case, params, fields));
        }
        if let Some(params) = &attributes.is {
            expanded.extend(expand_is(variant_ident, &variant_name_snake_case, params, fields));
        }
    }

    expanded = quote! {
        impl #impl_generics #enum_ident #ty_generics #where_clause {
            #expanded
        }

        #foreign_impls
    };

    Ok(expanded)
}
