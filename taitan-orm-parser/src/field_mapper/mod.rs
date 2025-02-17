use crate::sql_generator::KeywordsEscaper;
use crate::FieldDef;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use std::borrow::Cow;

pub struct FieldMapper;

pub enum CommaType {
    NoComma,
    LeadingComma,
    CheckedComma,
}
impl CommaType {
    pub fn parse(index: usize, first_required_index: usize) -> Self {
        if index == first_required_index {
            CommaType::NoComma
        } else if index < first_required_index {
            CommaType::CheckedComma
        } else {
            CommaType::LeadingComma
        }
    }
}

impl FieldMapper {
    pub fn add_name<'a>(
        field: &'a FieldDef<'a>,
        escaper: &'a dyn KeywordsEscaper,
        comma_type: &'a CommaType,
        check_optional: bool,
    ) -> TokenStream {
        let column_name = field.column_name(escaper);
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
        Self::wrap_check_optional(&field.struct_field.name, add_stream, check_optional)
    }

    pub fn add_mark<'a>(
        field: &'a FieldDef<'a>,
        comma_type: &'a CommaType,
        check_optional: bool,
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
        Self::wrap_check_optional(&field.struct_field.name, add_stream, check_optional)
    }

    pub fn add_indexed_mark<'a>(
        field: &'a FieldDef<'a>,
        comma_type: &'a CommaType,
        check_optional: bool,
    ) -> TokenStream {
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
        Self::wrap_check_optional(&field.struct_field.name, add_stream, check_optional)
    }

    pub fn add_names<'a>(
        fields: &'a [FieldDef<'a>],
        escaper: &'a dyn KeywordsEscaper,
        comma_type: &'a CommaType,
    ) -> TokenStream {
        let list_string = Self::gen_names_string(fields, escaper);
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

    pub fn add_marks<'a>(
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

    fn wrap_check_optional<T: AsRef<str>>(
        field_name: T,
        origin: TokenStream,
        check_optional: bool,
    ) -> TokenStream {
        let field_ident = format_ident!("{}", field_name.as_ref());
        if check_optional {
            quote! {
                if self.#field_ident.is_some() {
                    #origin
                }
            }
        } else {
            origin
        }
    }

    pub fn gen_plain_marks(fields: &[FieldDef]) -> String {
        fields.iter().map(|f| "?").collect::<Vec<&str>>().join(",")
    }
    pub fn gen_indexed_marks(fields: &[FieldDef]) -> String {
        fields
            .iter()
            .enumerate()
            .map(|(index, _)| format!("${}", index + 1))
            .collect()
    }
    pub fn gen_names_string<'a>(fields: &'a [FieldDef], escaper: &'a dyn KeywordsEscaper) -> String {
        fields
            .iter()
            .map(|f| f.column_name(escaper))
            .collect::<Vec<Cow<'_, str>>>()
            .join(",")
    }
}
