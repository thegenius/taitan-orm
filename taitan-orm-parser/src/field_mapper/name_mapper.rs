use crate::field_mapper::{CommaType, FieldWrapper};
use crate::FieldDef;
use proc_macro2::TokenStream;
use quote::quote;

pub trait NameMapper: FieldWrapper {
    fn add_field_name<'a>(
        &'a self,
        field: &'a FieldDef<'a>,
        comma_type: &'a CommaType,
    ) -> TokenStream {
        let column_name = field.column_name(self.get_escaper());
        let column_name_with_comma = format!(",{}", column_name);
        let add_stream = match comma_type {
            CommaType::CheckedComma => {
                quote! {
                    if has_prev {
                        fields.push(',');
                    }
                    fields.push_str(#column_name);
                    has_prev = true;
                }
            }
            CommaType::NoComma => {
                quote! {
                    fields.push_str(#column_name);
                    has_prev = true;
                }
            }
            CommaType::LeadingComma => {
                quote! {
                    fields.push_str(#column_name_with_comma);
                    // has_prev = true;
                }
            }
        };
        Self::wrap_check_optional(
            &field.struct_field.name,
            add_stream,
            field.struct_field.is_optional,
        )
    }


    // batch generate for non-optional fields
    fn add_fields_names<'a>(
        self: &'a Self,
        fields: &'a [FieldDef<'a>],
        comma_type: &'a CommaType,
    ) -> TokenStream {
        let list_string = self.gen_names_string(fields);
        let list_string_with_comma = format!(",{}", list_string);
        match comma_type {
            CommaType::CheckedComma => {
                quote! {
                    if has_prev {
                        fields.push(',');
                    }
                    fields.push_str(#list_string);
                    has_prev = true;
                }
            }
            CommaType::NoComma => {
                quote! {
                    fields.push_str(#list_string);
                    has_prev = true;
                }
            }
            CommaType::LeadingComma => {
                quote! {
                    fields.push_str(#list_string_with_comma);
                    has_prev = true;
                }
            }
        }
    }
}
