use crate::{FieldDef, FieldParser};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Field, Ident, Type};
use crate::info_parser::TypeParser;
use std::option::Option;

fn field_to_tokens(field: &Field, force_to_option: bool) -> proc_macro2::TokenStream {
    let ident = match &field.ident {
        Some(ident) => quote! { #ident },
        None => quote! { _unnamed }, // 匿名字段的默认名称
    };
    let ty = &field.ty;
    let vis = &field.vis;
    let attrs = &field.attrs;
    let origin_is_option =  TypeParser::is_option(ty);
    if force_to_option && !origin_is_option {
        quote! {
            #(#attrs)*
            #vis #ident: std::option::Option<#ty>
        }
    } else {
        quote! {
            #(#attrs)*
            #vis #ident: #ty
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct StructFieldMapper;

impl StructFieldMapper {
    pub fn map_to_field(&self, field: &FieldDef, force_to_option: bool) -> TokenStream {
        let origin_field = field.clone().struct_field.field.unwrap();
        field_to_tokens(&origin_field, force_to_option)
    }
}
