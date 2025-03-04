use super::super::base::{KeywordsEscaper, SingleFieldMapper};
use crate::FieldDef;
use std::borrow::Cow;

#[derive(Default, Debug, Clone)]
pub struct ConditionsMapper;

impl SingleFieldMapper for ConditionsMapper {
    fn map_static<'a>(&'a self, field: &'a FieldDef<'a>, escaper: &dyn KeywordsEscaper) -> Cow<'a, str> {
        let column_name = field.column_name(escaper);
        Cow::Owned(format!("{column_name}=?"))
    }

    fn map_static_indexed<'a>(&'a self, field: &'a FieldDef<'a>, escaper: &dyn KeywordsEscaper, index: usize) -> Cow<'a, str> {
        let column_name = field.column_name(escaper);
        Cow::Owned(format!("{column_name}=${{}}"))
    }

    fn map_dynamic<'a>(
        &'a self,
        field: &'a FieldDef<'a>,
        escaper: &dyn KeywordsEscaper,
    ) -> Cow<'a, str> {
        let column_name = field.column_name(escaper);
        Cow::Owned(format!("{}{{}}?", column_name))
    }

    fn map_dynamic_indexed<'a>(
        &'a self,
        field: &'a FieldDef<'a>,
        escaper: &dyn KeywordsEscaper,
    ) -> Cow<'a, str> {
        let column_name = field.column_name(escaper);
        Cow::Owned(format!("{}{{}}${{}}", column_name))
    }


    // fn get_value_name(&self) -> &'static str {
    //     "conditions"
    // }

    // fn map_single<'a>(
    //     &'a self,
    //     field: &'a FieldDef<'a>,
    //     escaper: &dyn KeywordsEscaper,
    //     indexed: bool,
    // ) -> FieldSeg<'a> {
    //     let field_name = &field.struct_field.name;
    //     let column_name = field.column_name(escaper);
    //     let ident = format_ident!("{}", field_name);
    //     if indexed {
    //         FieldSeg::Expr(FieldExprSeg::IndexedSeg {
    //             val: Cow::Owned(format!("{column_name}{{}}${{}}")),
    //             ident,
    //         })
    //     } else {
    //         FieldSeg::Expr(FieldExprSeg::Seg {
    //             val: Cow::Owned(format!("{column_name}{{}}?")),
    //             ident,
    //         })
    //     }
    // }

    // fn map_single<'a>(
    //     &'a self,
    //     field: &'a FieldDef<'a>,
    //     escaper: &dyn KeywordsEscaper,
    //     indexed: bool,
    // ) -> FieldSeg<'a> {
    //     if indexed {
    //         FieldSeg::IndexedSeg(Cow::Owned(format!("{}{{}}?", name)))
    //     } else {
    //         FieldSeg::Seg(Cow::Owned(format!("{}{{}}?", name)))
    //     }
    // }

    // fn map(
    //     &self,
    //     field: &FieldDef,
    //     escaper: &dyn KeywordsEscaper,
    //     leading_comma: LeadingCommaType,
    // ) -> Cow<'_, str> {
    //     panic!("condition can not be mapped at compile time")
    // }
    //
    // fn map_indexed<'a>(
    //     &'a self,
    //     field: &'a FieldDef<'a>,
    //     escaper: &dyn KeywordsEscaper,
    //     leading_comma: LeadingCommaType,
    //     index: usize,
    // ) -> Cow<'a, str> {
    //     panic!("condition can not be mapped at compile time")
    // }

    // fn map_with_leading_comma<'a>(
    //     &'a self,
    //     field: &'a FieldDef<'a>,
    //     escaper: &dyn KeywordsEscaper,
    // ) -> Cow<'a, str> {
    //     panic!("condition can not be mapped at compile time")
    // }

    // fn map_indexed_with_leading_comma<'a>(
    //     &'a self,
    //     field: &'a FieldDef<'a>,
    //     escaper: &dyn KeywordsEscaper,
    //     index: usize,
    // ) -> Cow<'a, str> {
    //     panic!("condition can not be mapped at compile time")
    // }

    //     match &self.b {
    //     Optional::Some(b) => {
    //     format!(",b{}${}", b.get_cmp_sql(), index)
    //     },
    //     Optional::Null=> {
    //     format!(",b=${}",  index)
    //     }
    //     Optional::None => {
    //     "".to_string()
    //     }
    // }

    // fn map_dynamic(
    //     &self,
    //     field: &FieldDef,
    //     escaper: &dyn KeywordsEscaper,
    //     leading_comma_type: LeadingCommaType,
    //     indexed: bool,
    // ) -> TokenStream {
    //     if !field.is_location_expr() {
    //         panic!("condition must be generated from LocationExpr")
    //     }
    //     let column_name = field.column_name(escaper);
    //     let field_ident = format_ident!("{}", &field.struct_field.name);
    //
    //     let normal_condition_stream = match leading_comma_type {
    //         LeadingCommaType::NoLeading => {
    //             let format_str = format!("{}{{}}?", column_name);
    //             quote! { format!(#format_str, self.#field_ident.get_cmp_sql()) }
    //         }
    //         LeadingCommaType::Leading => {
    //             let format_str = format!(",{}{{}}?", column_name);
    //             quote! {
    //                 format!(#format_str, self.#field_ident.get_cmp_sql())
    //             }
    //         }
    //         LeadingCommaType::CheckedLeading => {
    //             let format_str_without_leading_comma = format!("{}{{}}?", column_name);
    //             let format_str_with_leading_comma = format!(",{}{{}}?", column_name);
    //             quote! {
    //                 if has_prev {
    //                     format!(#format_str_with_leading_comma, self.#field_ident.get_cmp_sql())
    //                 } else {
    //                     has_prev = true;
    //                     format!(#format_str_without_leading_comma, self.#field_ident.get_cmp_sql())
    //                 }
    //             }
    //         }
    //     };
    //
    //     let index_add_stream = if indexed {
    //         quote! { index += 1; }
    //     } else {
    //         quote! {}
    //     };
    //
    //     if field.is_required() {
    //         return quote! {
    //             #normal_condition_stream
    //             #index_add_stream
    //         };
    //     }
    //
    //     let null_condition_stream = match leading_comma_type {
    //         LeadingCommaType::NoLeading => {
    //             let format_str = format!("{}=?", column_name);
    //             quote! { #format_str.to_string() }
    //         }
    //         LeadingCommaType::Leading => {
    //             let format_str = format!(",{}=?", column_name);
    //             quote! { #format_str.to_string() }
    //         }
    //         LeadingCommaType::CheckedLeading => {
    //             let format_str_without_leading_comma = format!("{}=?", column_name);
    //             let format_str_with_leading_comma = format!(",{}=?", column_name);
    //             quote! {
    //                 if has_prev {
    //                     #format_str_with_leading_comma.to_string()
    //                 } else {
    //                     has_prev = true;
    //                     #format_str_without_leading_comma.to_string()
    //                 }
    //             }
    //         }
    //     };
    //
    //     quote! {
    //         match &self.#field_ident {
    //             taitan_orm_trait::Optional::Some(#field_ident) => {
    //                 #normal_condition_stream
    //             }
    //             taitan_orm_trait::Optional::Null => {
    //                 #null_condition_stream
    //             }
    //             taitan_orm_trait::Optional::None => {
    //                 "".to_string()
    //             }
    //         }
    //     }
    // }
    //
    // fn map_dynamic_with_leading_comma(
    //     &self,
    //     field: &FieldDef,
    //     escaper: &dyn KeywordsEscaper,
    // ) -> TokenStream {
    //     // let name = field.column_name(escaper);
    //     // let struct_name = &field.struct_field.name;
    //     // let struct_ident = format_ident!("{}", struct_name);
    //     // let format_str = format!(",{}{{}}?", name);
    //     // quote! {format!(#format_str, self.#struct_ident.get_cmp_sql())}
    //     let column_name = field.column_name(escaper);
    //     let field_ident = format_ident!("{}", &field.struct_field.name);
    //     let format_str = format!(",{}{{}}?", column_name);
    //     let null_condition = format!(",{}=?", column_name);
    //     quote! {
    //         match &self.#field_ident {
    //             taitan_orm_trait::Optional::Some(#field_ident) => {
    //                 format!(#format_str, self.#field_ident.get_cmp_sql())
    //             }
    //             taitan_orm_trait::Optional::Null => {
    //                 #null_condition.to_string()
    //             }
    //             taitan_orm_trait::Optional::None => {
    //                 "".to_string()
    //             }
    //         }
    //     }
    // }

    // fn map_dynamic_indexed(&self, field: &FieldDef, escaper: &dyn KeywordsEscaper) -> TokenStream {
    //     // let name = field.column_name(escaper);
    //     // let struct_name = &field.struct_field.name;
    //     // let struct_ident = format_ident!("{}", struct_name);
    //     // let format_str = format!("{}{{}}${{}}", name);
    //     // quote! {format!(#format_str, self.#struct_ident.get_cmp_sql(), index)}
    //     let column_name = field.column_name(escaper);
    //     let field_ident = format_ident!("{}", &field.struct_field.name);
    //     let format_str = format!("{}{{}}{{}}", column_name);
    //     let null_condition_str = format!("{}={{}}", column_name);
    //     quote! {
    //         match &self.#field_ident {
    //             taitan_orm_trait::Optional::Some(#field_ident) => {
    //                 format!(#format_str, #field_ident.get_cmp_sql(), index)
    //             }
    //             taitan_orm_trait::Optional::Null => {
    //                 format!(#null_condition_str, index)
    //             }
    //             taitan_orm_trait::Optional::None => {
    //                 "".to_string()
    //             }
    //         }
    //     }
    // }
    //
    // fn map_dynamic_indexed_with_leading_comma(
    //     &self,
    //     field: &FieldDef,
    //     escaper: &dyn KeywordsEscaper,
    // ) -> TokenStream {
    //     // let name = field.column_name(escaper);
    //     // let struct_name = &field.struct_field.name;
    //     // let struct_ident = format_ident!("{}", struct_name);
    //     // let format_str = format!(",{}{{}}${{}}", name);
    //     // quote! {format!(#format_str, self.#struct_ident.get_cmp_sql(), index)}
    //     let column_name = field.column_name(escaper);
    //     let field_ident = format_ident!("{}", &field.struct_field.name);
    //     let format_str = format!(",{}{{}}{{}}", column_name);
    //     let null_condition_str = format!(",{}={{}}", column_name);
    //     quote! {
    //         match &self.#field_ident {
    //             taitan_orm_trait::Optional::Some(#field_ident) => {
    //                 format!(#format_str, #field_ident.get_cmp_sql(), index)
    //             }
    //             taitan_orm_trait::Optional::Null => {
    //                 format!(#null_condition_str, index)
    //             }
    //             taitan_orm_trait::Optional::None => {
    //                 "".to_string()
    //             }
    //         }
    //     }
    // }
}
