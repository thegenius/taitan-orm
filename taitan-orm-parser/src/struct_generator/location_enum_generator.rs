use crate::{FieldMapper, SqlGenerator, TableDef};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::fmt::Debug;

#[derive(Debug, Default)]
pub struct LocationEnumGenerator;

impl LocationEnumGenerator {
    pub fn generate(&self, table_def: &TableDef) -> TokenStream {
        let struct_name = &table_def.struct_name;
        let struct_ident = format_ident!("{}Location", struct_name);

        let field_mapper = FieldMapper::new();
        let table_name =  &table_def.table_name;
        let table_name_ident = format_ident!("{}", table_name);
        let sql_generator = SqlGenerator::default();
        let fields = &table_def.fields;
        let fields_stream = field_mapper.gen_enum_expr_variants(fields);
        quote! {
            #[derive(Clone, Debug, taitan_orm::macros::Parameter, taitan_orm::macros::Location, serde::Serialize, serde::Deserialize)]
            #[table(#table_name_ident)]
            pub enum #struct_ident {
                #fields_stream
            }
        }
    }
}
