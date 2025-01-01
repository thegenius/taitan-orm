use crate::fields::{FieldsContainer, FieldsFilter, FieldsParser};
use case::CaseExt;
use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote};
use syn::{Attribute, Field, FieldsNamed};

use crate::attrs::{AttrParser, DefaultAttrParser};
use crate::fields::UniqueParser;

fn generate_struct_and_impl(
    table_name: &str,
    struct_name: &str,
    fields: &Vec<Field>,
    mutation_fields: &Vec<Field>,
) -> TokenStream {
    let unique_field_names = FieldsParser::from_vec(fields).get_unique_field_names();
    let unique_arguments_sqlite = FieldsParser::from_vec(fields).gen_unique_arguments_sqlite();
    let unique_arguments_mysql = FieldsParser::from_vec(fields).gen_unique_arguments_mysql();
    let unique_arguments_postgres = FieldsParser::from_vec(fields).gen_unique_arguments_postgres();

    let update_arguments_sqlite = FieldsParser::from_vec(fields).gen_update_arguments_sqlite(mutation_fields);
    let update_arguments_mysql = FieldsParser::from_vec(fields).gen_update_arguments_mysql(mutation_fields);
    let update_arguments_postgres = FieldsParser::from_vec(fields).gen_update_arguments_postgres(mutation_fields);

    let struct_ident = Ident::new(&struct_name, Span::call_site());
    let mutation_ident = format_ident!("{}Mutation", table_name.to_camel());
    let fields_tokens = FieldsParser::from_vec(fields).get_not_option_fields();

    let output = quote! {
        #[derive(Debug, Clone)]
        pub struct #struct_ident {
            #fields_tokens
        }

        impl taitan_orm::traits::Unique for #struct_ident {

            type Mutation = #mutation_ident;

            fn get_table_name(&self) -> &'static str {
                #table_name
            }

            fn get_unique_field_names(&self) -> &'static [&'static str] {
                #unique_field_names
            }

            fn gen_update_arguments_sqlite<'a>(
                &'a self,
                mutation: &'a Self::Mutation,
            ) -> Result<sqlx::sqlite::SqliteArguments<'a>, sqlx::error::BoxDynError> {
                #update_arguments_sqlite
            }

            fn gen_update_arguments_mysql<'a>(
                &'a self,
                mutation: &'a Self::Mutation,
            ) -> Result<sqlx::mysql::MySqlArguments, sqlx::error::BoxDynError> {
                #update_arguments_mysql
            }

            fn gen_update_arguments_postgres<'a>(
                &'a self,
                mutation: &'a Self::Mutation,
            ) -> Result<sqlx::postgres::PgArguments, sqlx::error::BoxDynError> {
                #update_arguments_postgres
            }

            fn gen_unique_arguments_sqlite(&self) -> Result<sqlx::sqlite::SqliteArguments<'_>, sqlx::error::BoxDynError> {
                #unique_arguments_sqlite
            }

            fn gen_unique_arguments_mysql(&self) -> Result<sqlx::mysql::MySqlArguments, sqlx::error::BoxDynError> {
                #unique_arguments_mysql
            }

            fn gen_unique_arguments_postgres(&self) -> Result<sqlx::postgres::PgArguments, sqlx::error::BoxDynError> {
                #unique_arguments_postgres
            }
        }
    };

    output
}

pub fn generate_unique_structs_and_impls(
    ident: &Ident,
    attrs: &Vec<Attribute>,
    fields: &FieldsNamed,
) -> TokenStream {
    let table_name = DefaultAttrParser::extract_table_name(ident, attrs);
    let fields_vec = FieldsParser::from_named(fields).filter_annotated_fields("primary_key");
    let mutation_fields_vec = FieldsParser::from_named(fields).filter_not_annotated_fields("primary_key");
    let primary_struct_name = format!("{}Primary", table_name.to_camel());
    let mut all_unique_stream =
        generate_struct_and_impl(&table_name, &primary_struct_name, &fields_vec, &mutation_fields_vec);



    let parser = FieldsParser::from_named(fields);
    let order_fields_vec = DefaultAttrParser::extract_unique_key(attrs);
    // panic!("{:?}", order_fields_vec);
    order_fields_vec.iter().for_each(|fields| {
        let unique_fields = parser.filter_named_fields(fields);
        let mut unique_struct_name = format!("{}", table_name.to_camel());
        fields.iter().for_each(|field| {
            unique_struct_name.push_str(field.to_camel().as_ref());
        });
        unique_struct_name.push_str("Unique");
        let unique_stream =
            generate_struct_and_impl(&table_name, &unique_struct_name, &unique_fields, &mutation_fields_vec);
        all_unique_stream.extend(unique_stream);
    });

    all_unique_stream
}
