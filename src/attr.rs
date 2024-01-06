use proc_macro2::{Ident, Span};
use syn::meta::ParseNestedMeta;
use syn::Token;

#[derive(Debug, Clone, Default)]
pub struct EnumImplAttributes {
    pub as_ref_mut: Option<ClassicAttribute>,
    pub as_ref: Option<ClassicAttribute>,
    pub from: Option<ImplOrClassicAttribute>,
    pub into: Option<ClassicAttribute>,
    pub is: Option<ClassicAttribute>,
}

#[derive(Debug, Clone)]
pub struct ClassicAttribute {
    pub public: bool,
    pub rename: Option<Ident>,
}

#[derive(Debug, Clone)]
pub enum ImplOrClassicAttribute {
    Classic(ClassicAttribute),
    Impl,
}

impl ClassicAttribute {
    fn from_decoded(decoded: ParametrizedAttribute) -> syn::Result<Self> {
        match decoded.keyword {
            Keyword::None => Ok(Self {
                public: false,
                rename: decoded.param,
            }),
            Keyword::Pub => Ok(Self {
                public: true,
                rename: decoded.param,
            }),
            Keyword::Impl => Err(syn::Error::new_spanned(
                decoded.name,
                "invalid keyword `impl` for this attribute",
            )),
        }
    }
}

impl ImplOrClassicAttribute {
    fn from_decoded(decoded: ParametrizedAttribute) -> syn::Result<Self> {
        match decoded.keyword {
            Keyword::Impl => {
                if let Some(rename) = decoded.param {
                    Err(syn::Error::new_spanned(rename, "impl attributes cannot be renamed"))
                } else {
                    Ok(Self::Impl)
                }
            },
            _ => Ok(Self::Classic(ClassicAttribute::from_decoded(decoded)?)),
        }
    }
}

impl EnumImplAttributes {
    pub fn new(attrs: &Vec<syn::Attribute>) -> syn::Result<Self> {
        let mut res = Self::default();

        for attr in attrs {
            if !attr.path().is_ident("enum_impl") {
                continue;
            }

            if let syn::Meta::List(meta) = &attr.meta {
                if meta.tokens.is_empty() {
                    continue;
                }
            }

            attr.parse_nested_meta(|meta| {
                let decoded = ParametrizedAttribute::new(&meta)?;

                res.add(attr, decoded)?;

                Ok(())
            })?;
        }

        Ok(res)
    }

    fn add(&mut self, attr: &syn::Attribute, decoded: ParametrizedAttribute) -> syn::Result<()> {
        if match decoded.name.to_string().as_str() {
            "as_ref_mut" => self
                .as_ref_mut
                .replace(ClassicAttribute::from_decoded(decoded)?)
                .is_some(),
            "as_ref" => self.as_ref.replace(ClassicAttribute::from_decoded(decoded)?).is_some(),
            "from" => self
                .from
                .replace(ImplOrClassicAttribute::from_decoded(decoded)?)
                .is_some(),
            "into" => self.into.replace(ClassicAttribute::from_decoded(decoded)?).is_some(),
            "is" => self.is.replace(ClassicAttribute::from_decoded(decoded)?).is_some(),
            _ => return Err(syn::Error::new_spanned(decoded.name, "invalid enum_impl attribute")),
        } {
            return Err(syn::Error::new_spanned(attr, "duplicated attribute"));
        }

        Ok(())
    }
}

#[derive(Clone, Copy, Default, Debug)]
enum Keyword {
    #[default]
    None,
    Pub,
    Impl,
}

#[derive(Debug, Clone)]
struct ParametrizedAttribute {
    keyword: Keyword,
    name: Ident,
    param: Option<Ident>,
}

impl<'a> ParametrizedAttribute {
    pub fn new(meta: &'a ParseNestedMeta) -> syn::Result<ParametrizedAttribute> {
        let ident = meta.path.get_ident().unwrap().clone();

        let (keyword, name) = if meta.path.is_ident("pub") {
            (Keyword::Pub, meta.input.parse()?)
        } else if meta.path.is_ident("impl") {
            (Keyword::Impl, meta.input.parse()?)
        } else {
            (Keyword::None, ident)
        };

        let lookahead = meta.input.lookahead1();

        let param = if lookahead.peek(Token![=]) {
            meta.input.parse::<Token![=]>().unwrap();
            let lit = meta.input.parse::<syn::LitStr>()?;
            Some(Ident::new(&lit.value(), Span::call_site()))
        } else {
            None
        };

        Ok(Self { keyword, name, param })
    }
}
