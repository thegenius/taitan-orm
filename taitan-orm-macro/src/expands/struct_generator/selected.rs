use case::CaseExt;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{Attribute, FieldsNamed};
use crate::attrs::{AttrParser, DefaultAttrParser};
use crate::fields::{FieldsParser, StructConstructor};

pub fn generate_selected_struct(
    ident: &Ident,
    attrs: &Vec<Attribute>,
    fields: &FieldsNamed,
    should_serde: bool,
) -> TokenStream {
    let table_name = DefaultAttrParser::extract_table_name(ident, attrs);
    let selected_name = format!("{}SelectedEntity", table_name.to_camel());
    let parser = FieldsParser::from_named(fields);

    let struct_stream = parser.of_option_selected(&table_name, &selected_name, should_serde);
    let output = quote! {
        #struct_stream
    };

    output
}