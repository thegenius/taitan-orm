use super::KeywordsEscaper;
use crate::FieldDef;
use proc_macro2::TokenStream;
use std::borrow::Cow;

pub trait SingleFieldMapper {
    fn get_value_name(&self) -> &'static str;

    fn map_compile_time<'a>(
        &'a self,
        field: &'a FieldDef<'a>,
        escaper: &dyn KeywordsEscaper,
        index: Option<usize>,
        leading_comma: bool,
    ) -> Cow<'a, str> {
        if let Some(index) = index {
            if leading_comma {
                self.map_indexed_with_leading_comma(field, escaper, index)
            } else {
                self.map_indexed(field, escaper, index)
            }
        } else {
            if leading_comma {
                self.map_with_leading_comma(field, escaper)
            } else {
                self.map(field, escaper)
            }
        }
    }

    fn map_runtime_time<'a>(
        &'a self,
        field: &'a FieldDef<'a>,
        escaper: &dyn KeywordsEscaper,
        indexed: bool,
        leading_comma: bool,
    ) -> TokenStream {
        if indexed {
            if leading_comma {
                self.map_dynamic_indexed_with_leading_comma(field, escaper)
            } else {
                self.map_dynamic_indexed(field, escaper)
            }
        } else {
            if leading_comma {
                self.map_dynamic_with_leading_comma(field, escaper)
            } else {
                self.map_dynamic(field, escaper)
            }
        }
    }

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

    fn map_dynamic_with_leading_comma(
        &self,
        field: &FieldDef,
        escaper: &dyn KeywordsEscaper,
    ) -> TokenStream;

    fn map_dynamic_indexed(&self, field: &FieldDef, escaper: &dyn KeywordsEscaper) -> TokenStream;
    fn map_dynamic_indexed_with_leading_comma(
        &self,
        field: &FieldDef,
        escaper: &dyn KeywordsEscaper,
    ) -> TokenStream;
}
