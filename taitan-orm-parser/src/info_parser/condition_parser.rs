use std::borrow::Cow;
use case::CaseExt;
use syn::DeriveInput;
use crate::attr_parser::{AttrParser, NamedAttribute};
use crate::{FieldDef, FieldParser, InputParser, TableDef};
use crate::condition_def::ConditionDef;
use crate::table_def::NamedFieldsGroup;

#[derive(PartialEq, Clone, Copy, Default)]
pub struct ConditionParser;

impl ConditionParser {
    pub fn parse(input: &DeriveInput) -> ConditionDef {
        let struct_name = input.ident.to_string();
        let table_name_attr: Option<NamedAttribute> = AttrParser::extract(&input.attrs, "table");
        let table_name = if let Some(attr) = &table_name_attr {
            attr.get_single_value().to_owned()
        } else {
            Cow::Owned(struct_name.to_snake())
        };
        let serde_attr: Option<NamedAttribute> = AttrParser::extract(&input.attrs, "serde_struct");
        let serde_structs = serde_attr.map(|attr| attr.values).unwrap_or_default();

        let variants = InputParser::get_enum_variant_defs(&input.data).unwrap();

        ConditionDef {
            struct_name: Cow::Owned(struct_name),
            table_name,
            variants,
        }
    }
}
