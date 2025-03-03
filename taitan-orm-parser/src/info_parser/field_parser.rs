use crate::attr_parser::AttrParser;
use crate::field_def::{FieldDef, FieldName, StructFieldDef, TableColumnDef};
use crate::info_parser::type_parser::TypeParser;
use crate::LifetimeParser;
use quote::ToTokens;

use std::borrow::Cow;
use syn::{Attribute, Field};

pub struct FieldParser;
impl FieldParser {
    pub fn parse(field: &Field, is_enum_variant: bool, unnamed_idx: Option<usize>, column_name: Option<String>) -> FieldDef<'_> {
        let field_name = if let Some(ident) = field.clone().ident {
            FieldName::Named(Cow::Owned(ident.to_string()))
        } else {
            FieldName::unnamed(unnamed_idx.unwrap())
        };
        let field_type = TypeParser::get_inner_type(&field.ty);
        let field_type_str = field_type.to_token_stream().to_string();
        let is_location_expr = TypeParser::is_location_expr(&field_type);

        let is_optional = TypeParser::is_option(&field.ty);
        let lifetime = LifetimeParser::get_lifetime(&field.ty).map(|l| Cow::Owned(l.to_string()));

        let struct_field = StructFieldDef {
            name: field_name.clone(),
            rust_type: Cow::Owned(field_type_str.clone()),
            is_optional,
            is_enum_variant,
            is_location_expr,
            lifetime,
            field: Some(field.clone())
        };

        let attrs = &field.attrs;
        let field_attr = AttrParser::get_attr(&attrs, "field");
        let table_column = if let Some(attr) = field_attr {
            FieldAttrParser::parse(&attr)
        } else {
            if let Some(column_name) = column_name {
                TableColumnDef {
                    name: Some(Cow::Owned(column_name)),
                   ..Default::default()
                }
            } else {
                TableColumnDef::default()
            }
        };

        FieldDef {
            struct_field,
            table_column,
        }
    }
}

pub struct FieldAttrParser;
impl FieldAttrParser {
    // use this function will raise lifetime error
    // fn extract_single_value<'a>(attrs: &'a Vec<NamedAttribute>, name: &'a str)-> Option<Cow<'a, str>> {
    //     let attr_opt = attrs.iter().find(|a| a.name == Cow::Borrowed(name));
    //     attr_opt.map(|a|a.values[0].clone())
    // }

    fn parse_bool(value: Option<Cow<'_, str>>) -> bool {
        if let Some(value) = value {
            value == "true"
        } else {
            false
        }
    }

    pub fn parse<'a>(attr: &'a Attribute) -> TableColumnDef<'a> {
        if !AttrParser::is_attr(attr, "field") {
            panic!("attribute is not named as #[field]")
        }
        let named_attrs = AttrParser::parse_list(attr);
        let name_attr = named_attrs.iter().find(|a| a.name == Cow::Borrowed("name"));
        let name = name_attr.map(|a| a.values[0].clone());

        let name_attr = named_attrs
            .iter()
            .find(|a| a.name == Cow::Borrowed("column_type"));
        let column_type = name_attr.map(|a| a.values[0].clone());

        let default_attr = named_attrs
            .iter()
            .find(|a| a.name == Cow::Borrowed("default_value"));
        let default_value = default_attr.map(|a| a.values[0].clone());

        let generated_attr = named_attrs
            .iter()
            .find(|a| a.name == Cow::Borrowed("generated"));
        let generated = generated_attr.map(|a| a.values[0].clone());

        let nullable_attr = named_attrs
            .iter()
            .find(|a| a.name == Cow::Borrowed("nullable"));
        let nullable_opt = nullable_attr.map(|a| a.values[0].clone());
        let nullable = Self::parse_bool(nullable_opt);

        let auto_inc_attr = named_attrs
            .iter()
            .find(|a| a.name == Cow::Borrowed("auto_inc"));
        let auto_inc_opt = auto_inc_attr.map(|a| a.values[0].clone());
        let auto_inc = Self::parse_bool(auto_inc_opt);

        TableColumnDef {
            name,
            column_type,
            default_value,
            nullable,
            generated,
            auto_inc,
        }
    }
}
