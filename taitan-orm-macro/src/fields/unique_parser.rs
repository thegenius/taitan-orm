use crate::fields::mappers::{ArgsConstructorMySql, ArgsConstructorPostgres, ArgsConstructorSqlite, NamesAddConstructor, NamesConstructor};
use crate::fields::table_name_parser::TableNameParser;
use crate::fields::{FieldsContainer};
use proc_macro2::TokenStream;
use std::fmt::Debug;
use syn::Field;

// 1. Unique中不包含option的field
// 2. 因为涉及为多个Unique生成类，所以parser里面应该传入正确的fields
pub trait UniqueParser:
    FieldsContainer
    + NamesConstructor
    + NamesAddConstructor
    + TableNameParser
    + ArgsConstructorPostgres
    + ArgsConstructorSqlite
    + ArgsConstructorMySql
{
    fn get_unique_field_names(&self) -> TokenStream {
        self.of_names_array()
    }

    fn gen_update_arguments_sqlite<'a>(
        &'a self,
        mutation_fields: &'a Vec<Field>,
    ) -> TokenStream {
        self.of_unique_update_args_sqlite(mutation_fields)
    }
    fn gen_update_arguments_mysql<'a>(
        &'a self,
        mutation_fields: &'a Vec<Field>,
    ) -> TokenStream {
        self.of_unique_update_args_mysql(mutation_fields)
    }
    fn gen_update_arguments_postgres<'a>(
        &'a self,
        mutation_fields: &'a Vec<Field>,
    ) -> TokenStream {
        self.of_unique_update_args_postgres(mutation_fields)
    }

    fn gen_unique_arguments_sqlite(&self) -> TokenStream {
        self.of_not_option_args_sqlite()
    }

    fn gen_unique_arguments_mysql(&self) -> TokenStream {
        self.of_not_option_args_mysql()
    }

    fn gen_unique_arguments_postgres(&self) -> TokenStream {
        self.of_not_option_args_postgres()
    }
}
