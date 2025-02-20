use super::super::base::{KeywordsEscaper, SingleFieldMapper};
use crate::FieldDef;
use proc_macro2::TokenStream;
use quote::quote;
use std::borrow::Cow;

#[derive(Default, Debug, Clone)]
pub struct UpsertNamesMapper;

impl SingleFieldMapper for UpsertNamesMapper {
    fn get_value_name(&self) -> &'static str {
        "upsert_names"
    }

    fn map(&self, field: &FieldDef, escaper: &dyn KeywordsEscaper) -> Cow<'_, str> {
        let name = field.column_name(escaper);
        let upsert_name = field.column_name_upsert(escaper);
        Cow::Owned(format!("{}={}", name, upsert_name))
    }

    fn map_indexed<'a>(
        &'a self,
        field: &'a FieldDef<'a>,
        escaper: &dyn KeywordsEscaper,
        index: usize,
    ) -> Cow<'a, str> {
        let name = field.column_name(escaper);
        let upsert_name = field.column_name_upsert(escaper);
        Cow::Owned(format!("{}={}", name, upsert_name))
    }

    fn map_with_leading_comma<'a>(
        &'a self,
        field: &'a FieldDef<'a>,
        escaper: &dyn KeywordsEscaper,
    ) -> Cow<'a, str> {
        let name = field.column_name(escaper);
        let upsert_name = field.column_name_upsert(escaper);
        Cow::Owned(format!(",{}={}", name, upsert_name))
    }

    fn map_indexed_with_leading_comma<'a>(
        &'a self,
        field: &'a FieldDef<'a>,
        escaper: &dyn KeywordsEscaper,
        index: usize,
    ) -> Cow<'a, str> {
        let name = field.column_name(escaper);
        let upsert_name = field.column_name_upsert(escaper);
        Cow::Owned(format!(",{}={}", name, upsert_name))
    }

    fn map_dynamic(&self, field: &FieldDef, escaper: &dyn KeywordsEscaper) -> TokenStream {
        let name = field.column_name(escaper);
        let upsert_name = field.column_name_upsert(escaper);
        let format_str = format!("{}={}", name, upsert_name);
        quote! { #format_str }
    }

    fn map_dynamic_with_leading_comma(
        &self,
        field: &FieldDef,
        escaper: &dyn KeywordsEscaper,
    ) -> TokenStream {
        let name = field.column_name(escaper);
        let upsert_name = field.column_name_upsert(escaper);
        let format_str = format!(",{}={}", name, upsert_name);
        quote! { #format_str }
    }

    fn map_dynamic_indexed(&self, field: &FieldDef, escaper: &dyn KeywordsEscaper) -> TokenStream {
        let name = field.column_name(escaper);
        let upsert_name = field.column_name_upsert(escaper);
        let format_str = format!("{}={}", name, upsert_name);
        quote! { #format_str }
    }

    fn map_dynamic_indexed_with_leading_comma(
        &self,
        field: &FieldDef,
        escaper: &dyn KeywordsEscaper,
    ) -> TokenStream {
        let name = field.column_name(escaper);
        let upsert_name = field.column_name_upsert(escaper);
        let format_str = format!(",{}={}", name, upsert_name);
        quote! { #format_str }
    }
}
