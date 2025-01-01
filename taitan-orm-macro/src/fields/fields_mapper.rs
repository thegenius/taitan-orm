use proc_macro2::TokenStream;
use quote::quote;
use syn::Field;
use crate::fields::{DefaultFieldMapper, FieldMapType, FieldMapper, FieldsContainer, FieldsParser};
use crate::types::{DefaultTypeChecker, TypeChecker};

pub trait FieldsMapper: FieldsContainer {
    fn get_maybe_option_name_vec(&self) -> TokenStream;
}

impl FieldsMapper for FieldsParser {

    fn get_maybe_option_name_vec(&self) -> TokenStream {
        let tokens =
            DefaultFieldMapper::map_field_vec(self.get_fields(), &|field: Field| {
                let field_type = &field.ty;
                if DefaultTypeChecker::type_is_option(field_type) {
                    DefaultFieldMapper::map_field(
                        field,
                        FieldMapType::OptionNamePush,
                    )
                } else {
                    DefaultFieldMapper::map_field(field, FieldMapType::NamePush)
                }
            });
        quote!(
            let mut fields = Vec::new();
            #(#tokens;)*
            return fields;
        )
    }
}