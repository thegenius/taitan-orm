use crate::FieldDef;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

#[derive(Clone, Debug, Default)]
pub struct ArgsMapper;

impl ArgsMapper {
    pub fn map_add_to_args(&self, field: &FieldDef) -> TokenStream {
        let field_name = &field.struct_field.get_name();
        let field_ident = format_ident!("{}", field_name);
        if field.struct_field.is_option() {
            if field.struct_field.is_location_expr {
                quote! {
                    if let Some(f) =  &self.#field_ident {
                        sqlx::Arguments::add(args, &f.val)?
                    }
                }
            } else {
                quote! {
                    if let Some(f) =  &self.#field_ident {
                        sqlx::Arguments::add(args, f)?
                    }
                }
            }
        } else {
            if field.struct_field.is_location_expr {
                quote! {
                    sqlx::Arguments::add(args, &self.#field_ident.val)?
                }
            } else {
                quote! {
                    sqlx::Arguments::add(args, &self.#field_ident)?
                }
            }
        }
    }
}
