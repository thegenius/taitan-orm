use crate::FieldDef;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

#[derive(Clone, Debug, Default)]
pub struct RowMapper;

impl RowMapper {
    pub fn map_row_try_get(&self, field: &FieldDef) -> TokenStream {
        let field_name = &field.struct_field.get_name();
        let field_ident = format_ident!("{}", field_name);
        if field.struct_field.is_option() {
            quote! {
                if let Some(_) = &selection.#field_ident {
                    selected.#field_ident = Some(sqlx::Row::try_get(&row, i)?);
                    i += 1;
                }
            }
        } else {
            quote! {
                selected.#field_ident = Some(sqlx::Row::try_get(&row, i)?);
                i += 1;
            }
        }
    }
    pub fn map_selected_default(&self, field: &FieldDef) -> TokenStream {
        let field_name = &field.struct_field.get_name();
        let field_ident = format_ident!("{}", field_name);
        quote! {
            #field_ident: Some(None),
        }
    }
}
