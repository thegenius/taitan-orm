use crate::fields::mappers::{
    ArgsConstructorMySql, ArgsConstructorPostgres, ArgsConstructorSqlite, NamesAddConstructor,
    NamesConstructor,
};
use crate::fields::{FieldsContainer, TableNameParser};
use proc_macro2::TokenStream;

pub trait LocationParser:
    FieldsContainer
    + TableNameParser
    + NamesAddConstructor
    + NamesConstructor
    + ArgsConstructorPostgres
    + ArgsConstructorSqlite
    + ArgsConstructorMySql
{
    fn get_location_fields_name(&self) -> TokenStream {
        self.of_option_names_vec()
    }

    fn get_where_clause(&self) -> TokenStream {
        self.of_where_clause()
    }

    fn gen_location_arguments_sqlite(&self) -> TokenStream {
        self.of_location_args_sqlite()
    }

    fn gen_location_arguments_mysql(&self) -> TokenStream {
        self.of_location_args_mysql()
    }

    fn gen_location_arguments_postgres(&self) -> TokenStream {
        self.of_location_args_postgres()
    }
}
