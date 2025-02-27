use super::super::base::{KeywordsEscaper, SingleFieldMapper};
use crate::field_mapper::base::{FieldSeg, FieldValSeg, LeadingCommaType};
use crate::FieldDef;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::borrow::Cow;

#[derive(Default, Debug, Clone)]
pub struct UpsertSetsMapper;

impl SingleFieldMapper for UpsertSetsMapper {
    fn map_static<'a>(&'a self, field: &'a FieldDef<'a>, escaper: &dyn KeywordsEscaper) -> Cow<'a, str> {
        let name = field.column_name(escaper);
        let upsert_name = field.column_name_upsert(escaper);
        Cow::Owned(format!("{}={}", name, upsert_name))
    }

    // fn get_value_name(&self) -> &'static str {
    //     "upsert_names"
    // }

    // fn map_single<'a>(
    //     &'a self,
    //     field: &'a FieldDef<'a>,
    //     escaper: &dyn KeywordsEscaper,
    //     indexed: bool,
    // ) -> FieldSeg<'a> {
    //     let column_name = field.column_name(escaper);
    //     let upsert_name = field.column_name_upsert(escaper);
    //     let ident = format_ident!("{}", field.struct_field.name);
    //     FieldSeg::Val(FieldValSeg::Seg {
    //         val: Cow::Owned(format!("{column_name}={upsert_name}")),
    //         ident,
    //     })
    // }

    // fn map(
    //     &self,
    //     field: &FieldDef,
    //     escaper: &dyn KeywordsEscaper,
    //     leading_comma_type: LeadingCommaType,
    // ) -> Cow<'_, str> {
    //     match leading_comma_type {
    //         LeadingCommaType::NoLeading => {
    //             let name = field.column_name(escaper);
    //             let upsert_name = field.column_name_upsert(escaper);
    //             Cow::Owned(format!("{}={}", name, upsert_name))
    //         }
    //         LeadingCommaType::Leading => {
    //             let name = field.column_name(escaper);
    //             let upsert_name = field.column_name_upsert(escaper);
    //             Cow::Owned(format!(",{}={}", name, upsert_name))
    //         }
    //         LeadingCommaType::CheckedLeading => {
    //             panic!("can not generate checked leading comma in compile time")
    //         }
    //     }
    // }
    //
    // fn map_indexed<'a>(
    //     &'a self,
    //     field: &'a FieldDef<'a>,
    //     escaper: &dyn KeywordsEscaper,
    //     leading_comma_type: LeadingCommaType,
    //     index: usize,
    // ) -> Cow<'a, str> {
    //     UpsertSetsMapper::map(self, field, escaper, leading_comma_type)
    // }

    // fn map_with_leading_comma<'a>(
    //     &'a self,
    //     field: &'a FieldDef<'a>,
    //     escaper: &dyn KeywordsEscaper,
    // ) -> Cow<'a, str> {
    //     let name = field.column_name(escaper);
    //     let upsert_name = field.column_name_upsert(escaper);
    //     Cow::Owned(format!(",{}={}", name, upsert_name))
    // }

    // fn map_indexed_with_leading_comma<'a>(
    //     &'a self,
    //     field: &'a FieldDef<'a>,
    //     escaper: &dyn KeywordsEscaper,
    //     index: usize,
    // ) -> Cow<'a, str> {
    //     let name = field.column_name(escaper);
    //     let upsert_name = field.column_name_upsert(escaper);
    //     Cow::Owned(format!(",{}={}", name, upsert_name))
    // }

    // fn map_dynamic(
    //     &self,
    //     field: &FieldDef,
    //     escaper: &dyn KeywordsEscaper,
    //     leading_comma_type: LeadingCommaType,
    //     indexed: bool,
    // ) -> TokenStream {
    //     let name = field.column_name(escaper);
    //     let upsert_name = field.column_name_upsert(escaper);
    //     let format_str = format!("{}={}", name, upsert_name);
    //     quote! { #format_str }
    // }
    //
    // fn map_dynamic_with_leading_comma(
    //     &self,
    //     field: &FieldDef,
    //     escaper: &dyn KeywordsEscaper,
    // ) -> TokenStream {
    //     let name = field.column_name(escaper);
    //     let upsert_name = field.column_name_upsert(escaper);
    //     let format_str = format!(",{}={}", name, upsert_name);
    //     quote! { #format_str }
    // }
    //
    // fn map_dynamic_indexed(&self, field: &FieldDef, escaper: &dyn KeywordsEscaper) -> TokenStream {
    //     let name = field.column_name(escaper);
    //     let upsert_name = field.column_name_upsert(escaper);
    //     let format_str = format!("{}={}", name, upsert_name);
    //     quote! { #format_str }
    // }
    //
    // fn map_dynamic_indexed_with_leading_comma(
    //     &self,
    //     field: &FieldDef,
    //     escaper: &dyn KeywordsEscaper,
    // ) -> TokenStream {
    //     let name = field.column_name(escaper);
    //     let upsert_name = field.column_name_upsert(escaper);
    //     let format_str = format!(",{}={}", name, upsert_name);
    //     quote! { #format_str }
    // }
}
