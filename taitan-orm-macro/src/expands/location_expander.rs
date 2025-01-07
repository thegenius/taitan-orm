use case::CaseExt;
use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote};
use syn::{Attribute, FieldsNamed};
use crate::attrs::{AttrParser, DefaultAttrParser};
use crate::expands::struct_generator::generate_location_struct;
use crate::fields::FieldsParser;
use crate::fields::StructConstructor;
use crate::fields::LocationParser;





pub fn generate_location_struct_and_impl(
    ident: &Ident,
    attrs: &Vec<Attribute>,
    fields: &FieldsNamed,
    should_serde: bool,
) -> TokenStream {

    let table_name = DefaultAttrParser::extract_table_name(ident, attrs);
    let struct_ident = format_ident!("{}Location", table_name.to_camel());
    let struct_stream = generate_location_struct(ident, attrs, fields, should_serde);

    let location_impl_stream = generate_location_impl(&struct_ident, attrs, fields);

    let output = quote! {

        #struct_stream

        #location_impl_stream
    };

    output
}

pub fn generate_location_impl(
    struct_ident: &Ident,
    attrs: &Vec<Attribute>,
    fields: &FieldsNamed,
) -> TokenStream {

    let table_name = DefaultAttrParser::extract_table_name(struct_ident, attrs);
    let parser = FieldsParser::from_named(fields);

    let where_clause = FieldsParser::from_named(fields).get_where_clause();
    let location_fields_name = parser.get_location_fields_name();
    let location_arguments_sqlite = FieldsParser::from_named(fields).gen_location_arguments_sqlite();
    let location_arguments_mysql = FieldsParser::from_named(fields).gen_location_arguments_mysql();
    let location_arguments_postgres = FieldsParser::from_named(fields).gen_location_arguments_postgres();


    let output = quote! {

        impl taitan_orm::traits::Location for #struct_ident {

            fn get_table_name(&self) -> &'static str {
                #table_name
            }

            fn get_location_fields_name(&self) -> Vec<taitan_orm::prelude::FieldName> {
                #location_fields_name
            }

            fn get_where_clause(&self, wrap_char: char, place_holder: char) -> String {
                #where_clause
            }

            fn gen_location_arguments_sqlite(&self) -> Result<sqlx::sqlite::SqliteArguments<'_>, sqlx::error::BoxDynError> {
                #location_arguments_sqlite
            }

            fn gen_location_arguments_mysql(&self) -> Result<sqlx::mysql::MySqlArguments, sqlx::error::BoxDynError> {
                #location_arguments_mysql
            }

            fn gen_location_arguments_postgres(&self) -> Result<sqlx::postgres::PgArguments, sqlx::error::BoxDynError> {
                #location_arguments_postgres
            }
        }
    };

    output
}