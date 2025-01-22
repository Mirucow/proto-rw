use std::collections::VecDeque;

use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{DataEnum, Expr, Type, Visibility};

use crate::util::{build_type_quotes, check_meta, extract_generic_type, get_attrs_meta};

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

    let (read_value_func, write_value_func) = match seg_ident.to_string().as_str() {
        "u8" => (
            quote! { u8::read_proto(buf)? },
            quote! { u8::write_proto(&(value as u8), buf)?; },
        ),
        "LE" | "BE" | "Var" => {
            let gen_type = extract_generic_type(&segment)
                .unwrap_or_else(|| panic!("No generic type found for {}", seg_ident));
            (
                quote! { #seg_ident::<#gen_type>::read_proto(buf)?.0 },
                quote! { #seg_ident::<#gen_type>(value).write_proto(buf)?; },
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
        let v_fields = &v.fields;

        let (new_declare, read_func, write_func) = match v_fields {
            syn::Fields::Named(fields) => {
                handle_named_fields(v_ident, v_value, fields, &write_value_func)
            }
            syn::Fields::Unnamed(fields) => {
                handle_unnamed_fields(v_ident, v_value, fields, &write_value_func)
            }
            syn::Fields::Unit => handle_unit(v_ident, v_value, &write_value_func),
        };

        new_variants.push(new_declare);
        read_funcs.push(read_func);
        write_funcs.push(write_func);
    }

    (
        quote! {
            #vis enum #ident {
                #(#new_variants)*
            }
        },
        quote! {
            {
                let value = #read_value_func;
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
            match self {
                #(#write_funcs)*
            }
        },
    )
}

fn handle_named_fields(
    ident: &proc_macro2::Ident,
    value: Expr,
    fields: &syn::FieldsNamed,
    write_value_func: &TokenStream,
) -> (TokenStream, TokenStream, TokenStream) {
    let mut new_fields = vec![];
    let mut read_funcs = vec![];
    let mut write_funcs = vec![];
    let mut idents = vec![];

    for f in &fields.named {
        let f_ident = f.ident.clone().expect("No field ident found");
        let metas = get_attrs_meta(&f.attrs);
        let mut props = (VecDeque::new(), VecDeque::new());
        metas.iter().for_each(|meta| {
            check_meta(&mut props, meta);
        });

        let ty = &f.ty;
        let (new_field, read_func, write_func) = build_type_quotes(ty, &mut props, None);

        new_fields.push(quote! { #f_ident: #new_field, });
        read_funcs.push(quote! { #f_ident: #read_func, });
        write_funcs.push(quote! {
            let value = #f_ident;
            #write_func;
        });
        idents.push(f_ident);
    }

    (
        quote! { #ident { #(#new_fields)* }, },
        quote! { #value => Self::#ident { #(#read_funcs)* }, },
        quote! { Self::#ident { #(#idents),* } => {
                let value = #value;
                #write_value_func
                #(#write_funcs)*
            }
        },
    )
}

fn handle_unnamed_fields(
    ident: &proc_macro2::Ident,
    value: Expr,
    fields: &syn::FieldsUnnamed,
    write_value_func: &TokenStream,
) -> (TokenStream, TokenStream, TokenStream) {
    let mut new_fields = vec![];
    let mut read_funcs = vec![];
    let mut write_funcs = vec![];
    let mut indices = vec![];

    for (i, f) in fields.unnamed.iter().enumerate() {
        let f_index = proc_macro2::Ident::new(&format!("index_{}", i), Span::call_site());
        let metas = get_attrs_meta(&f.attrs);
        let mut props = (VecDeque::new(), VecDeque::new());
        metas.iter().for_each(|meta| {
            check_meta(&mut props, meta);
        });

        let ty = &f.ty;
        let (new_field, read_func, write_func) = build_type_quotes(ty, &mut props, None);

        new_fields.push(quote! { #new_field, });
        read_funcs.push(quote! { #read_func, });
        write_funcs.push(quote! {
            let value = #f_index;
            #write_func;
        });
        indices.push(f_index);
    }

    (
        quote! { #ident(#(#new_fields)*), },
        quote! { #value => Self::#ident(#(#read_funcs)*), },
        quote! { Self::#ident (#(#indices),*) => {
                let value = #value;
                #write_value_func;
                #(#write_funcs)*
            }
        },
    )
}

fn handle_unit(
    ident: &proc_macro2::Ident,
    value: Expr,
    write_value_func: &TokenStream,
) -> (TokenStream, TokenStream, TokenStream) {
    (
        quote! { #ident = #value, },
        quote! { #value => Self::#ident, },
        quote! { Self::#ident => {
                let value = #value;
                #write_value_func;
            }
        },
    )
}
