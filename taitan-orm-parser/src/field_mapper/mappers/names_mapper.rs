use super::super::base::{KeywordsEscaper, SingleFieldMapper};
use crate::FieldDef;
use std::borrow::Cow;
#[derive(Default, Debug, Clone)]
pub struct NamesMapper;

impl SingleFieldMapper for NamesMapper {
    fn map_static<'a>(&'a self, field: &'a FieldDef<'a>, escaper: &dyn KeywordsEscaper) -> Cow<'a, str> {
        field.column_name(escaper)
    }


    // fn get_value_name(&self) -> &'static str {
    //     "names"
    // }

    // fn map_single<'a>(&'a self, field: &'a FieldDef<'a>, escaper: &dyn KeywordsEscaper, indexed: bool) -> FieldSeg<'a> {
    //     let column_name = field.column_name(escaper);
    //     let ident = format_ident!("{}", field.struct_field.name);
    //     FieldSeg::Val(FieldValSeg::Seg {
    //         val: column_name,
    //         ident,
    //     })
    // }


    // fn map<'a>(
    //     &'a self,
    //     field: &'a FieldDef<'a>,
    //     escaper: &dyn KeywordsEscaper,
    //     leading_comma_type: LeadingCommaType,
    // ) -> Cow<'a, str> {
    //     match leading_comma_type {
    //         LeadingCommaType::NoLeading => field.column_name(escaper),
    //         LeadingCommaType::Leading => Cow::Owned(format!(",{}", field.column_name(escaper))),
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
    //     NamesMapper::map(self, field, escaper, leading_comma_type)
    // }

    // fn map_with_leading_comma<'a>(
    //     &'a self,
    //     field: &'a FieldDef<'a>,
    //     escaper: &dyn KeywordsEscaper,
    // ) -> Cow<'a, str> {
    //     Cow::Owned(format!(",{}", field.column_name(escaper)))
    // }

    // fn map_indexed_with_leading_comma<'a>(
    //     &'a self,
    //     field: &'a FieldDef<'a>,
    //     escaper: &dyn KeywordsEscaper,
    //     index: usize,
    // ) -> Cow<'a, str> {
    //     Cow::Owned(format!(",{}", field.column_name(escaper)))
    // }

    // fn map_dynamic(
    //     &self,
    //     field: &FieldDef,
    //     escaper: &dyn KeywordsEscaper,
    //     leading_comma_type: LeadingCommaType,
    //     indexed: bool,
    // ) -> TokenStream {
    //     let name = field.column_name(escaper);
    //     quote! { #name }
    // }
    //
    // fn map_dynamic_with_leading_comma(
    //     &self,
    //     field: &FieldDef,
    //     escaper: &dyn KeywordsEscaper,
    // ) -> TokenStream {
    //     let name = field.column_name(escaper);
    //     let name_with_comma = format!(",{}", name);
    //     quote! { #name_with_comma }
    // }
    //
    // fn map_dynamic_indexed(&self, field: &FieldDef, escaper: &dyn KeywordsEscaper) -> TokenStream {
    //     let name = field.column_name(escaper);
    //     quote! { #name }
    // }
    //
    // fn map_dynamic_indexed_with_leading_comma(
    //     &self,
    //     field: &FieldDef,
    //     escaper: &dyn KeywordsEscaper,
    // ) -> TokenStream {
    //     let name = format!(",{}", field.column_name(escaper));
    //     quote! { #name }
    // }
}
