use crate::attr_parser::AttrParser;
use crate::field_def::{FieldDef, TableColumnDef};
use crate::info_parser::type_parser::TypeParser;
use crate::LifetimeParser;
use quote::ToTokens;

use crate::info_parser::option_parser::OptionParser;
use crate::info_parser::ParsedField;
use case::CaseExt;
use std::borrow::Cow;
use syn::{Attribute, Field};

// pub struct FieldParser;
// impl FieldParser {
//     pub fn parse(
//         field: &Field,
//         is_enum_variant: bool,
//         unnamed_idx: Option<usize>,
//         external_column_name: Option<String>,
//     ) -> FieldDef<'_> {
//         let struct_field = ParsedField::parse(field, is_enum_variant, unnamed_idx);
//         let table_column = FieldAttrParser::parse(field, external_column_name);
//         FieldDef {
//             struct_field,
//             table_column,
//         }
//     }
// }

pub struct FieldAttrParser;
impl FieldAttrParser {
    // use this function will raise lifetime error
    // fn extract_single_value<'a>(attrs: &'a Vec<NamedAttribute>, name: &'a str)-> Option<Cow<'a, str>> {
    //     let attr_opt = attrs.iter().find(|a| a.name == Cow::Borrowed(name));
    //     attr_opt.map(|a|a.values[0].clone())
    // }

    pub fn parse<'a>(field: &'a Field, external_column_name: Option<String>) -> TableColumnDef<'a> {
        let field_attr = AttrParser::get_attr(&field.attrs, "field");
        let mut table_column_def: TableColumnDef = if let Some(attr) = field_attr {
            FieldAttrParser::parse_field_attr(&attr)
        } else {
            TableColumnDef::default()
        };
        if let Some(column_name) = external_column_name {
            table_column_def.name = Some(Cow::Owned(column_name));
        }
        table_column_def
    }

    fn parse_bool(value: Option<Cow<'_, str>>) -> bool {
        if let Some(value) = value {
            value == "true"
        } else {
            false
        }
    }

    pub fn parse_field_attr<'a>(attr: &'a Attribute) -> TableColumnDef<'a> {
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
