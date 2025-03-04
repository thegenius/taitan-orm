// use crate::{FieldDef, FieldParser};
// use proc_macro2::TokenStream;
// use quote::{format_ident, quote};
// use syn::{Field, Ident, Type};
// use crate::info_parser::TypeParser;
// use std::option::Option;
// use case::CaseExt;
//
// fn field_to_tokens(field: &Field, force_to_double_option: bool) -> proc_macro2::TokenStream {
//     let ident = match &field.ident {
//         Some(ident) => quote! { #ident },
//         None => quote! { _unnamed }, // 匿名字段的默认名称
//     };
//     let ty = &field.ty;
//     let vis = &field.vis;
//     let attrs = &field.attrs;
//
//     if force_to_double_option {
//         let inner_type = TypeParser::get_inner_type(ty);
//         quote! {
//             #(#attrs)*
//             #vis #ident: std::option::Option<std::option::Option<#inner_type>>
//         }
//     } else {
//         quote! {
//             #(#attrs)*
//             #vis #ident: #ty
//         }
//     }
// }
//
// fn field_to_enum_expr_tokens(field: &Field) -> proc_macro2::TokenStream {
//     let field_name = &field.clone().ident.unwrap().to_string();
//     let enum_variant_name = format_ident!("{}", field_name.to_camel());
//
//     let ty = &field.ty;
//     let vis = &field.vis;
//     let attrs = &field.attrs;
//     let inner_type = TypeParser::get_inner_type(ty);
//     quote! {
//         #enum_variant_name(
//             #(#attrs)*
//             taitan_orm_trait::LocationExpr<#inner_type>
//         )
//     }
// }
//
// #[derive(Clone, Debug, Default)]
// pub struct StructFieldMapper;
//
// impl StructFieldMapper {
//     pub fn map_to_field(&self, field: &FieldDef, force_to_option: bool) -> TokenStream {
//         let origin_field = field.clone().struct_field.origin_field;
//         field_to_tokens(&origin_field, force_to_option)
//     }
//
//     pub fn map_to_enum_expr(&self, field: &FieldDef) -> TokenStream {
//         let origin_field = field.clone().struct_field.origin_field;
//         field_to_enum_expr_tokens(&origin_field)
//     }
// }
