// use crate::sql_generator::KeywordsEscaper;
// use crate::FieldDef;
// use proc_macro2::TokenStream;
// use quote::{format_ident, quote};
// use std::borrow::Cow;
//
// pub trait FieldWrapper {
//     type Escaper: KeywordsEscaper;
//
//     fn get_escaper(&self) -> &Self::Escaper;
//     fn wrap_check_optional<T: AsRef<str>>(
//         field_name: T,
//         origin: TokenStream,
//         check_optional: bool,
//     ) -> TokenStream {
//         let field_ident = format_ident!("{}", field_name.as_ref());
//         if check_optional {
//             quote! {
//                 if self.#field_ident.is_some() {
//                     #origin
//                 }
//             }
//         } else {
//             origin
//         }
//     }
//
//     fn gen_plain_marks(fields: &[FieldDef]) -> String {
//         fields.iter().map(|f| "?").collect::<Vec<&str>>().join(",")
//     }
//     fn gen_indexed_marks(fields: &[FieldDef]) -> String {
//         fields
//             .iter()
//             .enumerate()
//             .map(|(index, _)| format!("${}", index))
//             .collect::<Vec<String>>()
//             .join(",")
//     }
//
//     fn gen_names_string<'a>(&self, fields: &'a [FieldDef]) -> String {
//         fields
//             .iter()
//             .map(|f| f.column_name(self.get_escaper()))
//             .collect::<Vec<Cow<'_, str>>>()
//             .join(",")
//     }
//
//     fn gen_plain_set(&self, fields: &[FieldDef]) -> String {
//         fields
//             .iter()
//             .map(|f| format!("{}=?", f.column_name(self.get_escaper())))
//             .collect::<Vec<String>>()
//             .join(",")
//     }
//     fn gen_indexed_set(&self, fields: &[FieldDef]) -> String {
//         fields
//             .iter()
//             .enumerate()
//             .map(|(index, f)| format!("{}=${}", f.column_name(self.get_escaper()), index))
//             .collect::<Vec<String>>()
//             .join(",")
//     }
// }
