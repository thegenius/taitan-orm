use crate::{FieldDef, KeywordsEscaper};
use proc_macro2::TokenStream;
use std::borrow::Cow;

pub trait SingleFieldMapper {
    fn get_value_name(&self) -> &'static str;
    fn map<'a>(&'a self, field: &'a FieldDef<'a>, escaper: &dyn KeywordsEscaper) -> Cow<'a, str>;
    fn map_indexed<'a>(
        &'a self,
        field: &'a FieldDef<'a>,
        escaper: &dyn KeywordsEscaper,
        index: usize,
    ) -> Cow<'a, str>;

    fn map_with_leading_comma<'a>(
        &'a self,
        field: &'a FieldDef<'a>,
        escaper: &dyn KeywordsEscaper,
    ) -> Cow<'a, str>;

    fn map_indexed_with_leading_comma<'a>(
        &'a self,
        field: &'a FieldDef<'a>,
        escaper: &dyn KeywordsEscaper,
        index: usize,
    ) -> Cow<'a, str>;

    fn map_dynamic(&self, field: &FieldDef, escaper: &dyn KeywordsEscaper) -> TokenStream;

    fn map_dynamic_with_leading_comma(&self, field: &FieldDef, escaper: &dyn KeywordsEscaper) -> TokenStream;

    fn map_dynamic_indexed(&self, field: &FieldDef, escaper: &dyn KeywordsEscaper) -> TokenStream;
    fn map_dynamic_indexed_with_leading_comma(&self, field: &FieldDef, escaper: &dyn KeywordsEscaper) -> TokenStream;
}





