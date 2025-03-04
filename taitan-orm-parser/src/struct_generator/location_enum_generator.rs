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
        // let table_name =  field_mapper.escape(&table_def.table_name, db_type);
        let sql_generator = SqlGenerator::default();
        let fields = table_def.get_not_primary_fields();
        let fields_stream = field_mapper.gen_enum_expr_variants(fields);
        quote! {
            #[derive(Clone, Debug, taitan_orm_macro::Parameter, taitan_orm_macro::LocationNew)]
            pub enum #struct_ident {
                #fields_stream
            }
        }
    }
}
