use syn::Attribute;
use crate::attr_parser::{AttrParser, NamedAttribute, NamedAttributes};
use crate::table_def::NamedFieldsGroup;
use crate::FieldDef;

pub struct IndexParser;

impl IndexParser {
    // pub fn new_indexes<'a>(
    //     fields: &'a [FieldDef<'a>],
    //     attrs_vec: &'a [NamedAttributes],
    // ) -> Vec<NamedFieldsGroup<'a>> {
    //     attrs_vec
    //         .iter()
    //         .map(|attrs| Self::new_index(fields, attrs))
    //         .collect()
    // }
    // pub fn new_index<'a>(
    //     fields: &'a [FieldDef<'a>],
    //     attrs: &'a NamedAttributes,
    // ) -> NamedFieldsGroup<'a> {
    //     let mut group = NamedFieldsGroup::default();
    //     Self::build_index_from_attrs(&mut group, fields, attrs);
    //     group
    // }
    //
    // pub fn new<'a>(fields: &'a [FieldDef<'a>], attr: &'a NamedAttribute) -> NamedFieldsGroup<'a> {
    //     let mut group = NamedFieldsGroup::default();
    //     Self::build_index_from_attr(&mut group, fields, attr);
    //     group
    // }
    //
    // pub fn build_index<'a>(fields: &'a [FieldDef<'a>], attrs: &'a [Attribute], name: &'a str) -> Vec<&'a FieldDef<'a>> {
    //     let primary_attr_opt: Option<NamedAttribute<'a>> = AttrParser::extract(attrs, name);
    //     if primary_attr_opt.is_none() {
    //         panic!("primary attribute not found in schema");
    //     }
    //     let primary_attr = primary_attr_opt.unwrap();
    //     IndexParser::filter_cloned(&fields, primary_attr.clone())
    // }
    //
    // pub fn filter_cloned<'a>(
    //     fields: &'a [FieldDef<'a>],
    //     attr: NamedAttribute,
    // ) -> Vec<&'a FieldDef<'a>> {
    //     fields
    //         .iter()
    //         .filter(|f| attr.values.iter().any(|a| a == f.column_name()))
    //         .collect::<Vec<_>>()
    // }
    // pub fn filter<'a>(
    //     fields: &'a [FieldDef<'a>],
    //     attr: &'a NamedAttribute,
    // ) -> Vec<&'a FieldDef<'a>> {
    //     fields
    //         .iter()
    //         .filter(|f| attr.values.iter().any(|a| a == f.column_name()))
    //         .collect::<Vec<_>>()
    // }

    // fn build_index_from_attrs<'a>(
    //     named: &mut NamedFieldsGroup<'a>,
    //     fields: &'a [FieldDef<'a>],
    //     attrs: &'a NamedAttributes,
    // ) {
    //     for attr in attrs.attrs.iter() {
    //         if attr.name == "name" {
    //             named.name = attr.values.get(0).unwrap().clone();
    //         }
    //         if attr.name == "fields" {
    //             named.fields = Self::filter(fields, attr);
    //         }
    //     }
    // }
    //
    // fn build_index_from_attr<'a>(
    //     named: &mut NamedFieldsGroup<'a>,
    //     fields: &'a [FieldDef<'a>],
    //     attr: &'a NamedAttribute,
    // ) {
    //     let field_defs = Self::filter(fields, attr);
    //     named.fields = field_defs
    // }
}
