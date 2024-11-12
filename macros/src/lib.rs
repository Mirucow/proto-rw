use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Type};

mod enums;
mod structs;
mod util;

#[proc_macro_attribute]
pub fn proto_rw(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);

    let vis = &input.vis;
    let ident = &input.ident;

    let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();

    let (new_declare, read_funcs, write_funcs) = match &input.data {
        syn::Data::Struct(data) => structs::build_struct(vis, ident, data),
        syn::Data::Enum(data) => {
            enums::build_enum(parse_macro_input!(attr as Type), vis, ident, data)
        }
        _ => unimplemented!(),
    };

    let expanded = quote! {
        #new_declare

        impl #impl_generics proto_rw::ProtoRw for #ident #type_generics #where_clause {
            fn read<R: proto_rw::PRead>(buf: &mut R) -> Result<Self, proto_rw::error::ProtoRwError> {
                Ok(#read_funcs)
            }

            fn write<W: proto_rw::PWrite>(&self, buf: &mut W) -> Result<(), proto_rw::error::ProtoRwError> {
                #write_funcs
                Ok(())
            }
        }
    };

    TokenStream::from(expanded)
}
