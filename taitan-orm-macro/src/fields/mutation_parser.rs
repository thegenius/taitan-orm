use crate::fields::mappers::{ArgsConstructorMySql, ArgsConstructorPostgres, ArgsConstructorSqlite, NamesAddConstructor, NamesConstructor};
use crate::fields::FieldsContainer;
use proc_macro2::TokenStream;
use syn::Field;

pub trait MutationParser:
    FieldsContainer
    + NamesAddConstructor
    + NamesConstructor
    + ArgsConstructorPostgres
    + ArgsConstructorSqlite
    + ArgsConstructorMySql
{
    fn get_mutation_fields_name(&self) -> TokenStream {
        self.of_maybe_option_names_vec()
    }

    fn gen_update_arguments_sqlite(&self, primary_fields: &Vec<Field>) -> TokenStream {
        self.of_update_args_sqlite(primary_fields)
    }

    fn gen_update_arguments_mysql(&self, primary_fields: &Vec<Field>) -> TokenStream {
        self.of_update_args_mysql(primary_fields)
    }

    fn gen_update_arguments_postgres(&self, primary_fields: &Vec<Field>) -> TokenStream {
        self.of_update_args_postgres(primary_fields)
    }

    fn gen_change_arguments_sqlite(&self, location_fields: &Vec<Field>) -> TokenStream {
        self.of_change_args_sqlite(location_fields)
    }

    fn gen_change_arguments_mysql(&self, location_fields: &Vec<Field>) -> TokenStream {
        self.of_change_args_mysql(location_fields)
    }

    fn gen_change_arguments_postgres(&self, location_fields: &Vec<Field>) -> TokenStream {
        self.of_change_args_postgres(location_fields)
    }
}
