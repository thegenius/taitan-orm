use crate::FieldDef;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

#[derive(Clone, Debug, Default)]
pub struct ArgsMapper;

impl ArgsMapper {
    pub fn map_add_to_args(&self, field: &FieldDef) -> TokenStream {
        let field_name = &field.struct_field.name;
        let field_ident = format_ident!("{}", field_name);
        if field.struct_field.is_optional {
            if field.struct_field.is_location_expr {
                quote! {
                    if let Optional::Some(f) =  &self.#field_ident {
                        taitan_orm_trait::brave_new::error::wrap_encode(sqlx::Arguments::add(args, &f.val))?;
                    }
                }
            } else {
                quote! {
                    if let Optional::Some(f) =  &self.#field_ident {
                        taitan_orm_trait::brave_new::error::wrap_encode(sqlx::Arguments::add(args, f))?;
                    }
                }
            }
        } else {
            if field.struct_field.is_location_expr {
                quote! {
                    taitan_orm_trait::brave_new::error::wrap_encode(sqlx::Arguments::add(args, &self.#field_ident.val))?;
                }
            } else {
                quote! {
                    taitan_orm_trait::brave_new::error::wrap_encode(sqlx::Arguments::add(args, &self.#field_ident))?;
                }
            }
        }
    }
}
