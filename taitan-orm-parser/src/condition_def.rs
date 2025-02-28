use crate::{ConditionParser, NamedVariantDef, TableDef};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use syn::DeriveInput;
use crate::info_parser::schema_parser::SchemaParser;

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct ConditionDef<'a> {
    pub struct_name: Cow<'a, str>,
    pub table_name: Cow<'a, str>,
    pub variants: Vec<NamedVariantDef<'a>>,
}

impl<'a> ConditionDef<'a> {
    pub fn parse(input: &'a DeriveInput) -> ConditionDef<'a> {

        ConditionParser::parse(&input)
    }
}