use std::borrow::Cow;
use proc_macro2::Span;
use quote::quote;
use syn::{Data, DataEnum, DataStruct, Error, Field, Fields, FieldsNamed};
pub struct InputParser;


#[derive(Debug, Clone)]
pub struct NamedVariant<'a> {
    pub name: Cow<'a, str>,
    pub fields: Vec<Field>,
}

impl InputParser {
    pub fn get_fields<'a>(data: &'a Data) -> Vec<&'a Field> {
        let named_fields_result = Self::get_fields_named(data);
        if let Ok(named_fields) = named_fields_result {
            named_fields.named.iter().collect()
        } else {
            Vec::new()
        }
    }

    // pub fn get_fields_named(data: &Data) -> syn::Result<FieldsNamed> {
    //     let fields = match data {
    //         Data::Enum(DataEnum {
    //             enum_token: token::Enum { span },
    //             ..
    //         })
    //         | Data::Union(DataUnion {
    //             union_token: token::Union { span },
    //             ..
    //         }) => {
    //             return Err(Error::new(*span, "Expected a `struct`"));
    //         }
    //
    //         Data::Struct(DataStruct {
    //             fields: Fields::Named(it),
    //             ..
    //         }) => it,
    //
    //         Data::Struct(_) => {
    //             return Err(Error::new(
    //                 Span::call_site(),
    //                 "Expected a `struct` with named fields",
    //             ));
    //         }
    //     };
    //     Ok(fields.clone())
    // }

    pub fn get_fields_named<'a>(data: &'a Data) -> syn::Result<&'a FieldsNamed> {
        if let Data::Struct(DataStruct { fields, .. }) = data {
            match fields {
                Fields::Named(it) => Ok(it),
                _ => Err(Error::new(
                    Span::call_site(),
                    "Expected a struct with named fields",
                )),
            }
        } else {
            Err(Error::new(
                Span::call_site(),
                "Expected a struct",
            ))
        }
    }

    pub fn get_enum_variant(data: &Data) -> syn::Result<Vec<NamedVariant>> {
        let Data::Enum(DataEnum { ref variants, .. }) = data else {
            return Err(syn::Error::new_spanned(
                quote! { enum },
                "expected enum data"
            ));
        };

        let mut named_variants = Vec::new();
        for variant in variants {
            if let Fields::Named(fields_named) = &variant.fields {
                named_variants.push(NamedVariant {
                    name: Cow::Owned(variant.ident.to_string()),
                    fields: fields_named.clone().named.into_iter().collect(),
                });
            }
        }

        if named_variants.is_empty() {
            Err(Error::new_spanned(
                quote! { enum },
                "no named fields variant found in the enum"
            ))
        } else {
            Ok(named_variants)
        }
    }
}
