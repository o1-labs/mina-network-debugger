extern crate proc_macro;

use proc_macro2::Literal;
use syn::{
    Data, DeriveInput, Lit, parse_macro_input, Attribute, Ident, Error, Type, PathArguments, Expr,
};

#[proc_macro]
pub fn license(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let l = match parse_macro_input!(input as Lit) {
        Lit::Str(s) => s.value(),
        l => {
            return Error::new(l.span(), "Expected string literal")
                .to_compile_error()
                .into();
        },
    };
    let license = l + "\u{0}";
    let license_len = license.len();

    let license = Literal::byte_string(license.as_bytes());

    proc_macro::TokenStream::from(quote::quote! {
        #[no_mangle]
        #[link_section = "license"]
        static LICENSE: [u8; #license_len] = *#license;

        #[panic_handler]
        fn panic(_panic: &core::panic::PanicInfo<'_>) -> ! {
            loop {}
        }

        #[lang = "eh_personality"]
        extern "C" fn eh_personality() {}
    })
}

#[proc_macro_derive(BpfApp, attributes(license, hashmap, array_percpu, ringbuf, prog))]
pub fn derive_bpf_app(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let DeriveInput { data, .. } = parse_macro_input!(input as DeriveInput);

    struct KernelTokens<T> {
        decl: T,
        new_field: T,
    }

    let kt = KernelTokens {
        decl: quote::quote! {},
        new_field: quote::quote! {},
    };

    fn generic_const(ty: &Type) -> Result<impl Iterator<Item = &'_ Expr> + '_, Error> {
        match ty {
            &Type::Path(ref path) => {
                let e = Error::new_spanned(path, "not enough path segments");
                match &path.path.segments.last().ok_or(e)?.arguments {
                    &PathArguments::AngleBracketed(ref args) => {
                        let it = args.args.iter().filter_map(|arg| match arg {
                            syn::GenericArgument::Const(e) => Some(e),
                            _ => None,
                        });
                        Ok(it)
                    },
                    a => {
                        let msg = "expected angle bracketed argument \
                            like `<'a, T>` in `std::slice::iter<'a, T>`";
                        Err(Error::new_spanned(a, msg))
                    },
                }
            },
            ty => Err(Error::new_spanned(
                ty,
                "expected path like `std::slice::Iter`",
            )),
        }
    }

    fn process_attrib(
        attrs: &[Attribute],
        name_ident: &Ident,
        ty: &Type,
    ) -> Result<KernelTokens<impl quote::ToTokens>, Error> {
        struct AttributeRingbuf {
            size: usize,
        }

        impl syn::parse::Parse for AttributeRingbuf {
            fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
                let _: syn::Ident = input.parse()?;
                let _: syn::Token![=] = input.parse()?;
                let size = match input.parse()? {
                    Lit::Int(l) => l.base10_digits().parse().map_err(|e| {
                        syn::Error::new(l.span(), format_args!("Bad integer literal {}", e))
                    })?,
                    l => return Err(syn::Error::new(l.span(), "Expected integer literal")),
                };
                Ok(AttributeRingbuf { size })
            }
        }

        let attribute = attrs.first().unwrap();
        let segment = attribute
            .path
            .segments
            .first()
            .ok_or_else(|| Error::new_spanned(attribute, "expected one path component"))?;
        let ident_str = segment.ident.to_string();
        match ident_str.as_str() {
            "ringbuf" => {
                let AttributeRingbuf { size } = attribute.parse_args()?;

                Ok(KernelTokens {
                    decl: quote::quote! {
                        #[no_mangle]
                        #[link_section = ".maps"]
                        #[allow(non_upper_case_globals)]
                        static mut #name_ident: ebpf_kern::RingBuffer<#size> =
                            ebpf_kern::RingBuffer::new();
                    },
                    new_field: quote::quote! {
                        #name_ident: ebpf_kern::RingBufferRef::new(&mut #name_ident),
                    },
                })
            },
            "array_percpu" => {
                let AttributeRingbuf { size } = attribute.parse_args()?;
                let mut it = generic_const(ty)?;
                let e = || Error::new_spanned(ty, "expected one generic constant arguments");
                let value_size = it.next().ok_or_else(e)?;

                Ok(KernelTokens {
                    decl: quote::quote! {
                        #[no_mangle]
                        #[link_section = ".maps"]
                        #[allow(non_upper_case_globals)]
                        static mut #name_ident: ebpf_kern::ArrayPerCpu<#value_size, #size> =
                            ebpf_kern::ArrayPerCpu::new();
                    },
                    new_field: quote::quote! {
                        #name_ident: ebpf_kern::ArrayPerCpuRef::new(&mut #name_ident),
                    },
                })
            },
            "hashmap" => {
                let AttributeRingbuf { size } = attribute.parse_args()?;
                let mut it = generic_const(ty)?;
                let e = || Error::new_spanned(ty, "expected two generic constant arguments");
                let key_size = it.next().ok_or_else(e)?;
                let value_size = it.next().ok_or_else(e)?;

                Ok(KernelTokens {
                    decl: quote::quote! {
                        #[no_mangle]
                        #[link_section = ".maps"]
                        #[allow(non_upper_case_globals)]
                        static mut #name_ident: ebpf_kern::HashMap<#key_size, #value_size, #size> =
                            ebpf_kern::HashMap::new();
                    },
                    new_field: quote::quote! {
                        #name_ident: ebpf_kern::HashMapRef::new(&mut #name_ident),
                    },
                })
            },
            "prog" => {
                if let Lit::Str(l) = attribute.parse_args()? {
                    Ok(KernelTokens {
                        decl: if l.value() == "xdp" {
                            quote::quote! {
                                #[no_mangle]
                                #[link_section = #l]
                                fn #name_ident(
                                    ctx: *const core::ffi::c_void,
                                ) -> core::ffi::c_int {
                                    let mut app = unsafe { app_instance() };
                                    let ctx = unsafe { ebpf_kern::xdp::Context::cast(ctx) };
                                    match app.#name_ident(ctx) {
                                        Ok(v) => v as _,
                                        Err(c) => ebpf_kern::xdp::Action::Aborted as _,
                                    }
                                }
                            }
                        } else {
                            quote::quote! {
                                #[no_mangle]
                                #[link_section = #l]
                                fn #name_ident(
                                    ctx: *const core::ffi::c_void,
                                ) -> core::ffi::c_int {
                                    let mut app = unsafe { app_instance() };
                                    let ctx = unsafe { ebpf_kern::Context::cast(ctx) };
                                    match app.#name_ident(ctx) {
                                        Ok(()) => 0,
                                        Err(c) => c,
                                    }
                                }
                            }
                        },
                        new_field: quote::quote! {
                            #name_ident: ebpf_kern::ProgRef::new(#name_ident),
                        },
                    })
                } else {
                    Err(Error::new_spanned(
                        &attribute.tokens,
                        "expected string literal",
                    ))
                }
            },
            _ => Ok(KernelTokens {
                decl: quote::quote! {},
                new_field: quote::quote! {},
            }),
        }
    }

    let kt: Result<_, Error> = match data {
        Data::Struct(data) => data.fields.into_iter().try_fold(kt, |kt, field| {
            let val = field.ident.unwrap();
            let ty = field.ty;

            let KernelTokens { decl, new_field } = process_attrib(&field.attrs, &val, &ty)?;
            let KernelTokens {
                decl: decl_a,
                new_field: new_field_a,
            } = kt;
            Ok(KernelTokens {
                decl: quote::quote! {
                    #decl_a #decl
                },
                new_field: quote::quote! {
                    #new_field_a #new_field
                },
            })
        }),
        _ => unimplemented!(),
    };

    let kt = match kt {
        Ok(kt) => kt,
        Err(e) => return e.to_compile_error().into(),
    };

    let KernelTokens { decl, new_field } = kt;

    proc_macro::TokenStream::from(quote::quote! {
        unsafe fn app_instance() -> App {
            #decl

            App {
                #new_field
            }
        }
    })
}
