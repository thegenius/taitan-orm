use std::borrow::Cow;
use crate::{FieldDef, KeywordsEscaper};
use proc_macro2::TokenStream;
use quote::quote;

pub trait SingleFieldMapper {
    fn get_value_name(&self) -> &'static str;
    fn map<'a>(&'a self, field: &'a FieldDef<'a>, escaper: &dyn KeywordsEscaper) -> Cow<'a, str>;
    fn map_with_leading_comma<'a>(&'a self, field: &'a FieldDef<'a>, escaper: &dyn KeywordsEscaper) -> Cow<'a, str>;
    fn map_indexed_dynamic(&self, field: &FieldDef, escaper: &dyn KeywordsEscaper) -> TokenStream;
    fn map_indexed_static(
        &self,
        field: &FieldDef,
        index: usize,
        escaper: &dyn KeywordsEscaper,
    ) -> TokenStream;
}

#[derive(Default)]
pub struct NamesMapper {}

impl SingleFieldMapper for NamesMapper {
    fn get_value_name(&self) -> &'static str {
        "names"
    }

    // fn map(&self, field: &FieldDef, escaper: &dyn KeywordsEscaper) -> TokenStream {
    //     let column_name = field.column_name(escaper);
    //     quote! { #column_name }
    // }

    fn map<'a>(&'a self, field: &'a FieldDef<'a>, escaper: &dyn KeywordsEscaper) -> Cow<'a, str> {
       field.column_name(escaper)
    }

    fn map_with_leading_comma<'a>(&'a self, field: &'a FieldDef<'a>, escaper: &dyn KeywordsEscaper) -> Cow<'a, str> {
        Cow::Owned(format!(",{}", field.column_name(escaper)))
    }

    fn map_indexed_dynamic(&self, field: &FieldDef, escaper: &dyn KeywordsEscaper) -> TokenStream {
        let name = field.column_name(escaper);
        quote! { #name }
    }

    fn map_indexed_static(
        &self,
        field: &FieldDef,
        index: usize,
        escaper: &dyn KeywordsEscaper,
    ) -> TokenStream {
        let name = field.column_name(escaper);
        quote! { #name }
    }
}

#[derive(Default)]
pub struct MarksMapper {}

impl SingleFieldMapper for MarksMapper {
    fn get_value_name(&self) -> &'static str {
        "marks"
    }

    // fn map(&self, field: &FieldDef, escaper: &dyn KeywordsEscaper) -> TokenStream {
    //     quote! { '?' }
    // }

    fn map(&self, field: &FieldDef, escaper: &dyn KeywordsEscaper) -> Cow<'_, str> {
        Cow::Borrowed("?")
    }

    fn map_with_leading_comma<'a>(&'a self, field: &'a FieldDef<'a>, escaper: &dyn KeywordsEscaper) -> Cow<'a, str> {
        Cow::Borrowed(",?")
    }

    fn map_indexed_dynamic(&self, field: &FieldDef, escaper: &dyn KeywordsEscaper) -> TokenStream {
        quote! {format!("${}", index)}
    }

    fn map_indexed_static(
        &self,
        field: &FieldDef,
        index: usize,
        escaper: &dyn KeywordsEscaper,
    ) -> TokenStream {
        let mark = format!("${}", index);
        quote! { #mark }
    }
}
