use std::borrow::Cow;
use proc_macro2::TokenStream;
use quote::quote;
use crate::{FieldDef};
use super::super::base::{KeywordsEscaper, SingleFieldMapper};
#[derive(Default, Debug, Clone)]
pub struct NamesMapper;

impl SingleFieldMapper for NamesMapper {
    fn get_value_name(&self) -> &'static str {
        "names"
    }

    fn map<'a>(&'a self, field: &'a FieldDef<'a>, escaper: &dyn KeywordsEscaper) -> Cow<'a, str> {
        field.column_name(escaper)
    }

    fn map_indexed<'a>(&'a self, field: &'a FieldDef<'a>, escaper: &dyn KeywordsEscaper, index: usize) -> Cow<'a, str> {
        field.column_name(escaper)
    }


    fn map_with_leading_comma<'a>(
        &'a self,
        field: &'a FieldDef<'a>,
        escaper: &dyn KeywordsEscaper,
    ) -> Cow<'a, str> {
        Cow::Owned(format!(",{}", field.column_name(escaper)))
    }

    fn map_indexed_with_leading_comma<'a>(&'a self, field: &'a FieldDef<'a>, escaper: &dyn KeywordsEscaper, index: usize) -> Cow<'a, str> {
        Cow::Owned(format!(",{}", field.column_name(escaper)))
    }

    fn map_dynamic(&self, field: &FieldDef, escaper: &dyn KeywordsEscaper) -> TokenStream {
        let name = field.column_name(escaper);
        quote! { #name }
    }

    fn map_dynamic_with_leading_comma(&self, field: &FieldDef, escaper: &dyn KeywordsEscaper) -> TokenStream {
        let name = field.column_name(escaper);
        quote! { #name }
    }


    fn map_dynamic_indexed(&self, field: &FieldDef, escaper: &dyn KeywordsEscaper) -> TokenStream {
        let name = field.column_name(escaper);
        quote! { #name }
    }

    fn map_dynamic_indexed_with_leading_comma(&self, field: &FieldDef, escaper: &dyn KeywordsEscaper) -> TokenStream {
        let name = format!(",{}", field.column_name(escaper));
        quote! { #name }
    }
}