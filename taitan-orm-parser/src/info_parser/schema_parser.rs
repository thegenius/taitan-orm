// use proc_macro::{TokenStream};
use crate::attr_parser::{AttrParser, NamedAttribute};
use case::CaseExt;
use proc_macro2::Span;
use quote::quote;
use std::borrow::Cow;
use syn::{token, Attribute, Data, DataEnum, DataStruct, DataUnion, Error, Fields, FieldsNamed};
// use crate::info_parser::attr_parser::extract_named_attr_val;
// use crate::info_parser::attr_parser::extract_named_attrs_val;

use crate::field_def::FieldDef;
use crate::info_parser::input_parser::InputParser;
use crate::table_def::TableDef;
use crate::FieldParser;



pub fn extract_table_def(struct_name: &str, attrs: &[Attribute], data: &Data) -> TableDef {
    let table_name_attr: Option<Attribute> = AttrParser::get_attr(attrs, "table");

    let table_name: String = if let Some(attr) = table_name_attr {
        if let Some(named_attr) = AttrParser::parse(&attr) {
            if named_attr.values.len() == 1 {
                named_attr.values.first().unwrap().to_string()
            } else {
                panic!("wrong table attribute")
            }
        } else {
            struct_name.to_snake()
        }
    } else {
        struct_name.to_snake()
    };

    let fields = InputParser::get_fields(data);
    let fields_def: Vec<FieldDef> = fields.clone().iter().map(FieldParser::parse).collect();

    let mut table_def = TableDef::default();
    table_def.table_name = Cow::Owned(table_name);
    table_def.columns = fields_def;
    // panic!("{:?}", table_def);
    table_def
}
