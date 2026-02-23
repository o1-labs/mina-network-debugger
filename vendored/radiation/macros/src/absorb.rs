// Copyright 2022 Vladislav Melnik
// SPDX-License-Identifier: MIT

use super::{find_attr, Tags};

pub fn derive(s: synstructure::Structure) -> proc_macro2::TokenStream {
    let (tags, tag_ty) = match Tags::new(&s) {
        Ok(v) => v,
        Err(err) => return err.into_compile_error(),
    };

    let se = quote::quote!(radiation);
    let mut body = quote::quote!();
    for t in tags {
        let (tag_val, variant) = match t {
            Ok(v) => v,
            Err(err) => return err.to_compile_error(),
        };

        let ctor = variant.construct(|_field, i| {
            format!("_{i}")
                .parse::<proc_macro2::TokenStream>()
                .expect("trivial code")
        });
        let parser = match variant.bindings().len() {
            0 => quote::quote! { #se::nom::combinator::success(#ctor) },
            len => {
                let mut p = quote::quote!();
                let mut limit_next = quote::quote!(L);
                for binding in variant.bindings() {
                    let ast = &binding.ast();
                    let limit =
                        extract_attr!(&ast.attrs, "limit").unwrap_or_else(|| limit_next.clone());
                    limit_next = quote::quote!(<#limit as #se::Limit>::Next);
                    let as_str = find_attr(&ast.attrs, "as_str").is_some();
                    let custom_absorb = extract_attr!(&ast.attrs, "custom_absorb");

                    if as_str {
                        p.extend(quote::quote!(
                            #se::nom::combinator::map_res(<&str>::absorb::<#limit>, str::parse),
                        ));
                    } else if let Some(absorb) = custom_absorb {
                        p.extend(quote::quote!(#absorb,));
                    } else {
                        p.extend(quote::quote!(#se::Absorb::absorb::<#limit>,));
                    };
                }

                let mut pat = quote::quote!();
                for i in 0..len {
                    let var = format!("_{i},")
                        .parse::<proc_macro2::TokenStream>()
                        .expect("trivial code");
                    pat.extend(var);
                }

                quote::quote! {
                    #se::nom::combinator::map(#se::nom::sequence::tuple((#p)), |(#pat)| #ctor)
                }
            }
        };

        body.extend(quote::quote! {
            if tag == #tag_val {
                #parser(input)
            } else
        })
    }

    let ident = &s.ast().ident;
    let gen_impl = quote! {
        gen impl<'pa> #se::Absorb<'pa> for @Self {
            fn absorb<L>(
                input: &'pa [u8],
            ) -> #se::nom::IResult<&'pa [u8], Self, #se::ParseError<&'pa [u8]>>
            where
                L: #se::Limit,
            {
                let original_input = <&[u8]>::clone(&input);
                let (input, tag) = #tag_ty::absorb::<()>(input)?;
                #body
                {
                    let kind = #se::ParseErrorKind::unknown_tag(tag, stringify!(#ident));
                    Err(kind.error(original_input))
                }
            }
        }
    };

    s.gen_impl(gen_impl)
}
