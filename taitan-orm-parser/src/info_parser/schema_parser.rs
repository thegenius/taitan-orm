// use proc_macro::{TokenStream};
use crate::attr_parser::{AttrParser, NamedAttribute, NamedAttributes};
use case::CaseExt;
use proc_macro2::Span;
use quote::quote;
use serde::{Deserialize, Serialize};
use std::borrow::{Borrow, Cow};
use std::collections::HashMap;
use syn::{
    token, Attribute, Data, DataEnum, DataStruct, DataUnion, DeriveInput, Error, Fields,
    FieldsNamed,
};

use crate::field_def::FieldDef;
use crate::info_parser::input_parser::InputParser;
use crate::table_def::{NamedFieldsGroup, TableDef};
use crate::FieldParser;
use crate::info_parser::index_parser::IndexParser;

pub struct SchemaParser;

// // #[table(user)]
// // #[serde_struct(entity, location)]
// // #[primary(id)]
// // #[unique(name = xxx, fields = (f1, f2))]
// // #[index(name = xxx, fields = (f1, f2, f3))]
// #[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
// pub struct TableDef<'a> {
//     pub table_name: Cow<'a, str>,
//     pub serde_structs: Vec<String>,
//     pub fields: Vec<FieldDef<'a>>,
//     pub primary_key_fields: Vec<FieldDef<'a>>,
//     pub non_primary_key_fields: Vec<FieldDef<'a>>,
//     pub unique_keys: HashMap<String, Vec<FieldDef<'a>>>,
//     pub index_key: HashMap<String, Vec<FieldDef<'a>>>,
// }
impl SchemaParser {



    pub fn parse<'a>(input: &'a DeriveInput) -> TableDef<'_> {
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
        let fields_def: Vec<FieldDef<'a>> = fields.iter().map(|f| FieldParser::parse(*f)).collect();

        // let primary_key_fields = IndexParser::build_index(&fields_def, &input.attrs, "primary");

        // let primary_attr_opt: Option<NamedAttribute<'a>> = AttrParser::extract(&input.attrs, "primary");
        // if primary_attr_opt.is_none() {
        //     panic!("primary attribute not found in schema");
        // }
        // let primary_attr = primary_attr_opt.unwrap();
        // let primary_key_fields: Vec<&'a FieldDef> =
        //     IndexParser::filter_cloned(&fields_def, primary_attr);



        let unique_attrs_list: Vec<NamedAttributes> =
            AttrParser::extract_multi_list(&input.attrs, "unique");
        // let serde_structs = serde_attr.map(|attr| attr.values);



        let mut table_def = TableDef::default();
        table_def.table_name = table_name.clone();
        table_def.serde_structs = serde_structs;
        table_def.fields = fields_def;
        // table_def.primary_fields = primary_key_fields;
        table_def
    }
}






// pub fn extract_table_def<'a>(struct_name: &'a str, attrs: &'a [Attribute], data: &'a Data) -> TableDef<'a> {
//
//
//     let table_name_attr: Option<&Attribute> = AttrParser::get_attr(attrs, "table");
//     let table_name = if let Some(attr) = table_name_attr {
//         AttrParser::parse_one_single(&attr).get_single_value().to_string()
//     } else {
//         struct_name.to_snake()
//     };
//
//     let fields = InputParser::get_fields(data);
//     let fields_def: Vec<FieldDef<'a>> = fields.iter().map(|f|FieldParser::parse(*f)).collect();
//
//     let mut table_def = TableDef::default();
//     // table_def.table_name = Cow::Owned(table_name);
//     // table_def.columns = fields_def;
//     // panic!("{:?}", table_def);
//     table_def
// }
