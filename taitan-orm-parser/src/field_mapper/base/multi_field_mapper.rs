use super::{KeywordsEscaper, SingleFieldMapper};
use crate::FieldDef;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::borrow::Cow;

pub trait MultiFieldMapper: SingleFieldMapper {
    fn parse_static<'a>(
        &'a self,
        field: &'a FieldDef<'a>,
        escaper: &dyn KeywordsEscaper,
        indexed: bool,
        index: usize,
        group_index: usize,
        first_required_group_index: usize,
    ) -> Cow<'a, str> {
        assert!(!field.struct_field.is_optional);
        assert_eq!(group_index, 0);
        assert_eq!(first_required_group_index, 0);
        if indexed {
            self.map_compile_time(field, escaper, Some(index), index == 0)
        } else {
            self.map_compile_time(field, escaper, None, index == 0)
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
        if indexed {
            stream.extend(quote! {
                index = index + 1;
            });
        }

        if let Some(name) = optional_name {
            assert!(!is_first_required);
            let ident = format_ident!("{}", name);
            quote! {
                if self.#ident.is_some() {
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
    fn parse_dynamic<'a>(
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
                return self.transform(false, indexed, Some(&field.struct_field.name), false, stream);
            }
            if group_index < first_required_group_index {
                // Optional Check Prev
                let stream = self.map_runtime_time(field, escaper, indexed, false);
                return self.transform(true, indexed, Some(&field.struct_field.name), false, stream);
            } else {
                assert_ne!(group_index, first_required_group_index);
                // Optional Leading Comma
                let stream = self.map_runtime_time(field, escaper, indexed, true);
                return self.transform(false, indexed, Some(&field.struct_field.name),false, stream);
            }
        }
    }

    fn map<'a, T>(&self, fields: T, escaper: &dyn KeywordsEscaper) -> TokenStream
    where
        T: IntoIterator<Item = &'a FieldDef<'a>>,
    {
        let stream = fields
            .into_iter()
            .map(|field| {
                let name = SingleFieldMapper::map(self, field.as_ref(), escaper);
                name
            })
            .collect::<Vec<Cow<'_, str>>>()
            .join(",");
        quote! {
            #stream
        }
    }

    fn map_indexed<'a, T>(&self, fields: T, escaper: &dyn KeywordsEscaper) -> TokenStream
    where
        T: IntoIterator<Item = &'a FieldDef<'a>>,
    {
        let stream = fields
            .into_iter()
            .enumerate()
            .map(|(index, field)| {
                let name = SingleFieldMapper::map_indexed(self, field, escaper, index);
                name
            })
            .collect::<Vec<Cow<'_, str>>>()
            .join(",");
        quote! {
            #stream
        }
    }

    fn map_with_leading_comma<'a, T>(&self, fields: T, escaper: &dyn KeywordsEscaper) -> TokenStream
    where
        T: IntoIterator<Item = &'a FieldDef<'a>>,
    {
        let stream = fields
            .into_iter()
            .map(|field| {
                let name = SingleFieldMapper::map_with_leading_comma(self, field, escaper);
                name
            })
            .collect::<Vec<Cow<'_, str>>>()
            .join("");
        quote! {
            #stream
        }
    }
    //
    // fn map_indexed_with_leading_comma<'a, T>(
    //     &self,
    //     fields: T,
    //     escaper: &dyn KeywordsEscaper,
    // ) -> TokenStream
    // where
    //     T: IntoIterator<Item = &'a FieldDef<'a>>,{
    //     let stream = fields
    //         .into_iter()
    //         .enumerate()
    //         .map(|(index, field)| {
    //             let name =
    //                 SingleFieldMapper::map_indexed_with_leading_comma(self, field, escaper, index);
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
