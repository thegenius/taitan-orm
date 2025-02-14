use crate::attr_parser::{AttrParser, NamedAttribute, NamedAttributes};
use crate::field_def::FieldDef;
use crate::{FieldParser, InputParser};
use case::CaseExt;
use serde::de::SeqAccess;
use serde::{Deserializer, Serialize};
use std::borrow::Cow;
use syn::DeriveInput;

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
    pub fn new(input: &'a DeriveInput) -> TableDef<'a> {
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
        let primary_attr = AttrParser::extract_one(attrs, "primary");
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
        Self {
            table_name: table_name.clone(),
            serde_structs,
            fields: fields_def,
            primary_fields: primary_attr.values,
            uniques,
            indexes,
        }
    }

    pub fn get_primary_fields(&'a self) -> Vec<&'a FieldDef<'a>> {
        self.filter(&self.primary_fields)
    }

    pub fn get_not_primary_fields(&'a self) -> Vec<&'a FieldDef<'a>> {
        self.filter_not_in(&self.primary_fields)
    }

    pub fn get_unique_names(&'a self) -> Vec<Cow<'a, str>> {
        self.uniques.iter().map(|u| u.name.clone()).collect()
    }

    pub fn get_index_names(&'a self) -> Vec<Cow<'a, str>> {
        self.indexes.iter().map(|u| u.name.clone()).collect()
    }

    pub fn get_unique_fields(&'a self, name: &str) -> Vec<&'a FieldDef<'a>> {
        let unique = self.uniques.iter().find(|u| u.name == name);
        if let Some(unique) = unique {
            self.filter(&unique.fields)
        } else {
            Vec::new()
        }
    }

    pub fn get_index_fields(&'a self, name: &str) -> Vec<&'a FieldDef<'a>> {
        let index = self.indexes.iter().find(|u| u.name == name);
        if let Some(index) = index {
            self.filter(&index.fields)
        } else {
            Vec::new()
        }
    }

    fn filter(&'a self, names: &[Cow<'a, str>]) -> Vec<&'a FieldDef<'a>> {
        self.fields
            .iter()
            .filter(|f| names.iter().any(|a| a == f.column_name()))
            .collect::<Vec<_>>()
    }

    fn filter_not_in(&'a self, names: &[Cow<'a, str>]) -> Vec<&'a FieldDef<'a>> {
        self.fields
            .iter()
            .filter(|f| names.iter().all(|a| a != f.column_name()))
            .collect::<Vec<_>>()
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize)]
pub struct NamedFieldsGroup<'a> {
    pub name: Cow<'a, str>,
    pub fields: Vec<Cow<'a, str>>,
}

impl<'a> From<NamedAttributes<'a>> for NamedFieldsGroup<'a> {
    fn from(attrs: NamedAttributes<'a>) -> NamedFieldsGroup<'a> {
        let mut named = NamedFieldsGroup::default();
        for attr in attrs.attrs.iter() {
            if attr.name == "name" {
                named.name = attr.values.get(0).unwrap().clone();
            }
            if attr.name == "fields" {
                let fields = &attr.values;
                named.fields = fields.clone();
            }
        }
        named
    }
}

impl<'a> From<NamedAttribute<'a>> for NamedFieldsGroup<'a> {
    fn from(attr: NamedAttribute<'a>) -> NamedFieldsGroup<'a> {
        NamedFieldsGroup {
            name: attr.name,
            fields: attr.values,
        }
    }
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
