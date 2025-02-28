use super::{FieldSeg, KeywordsEscaper, LeadingCommaType, SingleFieldMapper};
use crate::FieldDef;
use proc_macro2::TokenStream;
use quote::quote;
use std::borrow::Cow;
use crate::field_mapper::base::LeadingCommaType::Leading;
use crate::field_mapper::base::single_field_mapper::ConnectOp;

pub trait MultiFieldMapper: SingleFieldMapper {
    fn map_to_stream<'a, T>(
        &self,
        fields: T,
        escaper: &dyn KeywordsEscaper,
        indexed: bool,
        leading_comma_type: LeadingCommaType,
        connect_op: ConnectOp,
        is_enum: bool,
    ) -> TokenStream
    where
        T: IntoIterator<Item = &'a FieldDef<'a>>,
    {
        let segments = if indexed {
            MultiFieldMapper::map_static_indexed_fields(self, fields, escaper)
        } else {
            MultiFieldMapper::map_static_fields(self, fields, escaper)
        };
        // treat as not indexed segment
        FieldSeg::from_seg(segments, false).translate(leading_comma_type, connect_op, false, is_enum)
    }
    fn map_static_fields<'a, T>(&self, fields: T, escaper: &dyn KeywordsEscaper) -> Cow<'a, str>
    where
        T: IntoIterator<Item = &'a FieldDef<'a>>,
    {
        let s = fields
            .into_iter()
            .enumerate()
            .map(|(index, field)| SingleFieldMapper::map_static(self, field.as_ref(), escaper))
            .collect::<Vec<Cow<'_, str>>>()
            .join(",");
        Cow::Owned(s)
    }

    fn map_static_indexed_fields<'a, T>(
        &self,
        fields: T,
        escaper: &dyn KeywordsEscaper,
    ) -> Cow<'a, str>
    where
        T: IntoIterator<Item = &'a FieldDef<'a>>,
    {
        let s = fields
            .into_iter()
            .enumerate()
            .map(|(index, field)| {
                SingleFieldMapper::_map_static_indexed(self, field.as_ref(), escaper, index)
            })
            .collect::<Vec<Cow<'_, str>>>()
            .join(",");
        Cow::Owned(s)
    }

    fn map_dynamic_fields<'a, T>(
        &self,
        fields: T,
        escaper: &dyn KeywordsEscaper,
        is_optional: bool,
        indexed: bool,
        comma_type: LeadingCommaType,
        connect_op: ConnectOp,
        is_enum: bool,
    ) -> TokenStream
    where
        T: IntoIterator<Item = &'a FieldDef<'a>>, {
        let mut stream = TokenStream::new();
        for (index, field) in fields.into_iter().enumerate() {
            if index == 0 {
                stream.extend(self.map_single(field, escaper, is_optional, indexed, comma_type, connect_op, is_enum));
            } else {
                stream.extend(self.map_single(field, escaper, is_optional, indexed, Leading, connect_op, is_enum));
            }
        };
        stream
    }

    // fn map_group<'a, T>(
    //     &self,
    //     fields: T,
    //     escaper: &dyn KeywordsEscaper,
    //     indexed: bool,
    //     leading_comma: bool,
    // ) -> TokenStream
    // where
    //     T: IntoIterator<Item = &'a FieldDef<'a>>,
    // {
    //     if indexed {
    //         MultiFieldMapper::map_indexed(self, fields, escaper, leading_comma)
    //     } else {
    //         MultiFieldMapper::map(self, fields, escaper, leading_comma)
    //     }
    // }
    //
    // fn map<'a, T>(
    //     &self,
    //     fields: T,
    //     escaper: &dyn KeywordsEscaper,
    //     leading_comma: bool,
    // ) -> TokenStream
    // where
    //     T: IntoIterator<Item = &'a FieldDef<'a>>,
    // {
    //     let origin = fields
    //         .into_iter()
    //         .enumerate()
    //         .map(|(index, field)| {
    //             SingleFieldMapper::map(self, field.as_ref(), escaper, LeadingCommaType::NoLeading)
    //         })
    //         .collect::<Vec<Cow<'_, str>>>()
    //         .join(",");
    //     let stream = if leading_comma {
    //         format!(",{}", origin)
    //     } else {
    //         origin
    //     };
    //     quote! {
    //         #stream
    //     }
    // }
    //
    // fn map_indexed<'a, T>(
    //     &self,
    //     fields: T,
    //     escaper: &dyn KeywordsEscaper,
    //     leading_comma: bool,
    // ) -> TokenStream
    // where
    //     T: IntoIterator<Item = &'a FieldDef<'a>>,
    // {
    //     let origin = fields
    //         .into_iter()
    //         .enumerate()
    //         .map(|(index, field)| {
    //             let name = SingleFieldMapper::map_indexed(
    //                 self,
    //                 field,
    //                 escaper,
    //                 LeadingCommaType::NoLeading,
    //                 index,
    //             );
    //             name
    //         })
    //         .collect::<Vec<Cow<'_, str>>>()
    //         .join(",");
    //     let stream = if leading_comma {
    //         format!(",{}", origin)
    //     } else {
    //         origin
    //     };
    //     quote! {
    //         #stream
    //     }
    // }

    // fn map_with_leading_comma<'a, T>(&self, fields: T, escaper: &dyn KeywordsEscaper) -> TokenStream
    // where
    //     T: IntoIterator<Item = &'a FieldDef<'a>>,
    // {
    //     let stream = fields
    //         .into_iter()
    //         .map(|field| {
    //             let name = SingleFieldMapper::map(self, field, escaper, LeadingCommaType::Leading);
    //             name
    //         })
    //         .collect::<Vec<Cow<'_, str>>>()
    //         .join("");
    //     quote! {
    //         #stream
    //     }
    // }

    // fn map_indexed_with_leading_comma<'a, T>(
    //     &self,
    //     fields: T,
    //     escaper: &dyn KeywordsEscaper,
    // ) -> TokenStream
    // where
    //     T: IntoIterator<Item = &'a FieldDef<'a>>,
    // {
    //     let stream = fields
    //         .into_iter()
    //         .enumerate()
    //         .map(|(index, field)| {
    //             let name = SingleFieldMapper::map_indexed(
    //                 self,
    //                 field,
    //                 escaper,
    //                 LeadingCommaType::Leading,
    //                 index,
    //             );
    //             name
    //         })
    //         .collect::<Vec<Cow<'_, str>>>()
    //         .join("");
    //     quote! {
    //         #stream
    //     }
    // }
}

impl<T: SingleFieldMapper> MultiFieldMapper for T {}
