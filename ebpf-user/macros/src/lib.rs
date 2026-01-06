use syn::{Data, DeriveInput, parse_macro_input};

#[proc_macro_derive(BpfApp, attributes(license, hashmap, array_percpu, ringbuf, prog))]
pub fn derive_bpf_app(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input as DeriveInput);

    struct UserTokens<T> {
        map_cnt: T,
        prog_cnt: T,
        new_field: T,
        map_step: T,
        prog_step: T,
    }

    let ut = UserTokens {
        map_cnt: quote::quote! {},
        prog_cnt: quote::quote! {},
        new_field: quote::quote! {},
        map_step: quote::quote! {},
        prog_step: quote::quote! {},
    };

    let ut = match data {
        Data::Struct(data) => data.fields.into_iter().fold(ut, |ut, field| {
            let UserTokens {
                map_cnt,
                prog_cnt,
                new_field,
                map_step,
                prog_step,
            } = ut;
            let val = field.ident.unwrap();
            let val_str = format!("{}\0", val);
            let ty = field.ty;
            UserTokens {
                map_cnt: quote::quote! { #map_cnt + <#ty as ebpf_user::kind::AppItem>::MAP },
                prog_cnt: quote::quote! { #prog_cnt + <#ty as ebpf_user::kind::AppItem>::PROG },
                new_field: quote::quote! { #new_field #val: <#ty>::named(#val_str), },
                map_step: quote::quote! {
                    #map_step
                    if let ebpf_user::kind::AppItemKindMut::Map(v) = self.#val.kind_mut() {
                        if counter == index {
                            return Some(v);
                        } else {
                            counter += 1;
                        }
                    }
                },
                prog_step: quote::quote! {
                    #prog_step
                    if let ebpf_user::kind::AppItemKindMut::Prog(v) = self.#val.kind_mut() {
                        if counter == index {
                            return Some(v);
                        } else {
                            counter += 1;
                        }
                    }
                },
            }
        }),
        _ => unimplemented!(),
    };

    let UserTokens {
        map_cnt,
        prog_cnt,
        new_field,
        map_step,
        prog_step,
    } = ut;

    proc_macro::TokenStream::from(quote::quote! {
        impl ebpf_user::BpfApp for #ident {
            const MAP_CNT: usize = 0 #map_cnt;
            const PROG_CNT: usize = 0 #prog_cnt;

            fn instance() -> Self {
                use ebpf_user::kind::AppItem;

                #ident {
                    #new_field
                }
            }

            fn as_mut_map(&mut self, index: usize) -> Option<&mut ebpf_user::MapRef> {
                use ebpf_user::kind::AppItem;

                let mut counter = 0;
                #map_step
                let _ = counter;
                None
            }

            fn as_mut_prog(&mut self, index: usize) -> Option<&mut ebpf_user::ProgRef> {
                use ebpf_user::kind::AppItem;

                let mut counter = 0;
                #prog_step
                let _ = counter;
                None
            }
        }
    })
}
