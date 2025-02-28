use crate::{FieldDef, FieldParser};
use proc_macro2::Span;
use quote::quote;
use std::borrow::Cow;
use serde::{Deserialize, Serialize};
use syn::{Data, DataEnum, DataStruct, Error, Field, Fields, FieldsNamed};

pub struct InputParser;

#[derive(Debug, Clone)]
pub struct NamedVariant {
    pub name: String,
    pub fields: Vec<Field>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct NamedVariantDef<'a> {
    pub name: String,
    pub fields: Vec<FieldDef<'a>>,
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

    pub fn get_fields_named(data: &Data) -> syn::Result<&FieldsNamed> {
        if let Data::Struct(DataStruct { fields, .. }) = data {
            match fields {
                Fields::Named(it) => Ok(it),
                _ => Err(Error::new(
                    Span::call_site(),
                    "Expected a struct with named fields",
                )),
            }
        } else {
            Err(Error::new(Span::call_site(), "Expected a struct"))
        }
    }

    pub fn is_enum(data: &Data) -> bool {
        if let Data::Enum(DataEnum { .. }) = data {
            true
        } else {
            false
        }
    }

    pub fn is_struct(data: &Data) -> bool {
        if let Data::Struct(DataStruct { .. }) = data {
            true
        } else {
            false
        }
    }

    pub fn get_enum_variant(data: &Data) -> syn::Result<Vec<NamedVariant>> {
        let Data::Enum(DataEnum { ref variants, .. }) = data else {
            return Err(syn::Error::new_spanned(quote! { enum }, "expect enum"));
        };

        let mut named_variants = Vec::new();
        for variant in variants {
            match &variant.fields {
                Fields::Named(fields_named) => {
                    named_variants.push(NamedVariant {
                        name: variant.ident.to_string(),
                        fields: fields_named.clone().named.into_iter().collect(),
                    });
                }
                Fields::Unnamed(fields_unnamed) => {
                    named_variants.push(NamedVariant {
                        name: variant.ident.to_string(),
                        fields: fields_unnamed.clone().unnamed.into_iter().collect(),
                    });
                }
                Fields::Unit => {
                    named_variants.push(NamedVariant {
                        name: variant.ident.to_string(),
                        fields: Vec::new(),
                    });
                }
            }
        }

        if named_variants.is_empty() {
            Err(Error::new_spanned(
                quote! { enum },
                "no variant found in the enum",
            ))
        } else {
            Ok(named_variants)
        }
    }

    pub fn get_enum_variant_defs(data: &Data) -> syn::Result<Vec<NamedVariantDef>> {
        let Data::Enum(DataEnum { ref variants, .. }) = data else {
            return Err(syn::Error::new_spanned(quote! { enum }, "expect enum"));
        };

        let mut named_variants = Vec::new();
        for variant in variants {
            match &variant.fields {
                Fields::Named(fields_named) => {
                    named_variants.push(NamedVariantDef {
                        name: variant.ident.to_string(),
                        fields: fields_named
                            .named
                            .iter()
                            .map(|f| FieldParser::parse(f, true))
                            .collect(),
                    });
                }
                Fields::Unnamed(fields_unnamed) => {
                    named_variants.push(NamedVariantDef {
                        name: variant.ident.to_string(),
                        fields: fields_unnamed
                            .unnamed
                            .iter()
                            .map(|f| FieldParser::parse(f, true))
                            .collect(),
                    });
                }
                Fields::Unit => {
                    named_variants.push(NamedVariantDef {
                        name: variant.ident.to_string(),
                        fields: Vec::new(),
                    });
                }
            }
        }

        if named_variants.is_empty() {
            Err(Error::new_spanned(
                quote! { enum },
                "no variant found in the enum",
            ))
        } else {
            Ok(named_variants)
        }
    }
}
