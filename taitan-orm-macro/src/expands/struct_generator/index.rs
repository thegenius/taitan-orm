use crate::attrs::{AttrParser, DefaultAttrParser};
use crate::fields::{ArgsConstructorSqlite, FieldsFilter, FieldsParser, SqlConstructors, StructConstructor};
use case::CaseExt;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::{Attribute, FieldsNamed};

pub fn generate_index_struct(
    ident: &Ident,
    attrs: &Vec<Attribute>,
    fields: &FieldsNamed,
    should_serde: bool,
) -> TokenStream {
    let table_name = DefaultAttrParser::extract_table_name(ident, attrs);
    let index_fields = DefaultAttrParser::extract_index_fields(&attrs);
    let mut structs: Vec<TokenStream> = Vec::new();
    for index in index_fields.iter() {
        let index_struct_name = format!("{}Index{}", table_name.to_camel(), index.name.to_camel());
        let parser = FieldsParser::from_named(fields);
        let index_fields = parser.filter_named_fields(&index.fields.clone());
        let index_parser = FieldsParser::from_vec(&index_fields);
        let struct_stream = index_parser.of_index_enum(&table_name, &index_struct_name, should_serde);

        let struct_ident = format_ident!("{}", index_struct_name);

        let where_fn_stream = index_parser.of_index_enum_where_fn(&index_struct_name);
        let args_sqlite_stream = index_parser.of_index_enum_args_sqlite(&index_struct_name);
        let args_mysql_stream = index_parser.of_index_enum_args_mysql(&index_struct_name);
        let args_postgres_stream = index_parser.of_index_enum_args_postgres(&index_struct_name);

        let output = quote! {
            #struct_stream

            impl taitan_orm::traits::Location for #struct_ident {

                fn get_table_name(&self) -> &'static str {
                    #table_name
                }

                #where_fn_stream

                // fn gen_location_arguments_sqlite(&self) -> Result<sqlx::sqlite::SqliteArguments<'_>, sqlx::error::BoxDynError> {
                //     todo!()
                // }
                #args_sqlite_stream

                // fn gen_location_arguments_mysql(&self) -> Result<sqlx::mysql::MySqlArguments, sqlx::error::BoxDynError> {
                //     todo!()
                // }
                #args_mysql_stream

                // fn gen_location_arguments_postgres(&self) -> Result<sqlx::postgres::PgArguments, sqlx::error::BoxDynError> {
                //     todo!()
                // }
                #args_postgres_stream

            }
        };
        structs.push(output);
    }

    quote! {
        #( #structs )*
    }
}
