use crate::util::{build_type_quotes, check_meta, get_attrs_meta};
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::VecDeque;
use syn::{DataStruct, Fields, Index, Visibility};

pub fn build_struct(
    vis: &Visibility,
    ident: &proc_macro2::Ident,
    data: &DataStruct,
) -> (TokenStream, TokenStream, TokenStream) {
    let (new_declare, read_funcs, write_funcs) = match &data.fields {
        Fields::Named(fields) => handle_named_fields(vis, ident, fields),
        Fields::Unnamed(fields) => handle_unnamed_fields(vis, ident, fields),
        Fields::Unit => panic!("Unit structs are not supported"),
    };

    (
        new_declare,
        quote! { Self { #(#read_funcs)* } },
        quote! { #(#write_funcs)* },
    )
}

fn handle_named_fields(
    vis: &Visibility,
    ident: &proc_macro2::Ident,
    fields: &syn::FieldsNamed,
) -> (TokenStream, Vec<TokenStream>, Vec<TokenStream>) {
    let mut new_fields = vec![];
    let mut read_funcs = vec![];
    let mut write_funcs = vec![];

    for f in &fields.named {
        let metas = get_attrs_meta(&f.attrs);
        let mut props = (VecDeque::new(), VecDeque::new());
        metas.iter().for_each(|meta| {
            check_meta(&mut props, meta);
        });

        let ty = &f.ty;
        let (new_field, read_func, write_func) = build_type_quotes(ty, &mut props, None);
        let field_vis = &f.vis;
        let field_ident = &f.ident;

        new_fields.push(quote! {
            #field_vis #field_ident: #new_field,
        });
        read_funcs.push(quote! {
            #field_ident: { #read_func },
        });
        write_funcs.push(quote! {
            let value = self.#field_ident.clone();
            { #write_func; };
        });
    }

    (
        quote! {
            #vis struct #ident {
                #(#new_fields)*
            }
        },
        read_funcs,
        write_funcs,
    )
}

fn handle_unnamed_fields(
    vis: &Visibility,
    ident: &proc_macro2::Ident,
    fields: &syn::FieldsUnnamed,
) -> (TokenStream, Vec<TokenStream>, Vec<TokenStream>) {
    let mut new_fields = vec![];
    let mut read_funcs = vec![];
    let mut write_funcs = vec![];

    for (i, f) in fields.unnamed.iter().enumerate() {
        let index = Index::from(i);
        let metas = get_attrs_meta(&f.attrs);
        let mut props = (VecDeque::new(), VecDeque::new());
        metas.iter().for_each(|meta| {
            check_meta(&mut props, meta);
        });

        let ty = &f.ty;
        let (new_field, read_func, write_func) = build_type_quotes(ty, &mut props, None);
        let field_vis = &f.vis;

        new_fields.push(quote! { #field_vis #new_field, });
        read_funcs.push(quote! { #index: #read_func, });
        write_funcs.push(quote! {
            let value = self.#index.clone();
            #write_func;
        });
    }

    (
        quote! {
            #vis struct #ident(#(#new_fields)*);
        },
        read_funcs,
        write_funcs,
    )
}
