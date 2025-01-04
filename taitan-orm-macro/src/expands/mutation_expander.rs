use case::CaseExt;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::{Attribute, FieldsNamed};
use crate::attrs::{AttrParser, DefaultAttrParser};
use crate::fields::{FieldsContainer, FieldsFilter, FieldsParser, NamesConstructor, StructConstructor};
use crate::fields::{ArgsConstructorPostgres, ArgsConstructorMySql, ArgsConstructorSqlite};


pub fn generate_mutation_struct_and_impl(
    ident: &Ident,
    attrs: &Vec<Attribute>,
    fields: &FieldsNamed,
    should_serde: bool,
) -> TokenStream {
    let table_name = DefaultAttrParser::extract_table_name(ident, attrs);
    let fields_vec = FieldsParser::from_named(fields).filter_not_annotated_fields("primary_key");

    let parser = FieldsParser::from_named(fields);
    let location_fields_vec = parser.get_fields();

    let fields_name_vec = FieldsParser::from_vec(&fields_vec).of_option_names_vec();

    let change_args_sqlite = FieldsParser::from_vec(&fields_vec).of_change_args_sqlite(location_fields_vec);
    let change_args_mysql = FieldsParser::from_vec(&fields_vec).of_change_args_mysql(location_fields_vec);
    let change_args_postgres = FieldsParser::from_vec(&fields_vec).of_change_args_postgres(location_fields_vec);

    let mutation_struct_name =  format!("{}Mutation", table_name.to_camel());
    // let primary_struct_name =  format!("{}Primary", table_name.to_camel());
    let location_struct_name =  format!("{}Location", table_name.to_camel());
    let struct_ident = Ident::new(&mutation_struct_name, Span::call_site());
    // let primary_struct_ident = Ident::new(&primary_struct_name, Span::call_site());
    let location_struct_ident = Ident::new(&location_struct_name, Span::call_site());
    let struct_stream = FieldsParser::from_vec(&fields_vec).of_option(&mutation_struct_name, should_serde);

    let output = quote! {

        #struct_stream

        impl taitan_orm::traits::Mutation for #struct_ident {

            type Location = #location_struct_ident;

            fn get_mutation_fields_name(&self) -> Vec<taitan_orm::FieldName> {
                #fields_name_vec
            }

            fn gen_change_arguments_sqlite<'a>(
                &'a self,
                location: &'a Self::Location,
            ) -> Result<sqlx::sqlite::SqliteArguments<'a>, sqlx::error::BoxDynError> {
                #change_args_sqlite
            }
            fn gen_change_arguments_mysql<'a>(
                &'a self,
                location: &'a Self::Location,
            ) -> Result<sqlx::mysql::MySqlArguments, sqlx::error::BoxDynError> {
                #change_args_mysql
            }
            fn gen_change_arguments_postgres<'a>(
                &'a self,
                location: &'a Self::Location,
            ) -> Result<sqlx::postgres::PgArguments, sqlx::error::BoxDynError> {
                #change_args_postgres
            }
        }
    };

    output
}