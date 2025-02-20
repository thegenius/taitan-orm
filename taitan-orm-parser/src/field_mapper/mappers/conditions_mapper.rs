use std::borrow::Cow;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use crate::{FieldDef};
use super::super::base::{KeywordsEscaper, SingleFieldMapper};

#[derive(Default, Debug, Clone)]
pub struct ConditionsMapper;

impl SingleFieldMapper for ConditionsMapper {
    fn get_value_name(&self) -> &'static str {
        "conditions"
    }

    fn map(&self, field: &FieldDef, escaper: &dyn KeywordsEscaper) -> Cow<'_, str> {
        panic!("condition can not be mapped at compile time")
    }

    fn map_indexed<'a>(&'a self, field: &'a FieldDef<'a>, escaper: &dyn KeywordsEscaper, index: usize) -> Cow<'a, str> {
        panic!("condition can not be mapped at compile time")
    }

    fn map_with_leading_comma<'a>(
        &'a self,
        field: &'a FieldDef<'a>,
        escaper: &dyn KeywordsEscaper,
    ) -> Cow<'a, str> {
        panic!("condition can not be mapped at compile time")
    }

    fn map_indexed_with_leading_comma<'a>(&'a self, field: &'a FieldDef<'a>, escaper: &dyn KeywordsEscaper, index: usize) -> Cow<'a, str> {
        panic!("condition can not be mapped at compile time")
    }

    fn map_dynamic(&self, field: &FieldDef, escaper: &dyn KeywordsEscaper) -> TokenStream {
        let name = field.column_name(escaper);
        let struct_name = &field.struct_field.name;
        let struct_ident = format_ident!("{}", struct_name);
        let format_str = format!("{}{{}}?", name);
        quote! {format!(#format_str, self.#struct_ident.get_cmp_sql())}
    }

    fn map_dynamic_with_leading_comma(&self, field: &FieldDef, escaper: &dyn KeywordsEscaper) -> TokenStream {
        let name = field.column_name(escaper);
        let struct_name = &field.struct_field.name;
        let struct_ident = format_ident!("{}", struct_name);
        let format_str = format!(",{}{{}}?", name);
        quote! {format!(#format_str, self.#struct_ident.get_cmp_sql())}
    }


    fn map_dynamic_indexed(&self, field: &FieldDef, escaper: &dyn KeywordsEscaper) -> TokenStream {
        let name = field.column_name(escaper);
        let struct_name = &field.struct_field.name;
        let struct_ident = format_ident!("{}", struct_name);
        let format_str = format!("{}{{}}${{}}", name);
        quote! {format!(#format_str, self.#struct_ident.get_cmp_sql(), index)}
    }

    fn map_dynamic_indexed_with_leading_comma(&self, field: &FieldDef, escaper: &dyn KeywordsEscaper) -> TokenStream {
        let name = field.column_name(escaper);
        let struct_name = &field.struct_field.name;
        let struct_ident = format_ident!("{}", struct_name);
        let format_str = format!(",{}{{}}${{}}", name);
        quote! {format!(#format_str, self.#struct_ident.get_cmp_sql(), index)}
    }
}
