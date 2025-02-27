use super::super::base::{KeywordsEscaper, SingleFieldMapper};
use crate::field_mapper::base::{FieldSeg, FieldValSeg, LeadingCommaType};
use crate::FieldDef;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::borrow::Cow;

#[derive(Default, Clone, Debug)]
pub struct MarksMapper;

impl SingleFieldMapper for MarksMapper {
    fn map_static<'a>(&'a self, field: &'a FieldDef<'a>, escaper: &dyn KeywordsEscaper) -> Cow<'a, str> {
        Cow::Borrowed("?")
    }

    fn _map_static_indexed<'a>(&'a self, field: &'a FieldDef<'a>, escaper: &dyn KeywordsEscaper, index: usize) -> Cow<'a, str> {
        Cow::Owned(format!("${}", index + 1))
    }

    fn map_dynamic_indexed<'a>(&'a self, field: &'a FieldDef<'a>, escaper: &dyn KeywordsEscaper) -> Cow<'a, str> {
        Cow::Borrowed("${}")
    }


    // fn get_value_name(&self) -> &'static str {
    //     "marks"
    // }

    // fn map_single<'a>(
    //     &'a self,
    //     field: &'a FieldDef<'a>,
    //     escaper: &dyn KeywordsEscaper,
    //     indexed: bool,
    // ) -> FieldSeg<'a> {
    //     let ident = format_ident!("{}", field.struct_field.name);
    //     if indexed {
    //         FieldSeg::Val(FieldValSeg::IndexedSeg {
    //             val: Cow::Borrowed("${}"),
    //             ident,
    //         })
    //     } else {
    //         FieldSeg::Val(FieldValSeg::Seg {
    //             val: Cow::Borrowed("?"),
    //             ident,
    //         })
    //     }
    // }

    // fn map(
    //     &self,
    //     field: &FieldDef,
    //     escaper: &dyn KeywordsEscaper,
    //     leading_comma_type: LeadingCommaType,
    // ) -> Cow<'_, str> {
    //     match leading_comma_type {
    //         LeadingCommaType::NoLeading => Cow::Borrowed("?"),
    //         LeadingCommaType::Leading => Cow::Borrowed(",?"),
    //         LeadingCommaType::CheckedLeading => panic!(
    //             "MarksMapper: can not generate checked leading comma in compile time for {:?}",
    //             field
    //         ),
    //     }
    // }
    //
    // fn map_indexed<'a>(
    //     &'a self,
    //     field: &'a FieldDef<'a>,
    //     escaper: &dyn KeywordsEscaper,
    //     leading_comma: LeadingCommaType,
    //     index: usize,
    // ) -> Cow<'a, str> {
    //     match leading_comma {
    //         LeadingCommaType::NoLeading => Cow::Owned(format!("${}", index + 1)),
    //         LeadingCommaType::Leading => Cow::Owned(format!(",${}", index + 1)),
    //         LeadingCommaType::CheckedLeading => {
    //             panic!(
    //                 "MarksMapper: can not generate checked leading comma in compile time for {:?}",
    //                 field
    //             )
    //         }
    //     }
    // }

    // fn map_with_leading_comma<'a>(
    //     &'a self,
    //     field: &'a FieldDef<'a>,
    //     escaper: &dyn KeywordsEscaper,
    // ) -> Cow<'a, str> {
    //     Cow::Borrowed(",?")
    // }

    // fn map_indexed_with_leading_comma<'a>(
    //     &'a self,
    //     field: &'a FieldDef<'a>,
    //     escaper: &dyn KeywordsEscaper,
    //     index: usize,
    // ) -> Cow<'a, str> {
    //     Cow::Owned(format!(",${}", index + 1))
    // }

    // fn map_dynamic(
    //     &self,
    //     field: &FieldDef,
    //     escaper: &dyn KeywordsEscaper,
    //     leading_comma_type: LeadingCommaType,
    //     indexed: bool,
    // ) -> TokenStream {
    //     quote! { "?" }
    // }
    //
    // fn map_dynamic_with_leading_comma(
    //     &self,
    //     field: &FieldDef,
    //     escaper: &dyn KeywordsEscaper,
    // ) -> TokenStream {
    //     quote! { ",?" }
    // }
    //
    // fn map_dynamic_indexed(&self, field: &FieldDef, escaper: &dyn KeywordsEscaper) -> TokenStream {
    //     quote! {format!("${}", index)}
    // }
    //
    // fn map_dynamic_indexed_with_leading_comma(
    //     &self,
    //     field: &FieldDef,
    //     escaper: &dyn KeywordsEscaper,
    // ) -> TokenStream {
    //     quote! {format!(",${}", index)}
    // }
}
