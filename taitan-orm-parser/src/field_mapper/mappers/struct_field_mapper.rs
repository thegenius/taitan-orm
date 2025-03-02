use crate::FieldDef;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Field, Ident, Type};


fn field_to_tokens(field: &Field) -> proc_macro2::TokenStream {
    let ident = match &field.ident {
        Some(ident) => quote! { #ident },
        None => quote! { _unnamed }, // 匿名字段的默认名称
    };
    let ty = &field.ty;
    let vis = &field.vis;
    let attrs = &field.attrs;

    quote! {
        #(#attrs)*
        #vis #ident: #ty
    }
}

#[derive(Clone, Debug, Default)]
pub struct StructFieldMapper;

impl StructFieldMapper {
    pub fn map_to_field(&self, field: &FieldDef) -> TokenStream {
        let origin_field = field.clone().struct_field.field.unwrap();
        field_to_tokens(&origin_field)
    }
}
