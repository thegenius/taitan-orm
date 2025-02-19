use std::borrow::Cow;
use proc_macro2::TokenStream;
use quote::quote;
use crate::{FieldDef, KeywordsEscaper, SingleFieldMapper};

#[derive(Default)]
pub struct MarksMapper {}

impl SingleFieldMapper for MarksMapper {
    fn get_value_name(&self) -> &'static str {
        "marks"
    }
    fn map(&self, field: &FieldDef, escaper: &dyn KeywordsEscaper) -> Cow<'_, str> {
        Cow::Borrowed("?")
    }

    fn map_indexed<'a>(&'a self, field: &'a FieldDef<'a>, escaper: &dyn KeywordsEscaper, index: usize) -> Cow<'a, str> {
        Cow::Owned(format!("${}", index + 1))
    }

    fn map_with_leading_comma<'a>(
        &'a self,
        field: &'a FieldDef<'a>,
        escaper: &dyn KeywordsEscaper,
    ) -> Cow<'a, str> {
        Cow::Borrowed(",?")
    }

    fn map_indexed_with_leading_comma<'a>(&'a self, field: &'a FieldDef<'a>, escaper: &dyn KeywordsEscaper, index: usize) -> Cow<'a, str> {
        Cow::Owned(format!(",${}", index + 1))
    }

    fn map_dynamic(&self, field: &FieldDef, escaper: &dyn KeywordsEscaper) -> TokenStream {
        quote! { "?" }
    }

    fn map_dynamic_with_leading_comma(&self, field: &FieldDef, escaper: &dyn KeywordsEscaper) -> TokenStream {
        quote! { ",?" }
    }


    fn map_dynamic_indexed(&self, field: &FieldDef, escaper: &dyn KeywordsEscaper) -> TokenStream {
        quote! {format!("${}", index)}
    }

    fn map_dynamic_indexed_with_leading_comma(&self, field: &FieldDef, escaper: &dyn KeywordsEscaper) -> TokenStream {
        quote! {format!(",${}", index)}
    }
}