use crate::field_mapper::field_wrapper::FieldWrapper;
use crate::field_mapper::CommaType;
use crate::FieldDef;
use proc_macro2::TokenStream;
use quote::quote;

pub trait MarkMapper: FieldWrapper {
    fn add_field_mark<'a>(
        field: &'a FieldDef<'a>,
        comma_type: &'a CommaType,
    ) -> TokenStream {
        let add_stream = match comma_type {
            CommaType::CheckedComma => {
                quote! {
                    if has_prev {
                        marks.push(',');
                    }
                    marks.push('?');
                    has_prev = true;
                }
            }
            CommaType::NoComma => {
                quote! {
                    marks.push('?');
                    has_prev = true;
                }
            }
            CommaType::LeadingComma => {
                quote! {
                    marks.push_str(",?");
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

    fn add_field_indexed_mark<'a>(
        field: &'a FieldDef<'a>,
        comma_type: &'a CommaType,
    ) -> TokenStream {
        let add_stream = match comma_type {
            CommaType::CheckedComma => {
                quote! {
                    if has_prev {
                        marks.push(',');
                    }
                    marks.push_str(format!("${}", index));
                    index = index + 1;
                    has_prev = true;
                }
            }
            CommaType::NoComma => {
                quote! {
                    marks.push_str(format!("${}", index));
                    index = index + 1;
                    has_prev = true;
                }
            }
            CommaType::LeadingComma => {
                quote! {
                    marks.push_str(format!(",${}", index));
                    index = index + 1;
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

    fn add_fields_marks<'a>(
        fields: &'a [FieldDef<'a>],
        comma_type: &'a CommaType,
        indexed: bool,
    ) -> TokenStream {
        let marks = if indexed {
            Self::gen_indexed_marks(fields)
        } else {
            Self::gen_plain_marks(fields)
        };
        let marks_with_comma = format!(",{}", marks);
        match comma_type {
            CommaType::CheckedComma => {
                quote! {
                    if has_prev {
                        marks.push(',');
                    }
                    marks.push_str(#marks);
                    has_prev = true;
                }
            }
            CommaType::NoComma => {
                quote! {
                    marks.push_str(#marks);
                    has_prev = true;
                }
            }
            CommaType::LeadingComma => {
                quote! {
                    marks.push_str(#marks_with_comma);
                    has_prev = true;
                }
            }
        }
    }
}
