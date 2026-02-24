// Copyright 2022 Vladislav Melnik
// SPDX-License-Identifier: MIT

use super::find_attr;

pub fn derive(s: synstructure::Structure) -> proc_macro2::TokenStream {
    use syn::{
        ext::IdentExt,
        parse::{Parse, ParseStream},
        spanned::Spanned,
        Ident, Result, Token,
    };

    struct Limit {
        inner: syn::Type,
        next: syn::Type,
        lower: syn::Expr,
        upper: syn::Expr,
        description: Option<syn::Expr>,
    }

    impl Default for Limit {
        fn default() -> Self {
            Limit {
                inner: syn::parse("()".parse().expect("trivial")).expect("trivial"),
                next: syn::parse("()".parse().expect("trivial")).expect("trivial"),
                lower: syn::parse("0".parse().expect("trivial")).expect("trivial"),
                upper: syn::parse("usize::MAX".parse().expect("trivial")).expect("trivial"),
                description: None,
            }
        }
    }

    impl Parse for Limit {
        fn parse(input: ParseStream) -> Result<Self> {
            let mut s = Self::default();

            loop {
                if input.peek(Ident::peek_any) {
                    let ident = input.parse::<Ident>()?.to_string();
                    let _ = input.parse::<Token![=]>()?;
                    match ident.as_str() {
                        "inner" => s.inner = input.parse()?,
                        "next" => s.next = input.parse()?,
                        "lower" => s.lower = input.parse()?,
                        "upper" => s.upper = input.parse()?,
                        "description" => s.description = Some(input.parse()?),
                        ident => {
                            let msg = format!("unexpected key: {ident}");
                            return Err(syn::Error::new(ident.span(), msg));
                        }
                    }
                    if !input.peek(Token![,]) {
                        break;
                    } else {
                        input.parse::<Token![,]>()?;
                    }
                } else {
                    break;
                }
            }

            Ok(s)
        }
    }

    let Limit {
        inner,
        next,
        lower,
        upper,
        description,
    } = if let Some(limit) = find_attr(&s.ast().attrs, "limit") {
        match limit.parse_args::<Limit>() {
            Ok(v) => v,
            Err(err) => return err.to_compile_error(),
        }
    } else {
        Limit::default()
    };
    let description = description.map(|d| quote::quote!(#d)).unwrap_or_else(|| {
        let ident = &s.ast().ident;
        quote::quote!(stringify!(#ident))
    });

    let gen_impl = quote::quote! {
        gen impl Limit for @Self {
            type Inner = #inner;

            type Next = #next;

            const LOWER: usize = #lower;

            const UPPER: usize = #upper;

            const DESCRIPTION: &'static str = #description;
        }
    };

    s.gen_impl(gen_impl)
}
