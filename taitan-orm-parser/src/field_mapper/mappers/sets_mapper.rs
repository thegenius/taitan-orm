use super::super::base::{KeywordsEscaper, SingleFieldMapper};
use crate::field_mapper::base::{FieldSeg, FieldValSeg, LeadingCommaType};
use crate::FieldDef;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::borrow::Cow;

#[derive(Default, Debug, Clone)]
pub struct SetsMapper;

impl SingleFieldMapper for SetsMapper {
    fn _map_static<'a>(&'a self, field: &'a FieldDef<'a>, escaper: &dyn KeywordsEscaper) -> Cow<'a, str> {
        Cow::Owned(format!("{}=?", field.column_name(escaper)))
    }

    fn _map_static_indexed<'a>(&'a self, field: &'a FieldDef<'a>, escaper: &dyn KeywordsEscaper, index: usize) -> Cow<'a, str> {
        Cow::Owned(format!("{}=${}", field.column_name(escaper), index + 1))
    }


    // fn get_value_name(&self) -> &'static str {
    //     "sets"
    // }

    // fn map_single<'a>(&'a self, field: &'a FieldDef<'a>, escaper: &dyn KeywordsEscaper, indexed: bool) -> FieldSeg<'a> {
    //     let ident = format_ident!("{}", field.struct_field.name);
    //     let column_name = field.column_name(escaper);
    //     if indexed {
    //         FieldSeg::Val(FieldValSeg::IndexedSeg {
    //             val: Cow::Owned(format!("{column_name}=${{}}")),
    //             ident
    //         })
    //     } else {
    //         FieldSeg::Val(FieldValSeg::Seg {
    //             val: Cow::Owned(format!("{column_name}=?")),
    //             ident
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
    //         LeadingCommaType::Leading => Cow::Owned(format!(",{}=?", field.column_name(escaper))),
    //         LeadingCommaType::NoLeading => Cow::Owned(format!("{}=?", field.column_name(escaper))),
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
    //     match leading_comma_type {
    //         LeadingCommaType::NoLeading => {
    //             Cow::Owned(format!("{}=${}", field.column_name(escaper), index + 1))
    //         }
    //         LeadingCommaType::Leading => {
    //             Cow::Owned(format!(",{}=${}", field.column_name(escaper), index + 1))
    //         }
    //         LeadingCommaType::CheckedLeading => {
    //             panic!("can not generate checked leading comma in compile time")
    //         }
    //     }
    // }

    // fn map_with_leading_comma<'a>(
    //     &'a self,
    //     field: &'a FieldDef<'a>,
    //     escaper: &dyn KeywordsEscaper,
    // ) -> Cow<'a, str> {
    //     Cow::Owned(format!(",{}=?", field.column_name(escaper)))
    // }

    // fn map_indexed_with_leading_comma<'a>(
    //     &'a self,
    //     field: &'a FieldDef<'a>,
    //     escaper: &dyn KeywordsEscaper,
    //     index: usize,
    // ) -> Cow<'a, str> {
    //     Cow::Owned(format!(",{}=${}", field.column_name(escaper), index + 1))
    // }

    // fn map_dynamic(
    //     &self,
    //     field: &FieldDef,
    //     escaper: &dyn KeywordsEscaper,
    //     leading_comma_type: LeadingCommaType,
    //     indexed: bool,
    // ) -> TokenStream {
    //     let name = field.column_name(escaper);
    //     let format_str = format!("{}=?", name);
    //     quote! { #format_str }
    // }
    //
    // fn map_dynamic_with_leading_comma(
    //     &self,
    //     field: &FieldDef,
    //     escaper: &dyn KeywordsEscaper,
    // ) -> TokenStream {
    //     let name = field.column_name(escaper);
    //     let format_str = format!(",{}=?", name);
    //     quote! { #format_str }
    // }
    //
    // fn map_dynamic_indexed(&self, field: &FieldDef, escaper: &dyn KeywordsEscaper) -> TokenStream {
    //     let name = field.column_name(escaper);
    //     let format_str = format!("{}=${{}}", name);
    //     quote! {format!(#format_str, index)}
    // }
    //
    // fn map_dynamic_indexed_with_leading_comma(
    //     &self,
    //     field: &FieldDef,
    //     escaper: &dyn KeywordsEscaper,
    // ) -> TokenStream {
    //     let name = field.column_name(escaper);
    //     let format_str = format!(",{}=${{}}", name);
    //     quote! {format!(#format_str, index)}
    // }
}
