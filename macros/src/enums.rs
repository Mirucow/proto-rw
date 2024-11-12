use proc_macro2::TokenStream;
use quote::quote;
use syn::{DataEnum, Type, Visibility};

use crate::util::extract_generic_type;

pub fn build_enum(
    ty: Type,
    vis: &Visibility,
    ident: &proc_macro2::Ident,
    data: &DataEnum,
) -> (TokenStream, TokenStream, TokenStream) {
    let ty_path = match ty {
        Type::Path(ty_path) => ty_path,
        _ => panic!("Enum type must be a path"),
    };

    let segment = ty_path
        .path
        .segments
        .first()
        .cloned()
        .expect("No segments found in type path");
    let seg_ident = segment.ident.clone();

    let (read_func, write_func) = match seg_ident.to_string().as_str() {
        "u8" => (
            quote! { buf.read_proto::<u8>()? },
            quote! { buf.write_proto(value as u8)? },
        ),
        "LE" | "BE" | "Var" => {
            let gen_type = extract_generic_type(&segment)
                .unwrap_or_else(|| panic!("No generic type found for {}", seg_ident));
            (
                quote! { buf.read_proto::<#seg_ident<#gen_type>>()?.into() },
                quote! { #seg_ident::<#gen_type>(value).write(buf)?; },
            )
        }
        _ => panic!("Enum type must be a LE, BE, Var, or u8"),
    };

    let mut new_variants = vec![];
    let mut read_funcs = vec![];
    let mut write_funcs = vec![];

    for v in &data.variants {
        let v_ident = &v.ident;
        let v_value = v.discriminant.clone().expect("No variant value found").1;

        new_variants.push(quote! { #v_ident, });
        read_funcs.push(quote! { #v_value => #ident::#v_ident, });
        write_funcs.push(quote! { #ident::#v_ident => #v_value, });
    }

    (
        quote! {
            #vis enum #ident {
                #(#new_variants)*
            }
        },
        quote! {
            {
                let value = #read_func;
                match value {
                    #(#read_funcs)*
                    _ => Err(proto_rw::error::ProtoRwError::Error(format!(
                        "Get {} while reading {}",
                        value,
                        stringify!(#ident)
                    )))?,
                }
            }
        },
        quote! {
            let value = match self {
                #(#write_funcs)*
            };
            #write_func
        },
    )
}
