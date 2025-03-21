use crate::attrs::{AttrParser, DefaultAttrParser};
use crate::fields::FieldsParser;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::Attribute;

pub trait TableNameParser {
    fn get_table_name(ident: &Ident, attrs: &Vec<Attribute>) -> TokenStream;
}

impl TableNameParser for FieldsParser {
    fn get_table_name(ident: &Ident, attrs: &Vec<Attribute>) -> TokenStream {
        let table_name = DefaultAttrParser::extract_table_name(ident, attrs);
        quote! {
            #table_name
        }
    }
}
