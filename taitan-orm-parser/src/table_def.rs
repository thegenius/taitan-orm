use crate::attr_parser::{AttrParser, NamedAttribute, NamedAttributes};
use crate::field_def::FieldDef;
use serde::de::{SeqAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use std::borrow::Cow;
use std::collections::HashMap;
use std::marker::PhantomData;
use syn::{Attribute, Field};
use crate::{FieldParser, InputParser};
use crate::info_parser::IndexParser;

// #[table(user)]
// #[serde_struct(entity, location)]
// #[primary(id)]
// #[unique(name, age)]
// #[index(name = xxx, fields = (f1, f2, f3))]
#[derive(Debug, PartialEq, Clone, Default, Serialize)]
pub struct TableDef<'a> {
    pub table_name: Cow<'a, str>,
    pub serde_structs: Vec<Cow<'a, str>>,
    pub fields: Vec<FieldDef<'a>>,
    pub primary_fields: Vec<Cow<'a, str>>,
    pub uniques: Vec<NamedFieldsGroup<'a>>,
    pub indexes: Vec<NamedFieldsGroup<'a>>,
}

impl<'a> TableDef<'a> {
    pub fn new(fields: &'a [Field], attrs: &'a [Attribute]) -> TableDef<'a> {
        let fields_def: Vec<FieldDef> = fields.iter().map(|f| FieldParser::parse(f)).collect();
        let primary_attr = AttrParser::extract_one(attrs, "primary");

        Self {
            fields: fields_def,
            primary_fields: primary_attr.values,
            ..Default::default()
        }

    }
}


#[derive(Debug, PartialEq, Clone, Default, Serialize)]
pub struct NamedFieldsGroup<'a> {
    pub name: Cow<'a, str>,
    pub fields: Vec<Cow<'a, str>>,
}

// impl<'a> NamedFieldsGroup<'a> {
//
//     pub fn new_indexes(fields: &'a [FieldDef<'a>], attrs_vec: &'a [NamedAttributes]) -> Vec<Self> {
//         attrs_vec.iter().map(|attrs| Self::new_index(fields, attrs)).collect()
//     }
//     pub fn new_index(fields: &'a [FieldDef<'a>], attrs: &'a NamedAttributes) -> Self {
//         let mut group = NamedFieldsGroup::default();
//         Self::build_index_from_attrs(&mut group, fields, attrs);
//         group
//     }
//
//     pub fn new(fields: &'a [FieldDef<'a>], attr: &'a NamedAttribute) -> Self {
//         let mut group = NamedFieldsGroup::default();
//         Self::build_index_from_attr(&mut group, fields, attr);
//         group
//     }
//     fn build_index_from_attrs(named: &mut NamedFieldsGroup<'a>, fields: &'a [FieldDef<'a>], attrs: &'a NamedAttributes) {
//         for attr in attrs.attrs.iter() {
//             if attr.name == "name" {
//                 named.name = attr.values.get(0).unwrap().clone();
//             }
//             if attr.name == "fields" {
//                 let field_defs = fields
//                     .iter()
//                     .filter(|f| attr.values.iter().any(|a| a == f.column_name()))
//                     .collect::<Vec<_>>();
//                 named.fields = field_defs
//             }
//         }
//     }
//
//     fn build_index_from_attr(named: &mut NamedFieldsGroup<'a>, fields: &'a [FieldDef<'a>], attr: &'a NamedAttribute) {
//         let field_defs = fields
//             .iter()
//             .filter(|f| attr.values.iter().any(|a| a == f.column_name()))
//             .collect::<Vec<_>>();
//         named.fields = field_defs
//     }
// }
