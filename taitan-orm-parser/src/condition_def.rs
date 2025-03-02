use crate::{ConditionParser, FieldDef, NamedVariantDef, TableDef};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use syn::{DeriveInput, Variant};
use crate::info_parser::schema_parser::SchemaParser;


#[derive(Debug, PartialEq, Clone)]
pub enum VariantsOrFields<'a> {
    Variants(Vec<NamedVariantDef<'a>>),
    Fields(Vec<FieldDef<'a>>),
}


#[derive(Debug, PartialEq, Clone)]
pub struct ConditionDef<'a> {
    pub struct_name: Cow<'a, str>,
    pub table_name: Cow<'a, str>,
    pub variants_or_fields: VariantsOrFields<'a>,
}

impl<'a> ConditionDef<'a> {
    pub fn parse(input: &'a DeriveInput) -> ConditionDef<'a> {
        ConditionParser::parse(&input)
    }
}