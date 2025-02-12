// use proc_macro::{TokenStream};
use std::borrow::Cow;
use proc_macro2::Span;
use quote::quote;
use syn::{Data, Attribute, FieldsNamed, DataEnum, token, DataUnion, DataStruct, Error, Fields};
use crate::info_parser::attr_parser::extract_named_attr_val;
use crate::info_parser::attr_parser::extract_named_attrs_val;

use crate::info_parser::input_parser::InputParser;
use crate::field_def::FieldDef;
use crate::FieldParser;
use crate::table_def::TableDef;


pub fn extract_table_def(struct_name: &str, attrs: &[Attribute], data: &Data) -> TableDef {
    let table_names = extract_named_attrs_val(attrs, "table");
    if table_names.len() > 1 {
        panic!("`table` attribute does not have a single attribute");
    }
    let table_name = if table_names.len() == 1 {
        table_names[0].clone()
    } else {
        struct_name.to_string()
    };


    let fields = InputParser::get_fields_vec(data).unwrap();
    let fields_def: Vec<FieldDef> = fields.clone().iter().map(FieldParser::parse).collect();


    let mut table_def = TableDef::default();
    table_def.table_name = Cow::Owned(table_name);
    table_def.columns = fields_def;
    // panic!("{:?}", table_def);
    table_def
}

