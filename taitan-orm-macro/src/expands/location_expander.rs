use crate::attrs::{AttrParser, DefaultAttrParser};
use crate::expands::struct_generator::generate_location_struct;
use crate::fields::{FieldsFilter, FieldsParser};
use crate::fields::LocationParser;
use crate::fields::StructConstructor;
use case::CaseExt;
use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote};
use syn::{Attribute, FieldsNamed};

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

    let fields = FieldsParser::from_named(fields).filter_not_mode();

    let where_clause = FieldsParser::from_vec(&fields).get_where_clause();
    let location_arguments_sqlite =
        FieldsParser::from_vec(&fields).gen_location_arguments_sqlite();
    let location_arguments_mysql = FieldsParser::from_vec(&fields).gen_location_arguments_mysql();
    let location_arguments_postgres =
        FieldsParser::from_vec(&fields).gen_location_arguments_postgres();

    let output = quote! {
        impl #struct_ident {
            pub const ALL: taitan_orm::traits::AllLocation = taitan_orm::traits::AllLocation::new(#table_name);
        }

        impl taitan_orm::traits::Location for #struct_ident {

            fn get_table_name(&self) -> &'static str {
                #table_name
            }

            fn get_where_clause(&self) -> String {
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
