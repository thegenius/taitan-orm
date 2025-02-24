use super::KeywordsEscaper;
use crate::FieldDef;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use std::borrow::Cow;

#[derive(Clone, Debug, Copy)]
pub enum LeadingCommaType {
    Leading,
    NoLeading,
    CheckedLeading,
}

pub enum FieldSeg<'a> {
    // name, name=VALUES(name), ?, name=?, name{}?
    Seg(Cow<'a, str>),
    // ${}, name=${}, name{}${}
    IndexedSeg(Cow<'a, str>),
}

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
                self.map_indexed(field, escaper, LeadingCommaType::Leading, index)
            } else {
                self.map_indexed(field, escaper, LeadingCommaType::NoLeading, index)
            }
        } else {
            if leading_comma {
                self.map(field, escaper, LeadingCommaType::Leading)
            } else {
                self.map(field, escaper, LeadingCommaType::NoLeading)
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
                self.map_dynamic(field, escaper, LeadingCommaType::Leading, false)
            } else {
                self.map_dynamic(field, escaper, LeadingCommaType::NoLeading, false)
            }
        }
    }

    fn transform(
        &self,
        check_prev: bool,
        indexed: bool,
        optional_name: Option<&str>,
        is_first_required: bool,
        origin: TokenStream,
    ) -> TokenStream {
        // release mode, "xxx".as_ref() will be optimized to "xxx"
        // so add #origin.as_ref() will be zero overhead
        let mut stream = if check_prev {
            quote! {
                if has_prev {
                    s.push(',');
                } else {
                    has_prev = true;
                }
                s.push_str(#origin.as_ref());
            }
        } else {
            quote! {
                s.push_str(#origin.as_ref());
            }
        };

        // if dynamic indexed, add index + 1
        if indexed {
            stream.extend(quote! {
                index = index + 1;
            });
        }

        // if is optional, wrap it
        if let Some(name) = optional_name {
            assert!(!is_first_required);
            let ident = format_ident!("{}", name);
            quote! {
                if !self.#ident.is_none() {
                    #stream
                }
            }
        } else {
            if is_first_required {
                quote! {
                    #stream
                    has_prev = true;
                }
            } else {
                stream
            }
        }
    }

    fn transform_dynamic<'a>(
        &'a self,
        field: &'a FieldDef<'a>,
        escaper: &dyn KeywordsEscaper,
        indexed: bool,
        index: usize,
        group_index: usize,
        first_required_group_index: usize,
    ) -> TokenStream {
        if !field.struct_field.is_optional {
            if index == 0 && group_index == 0 {
                assert_eq!(first_required_group_index, 0);
                // Required;
                let stream = self.map_runtime_time(field, escaper, indexed, false);
                return self.transform(false, indexed, None, true, stream);
            }
            if index == 0 && group_index <= first_required_group_index {
                assert_eq!(group_index, first_required_group_index);
                assert_ne!(group_index, 0);
                // RequiredCheckPrev;
                let stream = self.map_runtime_time(field, escaper, indexed, false);
                return self.transform(true, indexed, None, true, stream);
            }
            // RequiredLeadingComma
            let stream = self.map_runtime_time(field, escaper, indexed, true);
            return self.transform(false, indexed, None, false, stream);
        } else {
            if index == 0 && group_index == 0 {
                assert_ne!(first_required_group_index, 0);
                // StreamType::Optional;
                let stream = self.map_runtime_time(field, escaper, indexed, false);
                return self.transform(
                    false,
                    indexed,
                    Some(&field.struct_field.name),
                    false,
                    stream,
                );
            }
            if group_index < first_required_group_index {
                // Optional Check Prev
                let stream = self.map_runtime_time(field, escaper, indexed, false);
                return self.transform(
                    true,
                    indexed,
                    Some(&field.struct_field.name),
                    false,
                    stream,
                );
            } else {
                assert_ne!(group_index, first_required_group_index);
                // Optional Leading Comma
                let stream = self.map_runtime_time(field, escaper, indexed, true);
                return self.transform(
                    false,
                    indexed,
                    Some(&field.struct_field.name),
                    false,
                    stream,
                );
            }
        }
    }



    fn map<'a>(
        &'a self,
        field: &'a FieldDef<'a>,
        escaper: &dyn KeywordsEscaper,
        leading_comma: LeadingCommaType,
    ) -> Cow<'a, str>;

    fn map_indexed<'a>(
        &'a self,
        field: &'a FieldDef<'a>,
        escaper: &dyn KeywordsEscaper,
        leading_comma: LeadingCommaType,
        index: usize,
    ) -> Cow<'a, str>;

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

    fn map_dynamic(
        &self,
        field: &FieldDef,
        escaper: &dyn KeywordsEscaper,
        leading_comma_type: LeadingCommaType,
        indexed: bool,
    ) -> TokenStream;

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
