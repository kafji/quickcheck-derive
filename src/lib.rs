use self::{enum_::derive_arbitrary_for_enum, struct_::derive_arbitrary_for_struct};
use proc_macro2::{Ident, Span};
use quote::{format_ident, ToTokens};
use syn::{
    parse_macro_input, parse_quote, Attribute, Data, DataEnum, DataStruct, DeriveInput, Expr,
    ExprAssign, ExprCall, ExprLit, ExprPath, Field, Fields, ItemFn, ItemImpl, Lit, Path, Variant,
};
use thiserror::Error;

#[proc_macro_derive(Arbitrary, attributes(arbitrary))]
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

fn gen_ctor(name: &Path, fields: &Fields) -> Result<Expr, Error> {
    let ctor = match fields {
        Fields::Named(fs) => {
            let (fields_names, fields_generators): (Vec<_>, Vec<ExprCall>) = fs
                .named
                .iter()
                .map(|field| {
                    let generator = gen_generator(field)?;
                    Result::<_, Error>::Ok((field.ident.as_ref(), generator))
                })
                .collect::<Result<Vec<_>, _>>()?
                .into_iter()
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
                .map(|field| gen_generator(field))
                .collect::<Result<_, _>>()?;
            parse_quote! {
                #name(#(#fields_generators,)*)
            }
        }
        Fields::Unit => parse_quote! {
            #name
        },
    };
    Ok(ctor)
}

fn gen_generator(field: &Field) -> Result<ExprCall, Error> {
    let generator = field
        .attrs
        .iter()
        .filter(|&x| {
            x.path
                .segments
                .first()
                .map(|path| &path.ident)
                .map(|x| *x == "arbitrary")
                .unwrap_or_default()
        })
        .filter(|&x| x.parse_args::<ExprAssign>().is_ok())
        .map(|x| extract_generator_ident(x))
        .next()
        .transpose()?
        .flatten()
        .unwrap_or_else(|| Ident::new("gen", Span::call_site()));
    Ok(parse_quote! {#generator(g)})
}

fn extract_generator_ident(attr: &Attribute) -> Result<Option<Ident>, Error> {
    let expr = attr.parse_args::<ExprAssign>()?;
    let ident =
        if matches!(*expr.left, Expr::Path(ExprPath{path,..}) if path.is_ident("generator")) {
            if let Expr::Lit(ExprLit {
                lit: Lit::Str(x), ..
            }) = *expr.right
            {
                Some(x.value())
            } else {
                None
            }
        } else {
            None
        }
        .map(|x| format_ident!("{}", x));
    Ok(ident)
}

mod struct_ {
    use super::*;

    pub(crate) fn derive_arbitrary_for_struct(
        name: &Ident,
        data: &DataStruct,
    ) -> Result<ItemImpl, Error> {
        let path: Path = parse_quote! {#name};
        let ctor = gen_ctor(&path, &data.fields)?;
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
        let fields = &variant.fields;
        let name: Path = parse_quote! { #enum_name::#variant_name };
        let ctor = gen_ctor(&name, fields).unwrap();
        parse_quote! {
            fn #fn_name(g: &mut ::quickcheck::Gen) -> #enum_name {
                #ctor
            }
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
