use std::borrow::{Borrow, Cow};
use case::CaseExt;
use syn::DeriveInput;
use crate::attr_parser::{AttrParser, NamedAttribute};
use crate::{FieldDef, FieldParser, InputParser};
use crate::table_def::{NamedFieldsGroup, TableDef};

pub struct SchemaParser;

impl SchemaParser {
    pub fn parse(input: &DeriveInput) -> TableDef {
        let struct_name = input.ident.to_string();
        let table_name_attr: Option<NamedAttribute> = AttrParser::extract(&input.attrs, "table");
        let table_name = if let Some(attr) = &table_name_attr {
            attr.get_single_value()
        } else {
            &Cow::Owned(struct_name.to_snake())
        };
        let serde_attr: Option<NamedAttribute> = AttrParser::extract(&input.attrs, "serde_struct");
        let serde_structs = serde_attr.map(|attr| attr.values).unwrap_or_default();

        let fields = InputParser::get_fields(&input.data);
        let attrs = &input.attrs;

        let fields_def: Vec<FieldDef> = fields.iter().map(|f| FieldParser::parse(f)).collect();
        let primary_attr = AttrParser::extract(attrs, "primary").expect("primary attribute missing");
        let uniques_attrs = AttrParser::extract_multi_list(attrs, "unique");
        let uniques = uniques_attrs
            .into_iter()
            .map(NamedFieldsGroup::from)
            .collect();
        let index_attrs = AttrParser::extract_multi_list(attrs, "index");
        let indexes = index_attrs
            .into_iter()
            .map(NamedFieldsGroup::from)
            .collect();
        TableDef {
            struct_name: Cow::Owned(struct_name),
            table_name: table_name.clone(),
            serde_structs,
            fields: fields_def,
            primary_fields: primary_attr.values,
            uniques,
            indexes,
        }
    }
}
