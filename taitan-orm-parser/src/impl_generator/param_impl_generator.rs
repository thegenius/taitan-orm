use crate::{DatabaseType, FieldMapper, SqlType, TableDef};
use proc_macro2::TokenStream;
use quote::quote;

#[derive(Debug, Default)]
pub struct ParameterTraitImplGenerator;
impl ParameterTraitImplGenerator {
    pub fn gen_add_to_args(
        &self,
        db_type: &DatabaseType,
        table_def: &TableDef,
    ) -> TokenStream {
        let struct_name = &table_def.struct_name;
        let field_mapper = FieldMapper::new();
        let stream = field_mapper.gen_add_to_args(&table_def.fields);
        match db_type {
            DatabaseType::MySql => {
                quote! {
                    impl Parameter<MySql> for #struct_name {
                        fn add_to_args<'a, 'b>(&'a self, args: &'b mut <Sqlite as Database>::Arguments<'a>) -> Result<()> {
                            #stream
                            Ok(())
                        }
                    }
                }
            }
            DatabaseType::Postgres => {
                quote! {
                    impl Parameter<Postgres> for #struct_name {
                        fn add_to_args<'a, 'b>(&'a self, args: &'b mut <Sqlite as Database>::Arguments<'a>) -> Result<()> {
                            #stream
                            Ok(())
                        }
                    }
                }
            }
            DatabaseType::Sqlite => {
                quote! {
                    impl Parameter<Sqlite> for #struct_name {
                        fn add_to_args<'a, 'b>(&'a self, args: &'b mut <Sqlite as Database>::Arguments<'a>) -> Result<()> {
                            #stream
                            Ok(())
                        }
                    }
                }
            }
        }
    }
}
