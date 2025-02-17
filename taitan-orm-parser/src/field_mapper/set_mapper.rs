use proc_macro2::TokenStream;
use quote::quote;
use crate::{FieldDef, FieldWrapper};
use crate::field_mapper::CommaType;

pub trait SetMapper: FieldWrapper {
    fn add_field_set<'a>(
        &'a self,
        field: &'a FieldDef<'a>,
        comma_type: &'a CommaType,
    ) -> TokenStream {
        let column_name = field.column_name(self.get_escaper());
        let set_clause = format!("{}=?", column_name);
        let set_clause_with_comma = format!(",{}=?", column_name);
        let add_stream = match comma_type {
            CommaType::CheckedComma => {
                quote! {
                    if has_prev {
                        clauses.push(',');
                    }
                    clauses.push_str(#set_clause);
                    has_prev = true;
                }
            }
            CommaType::NoComma => {
                quote! {
                    clauses.push_str(#set_clause);
                    has_prev = true;
                }
            }
            CommaType::LeadingComma => {
                quote! {
                    clauses.push_str(#set_clause_with_comma);
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
        &'a self,
        field: &'a FieldDef<'a>,
        comma_type: &'a CommaType,
    ) -> TokenStream {
        let column_name = field.column_name(self.get_escaper());
        let set_clause = format!("{}=?", column_name);
        let set_clause_with_comma = format!(",{}=?", column_name);
        let add_stream = match comma_type {
            CommaType::CheckedComma => {
                quote! {
                    if has_prev {
                        fields.push(',');
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
        &'a self,
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
