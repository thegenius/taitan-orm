use proc_macro2::TokenStream;
use quote::quote;
use crate::{DatabaseType, FieldMapper, SqlType, TableDef};

#[derive(Debug, Default)]
pub struct ArgsGenerator;
impl ArgsGenerator {
    pub fn gen_add_to_args(
        &self,
        db_type: &DatabaseType,
        sql_type: &SqlType,
        table_def: &TableDef,
    ) -> TokenStream {
        let field_mapper = FieldMapper::new();
        let stream = field_mapper.gen_add_to_args(&table_def.fields);
        quote! {
            #stream
            Ok(())
        }
    }
}