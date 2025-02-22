use crate::FieldDef;
use proc_macro2::TokenStream;
use quote::quote;

#[derive(Clone, Debug, Default)]
pub struct ArgsMapper;

impl ArgsMapper {
    pub fn map_add_to_args(&self, field: &FieldDef) -> TokenStream {
        let field_name = &field.struct_field.name;
        if field.struct_field.is_optional {
            if field.struct_field.is_location_expr {
                quote! {
                    if let Optional::Some(f) =  &self.#field_name {
                        wrap_encode(args.add(&f.val))?;
                    }
                }
            } else {
                quote! {
                    if let Optional::Some(f) =  &self.#field_name {
                        wrap_encode(args.add(f))?;
                    }
                }
            }
        } else {
            if field.struct_field.is_location_expr {
                quote! {
                    wrap_encode(args.add(&self.#field_name.val))?;
                }
            } else {
                quote! {
                    wrap_encode(args.add(&self.#field_name))?;
                }
            }
        }
    }
}
