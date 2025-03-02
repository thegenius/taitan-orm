use std::borrow::Cow;
use case::CaseExt;
use syn::DeriveInput;
use crate::attr_parser::{AttrParser, NamedAttribute};
use crate::{FieldDef, FieldParser, InputParser};
use crate::table_def::{translate_attr_groups, NamedFieldsGroup, TableDef};

pub struct SchemaParser;

impl SchemaParser {
    pub fn parse<'a>(input: &'a DeriveInput) -> TableDef<'a> {
        let struct_name = input.ident.to_string();
        let table_name_attr: Option<NamedAttribute> = AttrParser::extract(&input.attrs, "table");
        let table_name = if let Some(attr) = &table_name_attr {
            attr.get_single_value().to_owned()
        } else {
            Cow::Owned(struct_name.to_snake())
        };
        let serde_attr: Option<NamedAttribute> = AttrParser::extract(&input.attrs, "serde_struct");
        let serde_structs = serde_attr.map(|attr| attr.values).unwrap_or_default();

        let fields = InputParser::get_fields(&input.data);
        let attrs = &input.attrs;

        let fields_def: Vec<FieldDef> = fields.iter().map(|f| FieldParser::parse(f, false, None, None)).collect();
        let primary_attr = AttrParser::extract(attrs, "primary");
        let primary_fields = if let Some(attr) = &primary_attr {
            attr.values.clone()
        } else {
            Vec::new()
        };

        let uniques_attrs = AttrParser::extract_multi_list(attrs, "unique");
        let uniques = uniques_attrs
            .into_iter()
            .flat_map(translate_attr_groups)
            .collect();
        let index_attrs = AttrParser::extract_multi_list(attrs, "index");
        let indexes = index_attrs
            .into_iter()
            .flat_map(translate_attr_groups)
            .collect();
        TableDef {
            struct_name: Cow::Owned(struct_name),
            table_name: table_name.clone(),
            serde_structs: serde_structs.clone(),
            fields: fields_def,
            primary_fields,
            uniques,
            indexes,
        }
    }
}
