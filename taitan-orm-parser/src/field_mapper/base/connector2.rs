use crate::field_mapper::base::field_group_list2::{FieldGroup, FieldGroupList};

use crate::field_mapper::base::MultiFieldMapper;
use crate::field_mapper::base::SingleFieldMapper;
use crate::{FieldDef, KeywordsEscaper};
use proc_macro2::TokenStream;
use quote::quote;

pub trait Connector2: MultiFieldMapper {
    fn _connect<'a, T>(&self, fields: T, escaper: &dyn KeywordsEscaper) -> TokenStream
    where
        T: IntoIterator<Item = &'a FieldDef<'a>> + Clone,
    {
        let field_group_list = FieldGroupList::from(fields);
        let mut stream = quote! {
            let mut s = String::default();
            let mut has_prev = false;
        };

        let groups = field_group_list.groups;
        for group in groups.iter() {
            match group {
                FieldGroup::LeadingRequired { fields, comma_type } => {
                    stream.extend(self.map_to_stream(fields, escaper, false, *comma_type));
                    stream.extend(quote! { has_prev = true; });
                }

                FieldGroup::LeadingFailRequired { fields, comma_type }
                | FieldGroup::TrailingRequired { fields, comma_type } => {
                    stream.extend(self.map_to_stream(fields, escaper, false, *comma_type));
                }

                FieldGroup::LeadingOptional { field, comma_type }
                | FieldGroup::FollowingOptional { field, comma_type }
                | FieldGroup::TrailingOptional { field, comma_type } => {
                    stream.extend(self.map_single_optional(field, escaper, false, *comma_type));
                }
            }
        }
        quote! {
            {
                #stream
                s
            }
        }
    }

    fn _connect_indexed<'a, T>(&self, fields: T, escaper: &dyn KeywordsEscaper) -> TokenStream
    where
        T: IntoIterator<Item = &'a FieldDef<'a>> + Clone,
    {
        let field_group_list = FieldGroupList::from(fields);
        let mut stream = quote! {
            let mut s = String::default();
            let mut has_prev = false;
            let mut index = 0;
        };

        let groups = field_group_list.groups;
        for group in groups.iter() {
            match group {
                FieldGroup::LeadingRequired { fields, comma_type } => {
                    stream.extend(self.map_to_stream(fields, escaper, true, *comma_type));
                    stream.extend(quote! { has_prev = true; });
                    let len = fields.len();
                    stream.extend(quote! { index += #len; });
                }

                FieldGroup::LeadingFailRequired { fields, comma_type }
                | FieldGroup::TrailingRequired { fields, comma_type } => {
                    let s = self.map_dynamic_fields(fields, escaper, false, true, *comma_type);
                    stream.extend(s)
                }

                FieldGroup::LeadingOptional { field, comma_type }
                | FieldGroup::FollowingOptional { field, comma_type }
                | FieldGroup::TrailingOptional { field, comma_type } => {
                    stream.extend(self.map_single_optional(field, escaper, true, *comma_type));
                }
            }
        }
        quote! {
            {
                #stream
                s
            }
        }
    }

    fn _connect_expr<'a, T>(
        &self,
        fields: T,
        escaper: &dyn KeywordsEscaper,
        indexed: bool,
    ) -> TokenStream
    where
        T: IntoIterator<Item = &'a FieldDef<'a>> + Clone,
    {
        let field_group_list = FieldGroupList::from(fields);
        let mut stream = quote! {
            let mut s = String::default();
            let mut has_prev = false;
        };
        if indexed {
            stream.extend(quote! { let mut index = 0; });
        }

        let groups = field_group_list.groups;
        for group in groups.iter() {
            match group {
                FieldGroup::LeadingRequired { fields, comma_type } => {
                    let s = self.map_dynamic_fields(fields, escaper, false, indexed, *comma_type);
                    stream.extend(s);
                    stream.extend(quote! { has_prev = true; });
                }

                FieldGroup::LeadingFailRequired { fields, comma_type }
                | FieldGroup::TrailingRequired { fields, comma_type } => {
                    let s = self.map_dynamic_fields(fields, escaper, false, indexed, *comma_type);
                    stream.extend(s);
                }

                FieldGroup::LeadingOptional { field, comma_type }
                | FieldGroup::FollowingOptional { field, comma_type }
                | FieldGroup::TrailingOptional { field, comma_type } => {
                    stream.extend(self.map_single_optional(field, escaper, false, *comma_type));
                }
            }
        }
        quote! {
            {
                #stream
                s
            }
        }
    }
}

impl<T: MultiFieldMapper> Connector2 for T {}
