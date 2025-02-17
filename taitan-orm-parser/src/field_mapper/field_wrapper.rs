use std::borrow::Cow;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use crate::FieldDef;
use crate::sql_generator::KeywordsEscaper;

pub trait FieldWrapper {
    type Escaper: KeywordsEscaper;

    fn get_escaper(&self) -> &Self::Escaper;
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

    fn gen_plain_marks(fields: &[FieldDef]) -> String {
        fields.iter().map(|f| "?").collect::<Vec<&str>>().join(",")
    }
    fn gen_indexed_marks(fields: &[FieldDef]) -> String {
        fields
            .iter()
            .enumerate()
            .map(|(index, _)| format!("${}", index + 1))
            .collect()
    }
    fn gen_names_string<'a>(&self, fields: &'a [FieldDef]) -> String {
        fields
            .iter()
            .map(|f| f.column_name(self.get_escaper()))
            .collect::<Vec<Cow<'_, str>>>()
            .join(",")
    }
}