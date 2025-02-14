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




pub fn extract_table_def<'a>(struct_name: &'a str, attrs: &'a [Attribute], data: &'a Data) -> TableDef<'a> {


    let table_name_attr: Option<&Attribute> = AttrParser::get_attr(attrs, "table");
    let table_name = if let Some(attr) = table_name_attr {
        AttrParser::parse_one_single(&attr).get_single_value().to_string()
    } else {
        struct_name.to_snake()
    };

    let fields = InputParser::get_fields(data);
    let fields_def: Vec<FieldDef<'a>> = fields.iter().map(|f|FieldParser::parse(*f)).collect();

    let mut table_def = TableDef::default();
    // table_def.table_name = Cow::Owned(table_name);
    // table_def.columns = fields_def;
    // panic!("{:?}", table_def);
    table_def
}
