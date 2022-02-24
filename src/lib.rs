use self::{enum_::derive_arbitrary_for_enum, struct_::derive_arbitrary_for_struct};
use proc_macro2::Ident;
use quote::{format_ident, ToTokens};
use syn::{
    parse_macro_input, parse_quote, Data, DataEnum, DataStruct, DeriveInput, Expr, ExprCall,
    Fields, ItemFn, ItemImpl, Variant,
};
use thiserror::Error;

#[proc_macro_derive(Arbitrary)]
pub fn derive_arbitrary(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: DeriveInput = parse_macro_input!(input);
    let derive = match input.data {
        Data::Struct(st) => derive_arbitrary_for_struct(&input.ident, &st),
        Data::Enum(en) => derive_arbitrary_for_enum(&input.ident, &en),
        _ => todo!(),
    };
    let output = derive
        .map(ToTokens::into_token_stream)
        .unwrap_or_else(|x| x.as_ref().to_compile_error());
    proc_macro::TokenStream::from(output)
}

#[inline]
fn gen_arbitrary_instance(name: &Ident, func: &ItemFn) -> ItemImpl {
    parse_quote! {
        impl ::quickcheck::Arbitrary for #name {
            #func
        }
    }
}

fn gen_ctor(name: &Ident, fields: &Fields) -> Expr {
    match fields {
        Fields::Named(fs) => {
            let (fields_names, fields_generators): (Vec<_>, Vec<ExprCall>) = fs
                .named
                .iter()
                .map(|x| {
                    let name = x.ident.as_ref();
                    (name, parse_quote! {gen(g)})
                })
                .unzip();
            parse_quote! {
                #name {
                    #(#fields_names: #fields_generators,)*
                }
            }
        }
        Fields::Unnamed(fs) => {
            let fields_generators: Vec<ExprCall> = fs
                .unnamed
                .iter()
                .map(|_| {
                    parse_quote! {gen(g)}
                })
                .collect();
            parse_quote! {
                #name(#(#fields_generators,)*)
            }
        }
        Fields::Unit => parse_quote! {
            #name
        },
    }
}

mod struct_ {
    use super::*;

    pub(crate) fn derive_arbitrary_for_struct(
        name: &Ident,
        data: &DataStruct,
    ) -> Result<ItemImpl, Error> {
        let ctor = gen_ctor(name, &data.fields);
        let func: ItemFn = parse_quote! {
            fn arbitrary(g: &mut ::quickcheck::Gen) -> Self {
                use ::quickcheck::Arbitrary;
                fn gen<T: Arbitrary>(g: &mut ::quickcheck::Gen) -> T {
                    T::arbitrary(g)
                }
                #ctor
            }
        };
        let inst = gen_arbitrary_instance(name, &func);
        Ok(inst)
    }
}

mod enum_ {
    use super::*;
    use heck::AsSnakeCase;

    pub(crate) fn derive_arbitrary_for_enum(
        name: &Ident,
        data: &DataEnum,
    ) -> Result<ItemImpl, Error> {
        let factories: Vec<_> = data
            .variants
            .iter()
            .map(|x| gen_factory(&name, x))
            .collect();
        let factories_list: Vec<_> = { factories.iter().map(|x| &x.sig.ident) }.collect();
        let func: ItemFn = parse_quote! {
            fn arbitrary(g: &mut ::quickcheck::Gen) -> Self {
                use ::quickcheck::Arbitrary;
                fn gen<T: Arbitrary>(g: &mut ::quickcheck::Gen) -> T {
                    T::arbitrary(g)
                }
                #(#factories)*
                let fs = [#(#factories_list,)*];
                let f = g.choose(&fs).unwrap();
                f(g)
            }
        };
        let inst = gen_arbitrary_instance(name, &func);
        Ok(inst)
    }

    fn gen_factory(enum_name: &Ident, variant: &Variant) -> ItemFn {
        let variant_name = &variant.ident;
        let fn_name = format_ident!("gen_{}", AsSnakeCase(variant_name.to_string()).to_string());
        match &variant.fields {
            Fields::Named(fs) => {
                let (fields_names, fields_generators): (Vec<_>, Vec<ExprCall>) = fs
                    .named
                    .iter()
                    .map(|x| {
                        let name = x.ident.as_ref();
                        let ty = &x.ty;
                        (name, parse_quote! {#ty::arbitrary(g)})
                    })
                    .unzip();
                parse_quote! {
                    fn #fn_name(g: &mut ::quickcheck::Gen) -> #enum_name {
                        #enum_name::#variant_name {
                            #(#fields_names: #fields_generators,)*
                        }
                    }
                }
            }
            Fields::Unnamed(fs) => {
                let fields_generators: Vec<ExprCall> = fs
                    .unnamed
                    .iter()
                    .map(|x| {
                        let ty = &x.ty;
                        parse_quote! {#ty::arbitrary(g)}
                    })
                    .collect();
                parse_quote! {
                    fn #fn_name(g: &mut ::quickcheck::Gen) -> #enum_name {
                        #enum_name::#variant_name(#(#fields_generators,)*)
                    }
                }
            }
            Fields::Unit => parse_quote! {
                fn #fn_name(g: &mut ::quickcheck::Gen) -> #enum_name {
                    #enum_name::#variant_name
                }
            },
        }
    }
}

#[derive(Error, Debug)]
#[error(transparent)]
struct Error(#[from] syn::Error);

impl AsRef<syn::Error> for Error {
    fn as_ref(&self) -> &syn::Error {
        &self.0
    }
}
