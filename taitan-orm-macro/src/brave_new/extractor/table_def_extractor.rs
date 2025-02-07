use proc_macro::{TokenStream};
use std::borrow::Cow;
use quote::quote;
use syn::{Data, Attribute};
use crate::brave_new::extractor::attr_parser::extract_named_attr_val;
use crate::brave_new::extractor::attr_parser::extract_named_attrs_val;
use crate::brave_new::table_def::TableDef;
use crate::util::extract_fields;

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


    let fields = extract_fields(data).unwrap();


    let mut table_def = TableDef::default();
    table_def.table_name = Cow::Owned(table_name);

    panic!("{:?}", table_def);
    table_def
}