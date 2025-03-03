use super::KeywordsEscaper;
use crate::field_mapper::base::field_seg::FieldSeg;
use crate::{FieldDef};
use proc_macro2::TokenStream;
use quote::format_ident;
use std::borrow::Cow;


#[derive(Clone, Copy)]
pub enum ConnectOp {
    Comma,
    And,
    Or,
}
impl Default for ConnectOp {
    fn default() -> Self {
        ConnectOp::Comma
    }
}
impl ConnectOp {
    pub fn as_str(&self) -> &'static str {
        match self {
            ConnectOp::Comma => ",",
            ConnectOp::And => " AND ",
            ConnectOp::Or => " OR ",
        }
    }
}

#[derive(Clone, Debug, Copy)]
pub enum LeadingCommaType {
    Leading,
    NoLeading,
    CheckedLeading,
}


pub trait SingleFieldMapper {
    fn map_static<'a>(
        &'a self,
        field: &'a FieldDef<'a>,
        escaper: &dyn KeywordsEscaper,
    ) -> Cow<'a, str> {
        panic!("_map_static is not implemented")
    }
    fn map_static_indexed<'a>(
        &'a self,
        field: &'a FieldDef<'a>,
        escaper: &dyn KeywordsEscaper,
        index: usize,
    ) -> Cow<'a, str> {
        self.map_static(field, escaper)
    }
    fn map_dynamic<'a>(
        &'a self,
        field: &'a FieldDef<'a>,
        escaper: &dyn KeywordsEscaper,
    ) -> Cow<'a, str> {
        self.map_static(field, escaper)
    }
    fn map_dynamic_indexed<'a>(
        &'a self,
        field: &'a FieldDef<'a>,
        escaper: &dyn KeywordsEscaper,
    ) -> Cow<'a, str> {
        self.map_dynamic(field, escaper)
    }

    fn map_single<'a>(
        &'a self,
        field: &'a FieldDef<'a>,
        escaper: &dyn KeywordsEscaper,
        is_optional: bool,
        indexed: bool,
        leading_comma_type: LeadingCommaType,
        connect_op: ConnectOp,
        is_enum: bool,
        is_cond: bool,
    ) -> TokenStream {
        let ident = format_ident!("{}", field.struct_field.get_name());
        let seg = if indexed {
            if is_cond && !field.is_location_expr() {
                self.map_static_indexed(field, escaper, 0)
            } else {
                self.map_dynamic_indexed(field, escaper)
            }
        } else {
            if is_cond && !field.is_location_expr() {
                self.map_static(field, escaper)
            } else {
                self.map_dynamic(field, escaper)
            }
        };
        let seg = FieldSeg::from(seg, Some(ident), indexed, field.is_location_expr());
        seg.translate(leading_comma_type, connect_op, is_optional, is_enum)
    }
    fn map_single_optional<'a>(
        &'a self,
        field: &'a FieldDef<'a>,
        escaper: &dyn KeywordsEscaper,
        indexed: bool,
        leading_comma_type: LeadingCommaType,
        connect_op: ConnectOp,
        is_enum: bool,
        static_cond: bool,
    ) -> TokenStream {
        self.map_single(field, escaper, true, indexed, leading_comma_type, connect_op, is_enum, static_cond)
    }

    // fn get_value_name(&self) -> &'static str;
    //
    // fn map_compile_time<'a>(
    //     &'a self,
    //     field: &'a FieldDef<'a>,
    //     escaper: &dyn KeywordsEscaper,
    //     index: Option<usize>,
    //     leading_comma: bool,
    // ) -> Cow<'a, str> {
    //     if let Some(index) = index {
    //         if leading_comma {
    //             self.map_indexed(field, escaper, LeadingCommaType::Leading, index)
    //         } else {
    //             self.map_indexed(field, escaper, LeadingCommaType::NoLeading, index)
    //         }
    //     } else {
    //         if leading_comma {
    //             self.map(field, escaper, LeadingCommaType::Leading)
    //         } else {
    //             self.map(field, escaper, LeadingCommaType::NoLeading)
    //         }
    //     }
    // }
    //
    // fn map_runtime_time<'a>(
    //     &'a self,
    //     field: &'a FieldDef<'a>,
    //     escaper: &dyn KeywordsEscaper,
    //     indexed: bool,
    //     leading_comma: bool,
    // ) -> TokenStream {
    //     if indexed {
    //         if leading_comma {
    //             self.map_dynamic_indexed_with_leading_comma(field, escaper)
    //         } else {
    //             self.map_dynamic_indexed(field, escaper)
    //         }
    //     } else {
    //         if leading_comma {
    //             self.map_dynamic(field, escaper, LeadingCommaType::Leading, false)
    //         } else {
    //             self.map_dynamic(field, escaper, LeadingCommaType::NoLeading, false)
    //         }
    //     }
    // }
    //
    // fn transform(
    //     &self,
    //     check_prev: bool,
    //     indexed: bool,
    //     optional_name: Option<&str>,
    //     is_first_required: bool,
    //     origin: TokenStream,
    // ) -> TokenStream {
    //     // release mode, "xxx".as_ref() will be optimized to "xxx"
    //     // so add #origin.as_ref() will be zero overhead
    //     let mut stream = if check_prev {
    //         quote! {
    //             if has_prev {
    //                 s.push(',');
    //             } else {
    //                 has_prev = true;
    //             }
    //             s.push_str(#origin.as_ref());
    //         }
    //     } else {
    //         quote! {
    //             s.push_str(#origin.as_ref());
    //         }
    //     };
    //
    //     // if dynamic indexed, add index + 1
    //     if indexed {
    //         stream.extend(quote! {
    //             index = index + 1;
    //         });
    //     }
    //
    //     // if is optional, wrap it
    //     if let Some(name) = optional_name {
    //         assert!(!is_first_required);
    //         let ident = format_ident!("{}", name);
    //         quote! {
    //             if !self.#ident.is_none() {
    //                 #stream
    //             }
    //         }
    //     } else {
    //         if is_first_required {
    //             quote! {
    //                 #stream
    //                 has_prev = true;
    //             }
    //         } else {
    //             stream
    //         }
    //     }
    // }
    //
    // fn transform_dynamic<'a>(
    //     &'a self,
    //     field: &'a FieldDef<'a>,
    //     escaper: &dyn KeywordsEscaper,
    //     indexed: bool,
    //     index: usize,
    //     group_index: usize,
    //     first_required_group_index: usize,
    // ) -> TokenStream {
    //     if !field.struct_field.is_optional {
    //         if index == 0 && group_index == 0 {
    //             assert_eq!(first_required_group_index, 0);
    //             // Required;
    //             let stream = self.map_runtime_time(field, escaper, indexed, false);
    //             return self.transform(false, indexed, None, true, stream);
    //         }
    //         if index == 0 && group_index <= first_required_group_index {
    //             assert_eq!(group_index, first_required_group_index);
    //             assert_ne!(group_index, 0);
    //             // RequiredCheckPrev;
    //             let stream = self.map_runtime_time(field, escaper, indexed, false);
    //             return self.transform(true, indexed, None, true, stream);
    //         }
    //         // RequiredLeadingComma
    //         let stream = self.map_runtime_time(field, escaper, indexed, true);
    //         return self.transform(false, indexed, None, false, stream);
    //     } else {
    //         if index == 0 && group_index == 0 {
    //             assert_ne!(first_required_group_index, 0);
    //             // StreamType::Optional;
    //             let stream = self.map_runtime_time(field, escaper, indexed, false);
    //             return self.transform(
    //                 false,
    //                 indexed,
    //                 Some(&field.struct_field.name),
    //                 false,
    //                 stream,
    //             );
    //         }
    //         if group_index < first_required_group_index {
    //             // Optional Check Prev
    //             let stream = self.map_runtime_time(field, escaper, indexed, false);
    //             return self.transform(
    //                 true,
    //                 indexed,
    //                 Some(&field.struct_field.name),
    //                 false,
    //                 stream,
    //             );
    //         } else {
    //             assert_ne!(group_index, first_required_group_index);
    //             // Optional Leading Comma
    //             let stream = self.map_runtime_time(field, escaper, indexed, true);
    //             return self.transform(
    //                 false,
    //                 indexed,
    //                 Some(&field.struct_field.name),
    //                 false,
    //                 stream,
    //             );
    //         }
    //     }
    // }

    // fn map_single_compiled<'a>(
    //     &'a self,
    //     field: &'a FieldDef<'a>,
    //     escaper: &dyn KeywordsEscaper,
    //     index: Option<usize>,
    // ) -> Cow<'a, str> {
    //     if let Some(idx) = index {
    //         let field_seg = self.map_single(field, escaper, true);
    //         assert_eq!(field_seg.is_expr(), false);
    //         assert_eq!(field_seg.is_indexed(), true);
    //         Cow::Owned(format!(field_seg.get_value())
    //     } else {
    //         let field_seg = self.map_single(field, escaper, false);
    //         assert_eq!(field_seg.is_expr(), false);
    //         assert_eq!(field_seg.is_indexed(), false);
    //         Cow::Borrowed(field_seg.get_value())
    //     }
    // }

    // fn map_single<'a>(
    //     &'a self,
    //     field: &'a FieldDef<'a>,
    //     escaper: &dyn KeywordsEscaper,
    //     indexed: bool,
    // ) -> FieldSeg<'a>;

    // fn map<'a>(
    //     &'a self,
    //     field: &'a FieldDef<'a>,
    //     escaper: &dyn KeywordsEscaper,
    //     leading_comma: LeadingCommaType,
    // ) -> Cow<'a, str>;
    //
    // fn map_indexed<'a>(
    //     &'a self,
    //     field: &'a FieldDef<'a>,
    //     escaper: &dyn KeywordsEscaper,
    //     leading_comma: LeadingCommaType,
    //     index: usize,
    // ) -> Cow<'a, str>;

    // fn translate_dynamic<'a>(
    //     &'a self,
    //     field: &'a FieldDef<'a>,
    //     escaper: &dyn KeywordsEscaper,
    //     leading_comma: LeadingCommaType,
    //     indexed: bool,
    // ) -> TokenStream {
    //     let ident = format_ident!("{}", field.struct_field.name);
    //     let field_seg = FieldSeg::from_str()
    // }

    // fn map_with_leading_comma<'a>(
    //     &'a self,
    //     field: &'a FieldDef<'a>,
    //     escaper: &dyn KeywordsEscaper,
    // ) -> Cow<'a, str>;

    // fn map_indexed_with_leading_comma<'a>(
    //     &'a self,
    //     field: &'a FieldDef<'a>,
    //     escaper: &dyn KeywordsEscaper,
    //     index: usize,
    // ) -> Cow<'a, str>;

    // fn map_dynamic(
    //     &self,
    //     field: &FieldDef,
    //     escaper: &dyn KeywordsEscaper,
    //     leading_comma_type: LeadingCommaType,
    //     indexed: bool,
    // ) -> TokenStream;
    //
    // fn map_dynamic_with_leading_comma(
    //     &self,
    //     field: &FieldDef,
    //     escaper: &dyn KeywordsEscaper,
    // ) -> TokenStream;
    //
    // fn map_dynamic_indexed(&self, field: &FieldDef, escaper: &dyn KeywordsEscaper) -> TokenStream;
    // fn map_dynamic_indexed_with_leading_comma(
    //     &self,
    //     field: &FieldDef,
    //     escaper: &dyn KeywordsEscaper,
    // ) -> TokenStream;
}
