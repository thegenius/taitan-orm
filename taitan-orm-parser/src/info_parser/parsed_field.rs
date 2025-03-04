use crate::info_parser::option_parser::OptionParser;
use crate::info_parser::TypeParser;
use crate::{FieldName, LifetimeParser};
use case::CaseExt;
use quote::{format_ident, quote, ToTokens};
use std::borrow::Cow;
use syn::Field;

#[derive(Debug, PartialEq, Clone)]
pub struct ParsedField<'a> {
    pub name: FieldName<'a>,
    pub rust_type: Cow<'a, str>,
    pub option_nest_level: usize,
    pub is_location_expr: bool,
    pub is_enum_variant: bool,
    pub lifetime: Option<Cow<'a, str>>,
    pub origin_field: Field,
}

#[derive(Debug, Clone, Copy)]
pub enum FieldTokenType {
    InnerMostType,    // primary/unique/index使用
    NestedOptionType, // mutation/selected使用, Option<Option<T>>
    VariantExpr,      // Location使用
}
impl<'a> ParsedField<'a> {
    pub fn parse(field: &Field, is_enum_variant: bool, unnamed_idx: Option<usize>) -> Self {
        let field_name = if let Some(ident) = field.clone().ident {
            FieldName::Named(Cow::Owned(ident.to_string()))
        } else {
            FieldName::unnamed(unnamed_idx.unwrap())
        };
        Self::new(&field, field_name, is_enum_variant)
    }
    pub fn new(
        origin_field: &Field,
        field_name: FieldName<'a>,
        is_enum_variant: bool,
    ) -> ParsedField<'a> {
        let (field_type, option_nest_level) =
            OptionParser::get_nested_option_inner_type(&origin_field.ty);
        let field_type_str = field_type.to_token_stream().to_string();
        let is_location_expr = TypeParser::is_location_expr(&field_type);
        let lifetime =
            LifetimeParser::get_lifetime(&origin_field.ty).map(|l| Cow::Owned(l.to_string()));
        ParsedField {
            name: field_name.clone(),
            rust_type: Cow::Owned(field_type_str.clone()),
            is_enum_variant,
            is_location_expr,
            lifetime,
            option_nest_level,
            origin_field: origin_field.clone(),
        }
    }

    pub fn get_name(&self) -> Cow<'a, str> {
        match &self.name {
            FieldName::Named(n) => n.clone(),
            FieldName::Unnamed { idx: _, name } => name.clone(),
        }
    }
    pub fn is_option(&self) -> bool {
        self.option_nest_level > 0
    }
    pub fn to_token_stream(&self, token_type: FieldTokenType) -> proc_macro2::TokenStream {
        let field_name = &self.name.get_name();
        let field_ident = format_ident!("{}", field_name);
        let ty = &self.origin_field.ty;
        let vis = &self.origin_field.vis;
        let attrs = &self.origin_field.attrs;
        let (inner_type, _) = OptionParser::get_nested_option_inner_type(ty);
        return match token_type {
            FieldTokenType::InnerMostType => {
                quote! {
                    #(#attrs)*
                    #vis #field_ident :#inner_type
                }
            }
            FieldTokenType::NestedOptionType => {
                quote! {
                    #(#attrs)*
                    #vis #field_ident :std::option::Option<std::option::Option<#inner_type>>
                }
            }
            FieldTokenType::VariantExpr => {
                let enum_variant_name = format_ident!("{}", field_name.to_camel());
                quote! {
                    #enum_variant_name(
                        #(#attrs)*
                        taitan_orm_trait::LocationExpr<#inner_type>
                    )
                }
            }
        };
    }
}
