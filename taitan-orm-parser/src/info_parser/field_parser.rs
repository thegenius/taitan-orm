use std::borrow::Cow;
use quote::ToTokens;
use serde::{Deserialize, Serialize};
use syn::Field;
use crate::info_parser::type_parser::{TypeParser};
use crate::field_def::{FieldDef, StructFieldDef, TableColumnDef};
use crate::LifetimeParser;
// #[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
// pub struct StructFieldDef {
//     name: Cow<'static, str>,
//     rust_type: Cow<'static, str>,
//     default_value: Option<Cow<'static, str>>,
//     is_optional: bool,
// }
//
//
// #[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
// pub struct TableColumnDef {
//     name: Option<Cow<'static, str>>,
//     column_type: Option<Cow<'static, str>>,
//     default_value: Option<Cow<'static, str>>,
//     is_nullable: bool,
//     is_generated: bool,
//     is_auto_inc: bool,
//     is_primary_key_part: bool,
// }

pub struct FieldParser;

impl FieldParser {
    pub fn parse(field: &Field) -> FieldDef {

        let field_name = field.clone().ident.unwrap().to_string();
        let field_type = TypeParser::get_inner_type(&field.ty);
        let field_type_str = field_type.to_token_stream().to_string();
        let is_optional = TypeParser::is_option(&field.ty);
        let lifetime = LifetimeParser::get_lifetime(&field.ty).map(|l| Cow::Owned(l.to_string()));

        let mut field_def = FieldDef::default();
        field_def.struct_field.name = Cow::Owned(field_name.clone());
        field_def.struct_field.rust_type = Cow::Owned(field_type_str.clone());
        field_def.struct_field.is_optional = is_optional;
        field_def.struct_field.lifetime = lifetime;
        field_def
    }
}
