// Copyright 2022 Vladislav Melnik
// SPDX-License-Identifier: MIT

#![forbid(unsafe_code)]
#![deny(clippy::all)]

#[macro_use]
extern crate synstructure;

fn find_attr(attrs: &[syn::Attribute], name: &str) -> Option<syn::Attribute> {
    attrs.iter().find(|a| a.path().is_ident(name)).cloned()
}

macro_rules! extract_attr {
    ($attrs:expr, $name:expr) => {
        match find_attr($attrs, $name) {
            Some(attr) => match attr.parse_args::<proc_macro2::TokenStream>() {
                Ok(v) => Some(v),
                Err(err) => return err.into_compile_error(),
            },
            None => None,
        }
    };
}

struct Tags<'a> {
    structure: &'a synstructure::Structure<'a>,
    current: proc_macro2::TokenStream,
    pos: usize,
}

impl<'a> Tags<'a> {
    fn new(
        structure: &'a synstructure::Structure<'a>,
    ) -> Result<(Self, proc_macro2::TokenStream), syn::Error> {
        let default_ty = if structure.variants().len() == 1 {
            quote::quote!(<()>)
        } else {
            quote::quote!(u16)
        };
        let ty = match find_attr(&structure.ast().attrs, "tag") {
            Some(attr) => attr.parse_args::<proc_macro2::TokenStream>()?,
            None => default_ty,
        };

        let current = quote::quote!(#ty::default());

        Ok((
            Tags {
                structure,
                current,
                pos: 0,
            },
            ty,
        ))
    }
}

impl<'a> Iterator for Tags<'a> {
    type Item = Result<(proc_macro2::TokenStream, &'a synstructure::VariantInfo<'a>), syn::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos == self.structure.variants().len() {
            None
        } else {
            let variant = &self.structure.variants()[self.pos];
            self.pos += 1;
            let tag_val = match find_attr(variant.ast().attrs, "tag") {
                Some(attr) => match attr.parse_args::<proc_macro2::TokenStream>() {
                    Ok(v) => v,
                    Err(err) => return Some(Err(err)),
                },
                None => self.current.clone(),
            };
            self.current = quote::quote!(#tag_val + 1);

            Some(Ok((tag_val, variant)))
        }
    }
}

mod absorb;
decl_derive!([Absorb, attributes(custom_absorb, as_str, limit, tag)] => absorb::derive);

mod emit;
decl_derive!([Emit, attributes(custom_emit, as_str, tag)] => emit::derive);

mod limit;
decl_derive!([Limit, attributes(limit)] => limit::derive);
