use core::panic;
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::VecDeque;
use syn::{parse::ParseStream, Attribute, Index, Meta, MetaList, PathSegment, Token, Type};

pub fn build_type_quotes(
    ty: &Type,
    props: &mut (VecDeque<Option<Type>>, VecDeque<Type>),
    index: Option<Index>,
) -> (TokenStream, TokenStream, TokenStream) {
    let value = if let Some(index) = index {
        quote! { value.#index }
    } else {
        quote! { value }
    };

    match ty {
        Type::Path(type_path) => handle_path_type(type_path, props, value),
        Type::Tuple(type_tuple) => handle_tuple_type(type_tuple, props, value),
        Type::Array(type_array) => handle_array_type(type_array, props, value),
        _ => panic!("Unsupported type found. Expected a path or tuple"),
    }
}

fn handle_path_type(
    type_path: &syn::TypePath,
    props: &mut (VecDeque<Option<Type>>, VecDeque<Type>),
    value: proc_macro2::TokenStream,
) -> (TokenStream, TokenStream, TokenStream) {
    let segment = type_path
        .path
        .segments
        .first()
        .cloned()
        .expect("No segments found in type path");
    let ident = segment.ident.clone();

    match ident.to_string().as_str() {
        "LE" | "BE" | "Var" => handle_numeric_type(&segment, &ident, value),
        "Vec" => handle_vec_type(&segment, props, value),
        _ => handle_default_type(type_path, props, value),
    }
}

fn handle_numeric_type(
    segment: &PathSegment,
    ident: &syn::Ident,
    value: TokenStream,
) -> (TokenStream, TokenStream, TokenStream) {
    let gen_type = extract_generic_type(segment)
        .unwrap_or_else(|| panic!("No generic type found for {}", ident));

    (
        quote! { #gen_type },
        quote! { buf.read_proto::<#ident<#gen_type>>()?.into() },
        quote! { #ident::<#gen_type>::from(#value).write(buf)? },
    )
}

fn handle_vec_type(
    segment: &PathSegment,
    props: &mut (VecDeque<Option<Type>>, VecDeque<Type>),
    value: TokenStream,
) -> (TokenStream, TokenStream, TokenStream) {
    let gen_type =
        extract_generic_type(segment).unwrap_or_else(|| panic!("No generic type found for Vec"));
    let length_type = props.1.pop_front().expect("No length type found for Vec");

    let (length_ident, length_gen_type) = extract_length_type(&length_type);
    let (inner_type, inner_read, inner_write) = build_type_quotes(&gen_type, props, None);

    (
        quote! { Vec<#inner_type> },
        quote! {
            let len: #length_gen_type = buf.read_proto::<#length_ident<#length_gen_type>>()?.into();
            let mut vec = Vec::with_capacity(len as usize);
            for _ in 0..len {
                vec.push({ #inner_read });
            }
            vec
        },
        quote! {
            let len = #value.len() as #length_gen_type;
            #length_ident::<#length_gen_type>(len).write(buf)?;
            for value in #value {
                { #inner_write }
            }
        },
    )
}

fn handle_default_type(
    ty: &syn::TypePath,
    props: &mut (VecDeque<Option<Type>>, VecDeque<Type>),
    value: TokenStream,
) -> (TokenStream, TokenStream, TokenStream) {
    if let Some(convert_type) = props.0.pop_front() {
        if let Some(convert_type) = convert_type {
            return (
                quote! { #convert_type },
                quote! { buf.read_proto::<#ty>()?.into() },
                quote! { #ty::from(#value).write(buf)? },
            );
        }
    }

    (
        quote! { #ty },
        quote! { buf.read_proto::<#ty>()? },
        quote! { #value.write(buf)? },
    )
}

fn extract_length_type(length_type: &Type) -> (syn::Ident, Type) {
    if let Type::Path(type_path) = length_type {
        let segment = type_path
            .path
            .segments
            .first()
            .cloned()
            .expect("No segments found in type path");
        let ident = segment.ident.clone();

        if ["LE", "BE", "Var"].contains(&ident.to_string().as_str()) {
            let gen_type =
                extract_generic_type(&segment).expect("No generic type found for length type");
            (ident, gen_type)
        } else {
            panic!("Invalid length type found for Vec: Not LE, BE, or Var");
        }
    } else {
        panic!("Invalid length type found for Vec: Not a path");
    }
}

fn handle_tuple_type(
    type_tuple: &syn::TypeTuple,
    props: &mut (VecDeque<Option<Type>>, VecDeque<Type>),
    value: TokenStream,
) -> (TokenStream, TokenStream, TokenStream) {
    let mut new_fields = vec![];
    let mut read_funcs = vec![];
    let mut write_funcs = vec![];

    for (i, ty) in type_tuple.elems.iter().enumerate() {
        let index = Index::from(i);
        let (new_field, read_func, write_func) = build_type_quotes(ty, props, Some(index));

        new_fields.push(new_field);
        read_funcs.push(read_func);
        write_funcs.push(write_func);
    }

    (
        quote! { (#(#new_fields),*) },
        quote! { (#({ #read_funcs }),*) },
        quote! {
            {
                let value = #value;
                #({ #write_funcs };)*
            }
        },
    )
}

fn handle_array_type(
    type_array: &syn::TypeArray,
    props: &mut (VecDeque<Option<Type>>, VecDeque<Type>),
    value: TokenStream,
) -> (TokenStream, TokenStream, TokenStream) {
    let len = &type_array.len;
    let (inner_type, inner_read, inner_write) = build_type_quotes(&type_array.elem, props, None);

    (
        quote! { [#inner_type; #len] },
        quote! {
            {
                let mut arr = vec![];
                for _ in 0..#len {
                    arr.push({ #inner_read });
                }
                arr.try_into().map_err(|_| proto_rw::error::ProtoRwError::Error(format!(
                    "Array length does not match. Expected {}",
                    #len
                )))?
            }
        },
        quote! {
            for value in #value.iter() {
                { #inner_write };
            }
        },
    )
}

pub fn get_attrs_meta(attrs: &[Attribute]) -> Vec<Meta> {
    attrs.iter().map(|attr| attr.meta.clone()).collect()
}

pub fn check_meta(props: &mut (VecDeque<Option<Type>>, VecDeque<Type>), meta: &Meta) {
    let Meta::List(list) = meta else {
        return;
    };

    let ident = list
        .path
        .get_ident()
        .expect("No ident found in meta list")
        .to_string();

    match ident.as_str() {
        "convert" => handle_convert(list, props),
        "length" => handle_length(list, props),
        _ => panic!("Unsupported meta found. Expected convert or length"),
    }
}

fn handle_convert(list: &MetaList, props: &mut (VecDeque<Option<Type>>, VecDeque<Type>)) {
    let convert_types = list
        .parse_args_with(|input: ParseStream| {
            let mut types = Vec::new();

            while !input.is_empty() {
                let ty = input.parse::<Type>()?;
                let content;
                syn::bracketed!(content in input);
                let index = content
                    .parse::<syn::Index>()
                    .expect("Expected a numeric index in brackets");

                types.push((ty, index.index));
                input.parse::<Token![,]>().ok();
            }

            Ok(types)
        })
        .expect("Expected type[index] pairs");

    let mut convert_type_list = VecDeque::new();
    for (ty, index) in convert_types {
        while convert_type_list.len() < index as usize {
            convert_type_list.push_back(None);
        }
        convert_type_list.push_back(Some(ty));
    }

    props.0 = convert_type_list;
}

fn handle_length(list: &MetaList, props: &mut (VecDeque<Option<Type>>, VecDeque<Type>)) {
    let length_types = list
        .parse_args_with(|input: ParseStream| {
            let mut types = Vec::new();
            while !input.is_empty() {
                types.push(input.parse::<Type>()?);
                input.parse::<Token![,]>().ok();
            }
            Ok(types)
        })
        .expect("Expected a list of types");

    props.1 = VecDeque::from(length_types);
}

pub fn extract_generic_type(segment: &PathSegment) -> Option<Type> {
    if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
        if let Some(syn::GenericArgument::Type(ty)) = args.args.first() {
            return Some(ty.clone());
        }
    }

    None
}
