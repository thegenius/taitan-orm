use proc_macro2::{self, Ident, Span};
use quote::quote;

use crate::attrs::{AttrParser, DefaultAttrParser};
use crate::fields::{DefaultFieldMapper, FieldMapType, FieldMapper};
use crate::types::{DefaultTypeChecker, TypeChecker};
use syn::{parse_str, token::{self}, Data, DataEnum, DataStruct, DataUnion, Error, Field, Fields, FieldsNamed, Path, Result};


pub fn create_path_from_str(path_str: &str) -> Path {
    parse_str(path_str).expect("Path should be valid")
}

pub fn build_fields_name(fields: &Vec<Field>) -> Vec<proc_macro2::TokenStream> {
    DefaultFieldMapper::map_field_vec(fields, &|field: Field| {
        DefaultFieldMapper::map_field(field, FieldMapType::String)
    })
}

pub fn build_fields_name_with_option(fields: &Vec<Field>) -> Vec<proc_macro2::TokenStream> {
    let mut result: Vec<proc_macro2::TokenStream> = Vec::new();
    result.push(quote!(
        let mut fields: Vec<String> = Vec::new();
    ));
    let push_stmt = DefaultFieldMapper::map_field_vec(fields, &|field: Field| {
        if DefaultTypeChecker::field_is_option(&field) {
            DefaultFieldMapper::map_field(field, FieldMapType::OptionNamePush)
        } else {
            DefaultFieldMapper::map_field(field, FieldMapType::NamePush)
        }
    });
    result.extend(push_stmt);
    result.push(quote!(fields));
    return result;
}

pub fn build_args_add_clause(fields: &Vec<Field>, cloned: bool) -> Vec<proc_macro2::TokenStream> {
    if cloned {
        DefaultFieldMapper::map_field_vec(fields, &|f: Field| {
            DefaultFieldMapper::map_field(f, FieldMapType::ArgsAddClone)
        })
    } else {
        DefaultFieldMapper::map_field_vec(fields, &|f: Field| {
            DefaultFieldMapper::map_field(f, FieldMapType::ArgsAdd)
        })
    }
}
pub fn build_args_add_ref_clause_by_vec(fields: &Vec<Field>) -> Vec<proc_macro2::TokenStream> {
    DefaultFieldMapper::map_field_vec(fields, &|f: Field| {
        DefaultFieldMapper::map_field(f, FieldMapType::ArgsAddClone)
    })
}

pub fn build_args_push_clause(fields: &FieldsNamed) -> Vec<proc_macro2::TokenStream> {
    DefaultFieldMapper::map_fields(fields, &|f: Field| {
        DefaultFieldMapper::map_field(f, FieldMapType::ArgsAdd)
    })
}

pub fn build_args_add_ref_clause(fields: &FieldsNamed) -> Vec<proc_macro2::TokenStream> {
    DefaultFieldMapper::map_fields(fields, &|f: Field| {
        DefaultFieldMapper::map_field(f, FieldMapType::ArgsAddRef)
    })
}

pub fn build_args_add_option_ref_clause(fields: &FieldsNamed) -> Vec<proc_macro2::TokenStream> {
    DefaultFieldMapper::map_fields(fields, &|f: Field| {
        DefaultFieldMapper::map_field(f, FieldMapType::ArgsAddOptionRef)
    })
}

pub fn build_args_add_option_location_ref_clause(
    fields: &FieldsNamed,
) -> Vec<proc_macro2::TokenStream> {
    DefaultFieldMapper::map_fields(fields, &|f: Field| {
        DefaultFieldMapper::map_field(f, FieldMapType::ArgsAddOptionLocationRef)
    })
}

pub fn gen_args_add_maybe_option(fields: &Vec<Field>) -> Vec<proc_macro2::TokenStream> {
    let mut result: Vec<proc_macro2::TokenStream> = Vec::new();
    let add_clause = DefaultFieldMapper::map_field_vec(fields, &|field: Field| {
        if DefaultTypeChecker::field_is_option(&field) {
            DefaultFieldMapper::map_field(field, FieldMapType::ArgsAddOptionRef)
        } else {
            DefaultFieldMapper::map_field(field, FieldMapType::ArgsAddRef)
        }
    });
    result.extend(add_clause);
    result
}

pub fn extract_enum(data: &Data) -> Result<&DataEnum> {
    match data {
        Data::Enum(data_enum) => Ok(data_enum),

        _ => Err(Error::new(
            Span::call_site(),
            "Expected a `struct` with named fields",
        )),
    }
}

pub fn extract_enum_variants(data: &Data) -> Result<Vec<Ident>> {
    let data_enum = extract_enum(data)?;
    let mut variant_names = Vec::new();
    for variant in &data_enum.variants {
        let variant_name: Ident = variant.ident.clone();
        variant_names.push(variant_name);
    }
    Ok(variant_names)
}

/**
extract FieldsNamed from struct,
throw Error when annotated on enum/union/struct-with-anonymous
*/
pub fn extract_fields(data: &Data) -> Result<FieldsNamed> {
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

pub fn extract_selected_fields_name(fields: &FieldsNamed) -> Vec<proc_macro2::TokenStream> {
    DefaultFieldMapper::map_fields(fields, &|field: Field| {
        DefaultFieldMapper::map_field(field, FieldMapType::BoolPush)
    })
}
pub fn extract_order_by_fields_name(fields: &FieldsNamed) -> Vec<proc_macro2::TokenStream> {
    DefaultFieldMapper::map_fields(fields, &|field: Field| {
        DefaultFieldMapper::map_field(field, FieldMapType::BoolPush)
    })
}

pub fn extract_fields_name_str(fields: &FieldsNamed) -> Vec<proc_macro2::TokenStream> {
    DefaultFieldMapper::map_fields(fields, &|field: Field| {
        DefaultFieldMapper::map_field(field, FieldMapType::Str)
    })
}

pub fn extract_fields_name(fields: &FieldsNamed) -> Vec<proc_macro2::TokenStream> {
    DefaultFieldMapper::map_fields(fields, &|field: Field| {
        DefaultFieldMapper::map_field(field, FieldMapType::String)
    })
}

pub fn extract_annotated_fields(fields: &FieldsNamed, name: &str) -> Vec<Field> {
    let mut result: Vec<Field> = Vec::new();
    let cloned_named: FieldsNamed = fields.clone();
    for field in cloned_named.named.into_iter() {
        let has_attr = DefaultAttrParser::check_has_attr(&field.attrs, name);
        if has_attr {
            result.push(field.clone());
        }
    }
    return result;
}

pub fn extract_not_annotated_fields(fields: &FieldsNamed, name: &str) -> Vec<Field> {
    let mut result: Vec<Field> = Vec::new();
    let cloned_named: FieldsNamed = fields.clone();
    for field in cloned_named.named.into_iter() {
        let has_attr = DefaultAttrParser::check_has_attr(&field.attrs, name);
        if !has_attr {
            result.push(field.clone());
        }
    }
    return result;
}
