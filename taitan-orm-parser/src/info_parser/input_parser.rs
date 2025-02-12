use proc_macro2::Span;
use syn::{token, Data, DataEnum, DataStruct, DataUnion, Error, Field, Fields, FieldsNamed};
pub struct InputParser;

impl InputParser {
    pub fn get_fields_vec(data: &Data) -> syn::Result<Vec<Field>> {
        let named_fields = Self::get_fields_named(data)?;
        Ok(named_fields.named.into_iter().collect())
    }

    pub fn get_fields_named(data: &Data) -> syn::Result<FieldsNamed> {
        let fields = match data {
            Data::Enum(DataEnum {
                enum_token: token::Enum { span },
                ..
            })
            | Data::Union(DataUnion {
                union_token: token::Union { span },
                ..
            }) => {
                return Err(Error::new(*span, "Expected a `struct`"));
            }

            Data::Struct(DataStruct {
                fields: Fields::Named(it),
                ..
            }) => it,

            Data::Struct(_) => {
                return Err(Error::new(
                    Span::call_site(),
                    "Expected a `struct` with named fields",
                ));
            }
        };
        Ok(fields.clone())
    }
}
