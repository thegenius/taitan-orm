use std::borrow::Cow;
use proc_macro2::TokenStream;
use quote::quote;
use crate::{FieldDef, KeywordsEscaper, SingleFieldMapper};

#[derive(Default)]
pub struct SetsMapper {}

impl SingleFieldMapper for SetsMapper {
    fn get_value_name(&self) -> &'static str {
        "sets"
    }

    fn map(&self, field: &FieldDef, escaper: &dyn KeywordsEscaper) -> Cow<'_, str> {
        Cow::Owned(format!("{}=?", field.column_name(escaper)))
    }

    fn map_indexed<'a>(&'a self, field: &'a FieldDef<'a>, escaper: &dyn KeywordsEscaper, index: usize) -> Cow<'a, str> {
        Cow::Owned(format!("{}=${}", field.column_name(escaper), index + 1))
    }

    fn map_with_leading_comma<'a>(
        &'a self,
        field: &'a FieldDef<'a>,
        escaper: &dyn KeywordsEscaper,
    ) -> Cow<'a, str> {
        Cow::Owned(format!(",{}=?", field.column_name(escaper)))
    }

    fn map_indexed_with_leading_comma<'a>(&'a self, field: &'a FieldDef<'a>, escaper: &dyn KeywordsEscaper, index: usize) -> Cow<'a, str> {
        Cow::Owned(format!(",{}=${}", field.column_name(escaper), index + 1))
    }

    fn map_dynamic(&self, field: &FieldDef, escaper: &dyn KeywordsEscaper) -> TokenStream {
        let name = field.column_name(escaper);
        let format_str = format!("{}=?", name);
        quote! { #format_str }
    }

    fn map_dynamic_with_leading_comma(&self, field: &FieldDef, escaper: &dyn KeywordsEscaper) -> TokenStream {
        let name = field.column_name(escaper);
        let format_str = format!(",{}=?", name);
        quote! { #format_str }
    }


    fn map_dynamic_indexed(&self, field: &FieldDef, escaper: &dyn KeywordsEscaper) -> TokenStream {
        let name = field.column_name(escaper);
        let format_str = format!("{}=${{}}", name);
        quote! {format!(#format_str, index)}
    }

    fn map_dynamic_indexed_with_leading_comma(&self, field: &FieldDef, escaper: &dyn KeywordsEscaper) -> TokenStream {
        let name = field.column_name(escaper);
        let format_str = format!(",{}=${{}}", name);
        quote! {format!(#format_str, index)}
    }
}