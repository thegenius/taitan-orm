use case::CaseExt;
use proc_macro2::{Ident, TokenStream};
use syn::{Attribute, FieldsNamed};
use crate::attrs::{AttrParser, DefaultAttrParser};
use crate::fields::{FieldsParser, StructConstructor};

pub fn generate_location_struct(
    ident: &Ident,
    attrs: &Vec<Attribute>,
    fields: &FieldsNamed,
    should_serde: bool,
) -> TokenStream {
    let table_name = DefaultAttrParser::extract_table_name(ident, attrs);
    let struct_name = format!("{}Location", table_name.to_camel());
    FieldsParser::from_named(fields).of_location(&table_name, &struct_name, should_serde)
}