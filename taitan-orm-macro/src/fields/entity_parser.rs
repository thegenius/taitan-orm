
use crate::fields::table_name_parser::TableNameParser;
use crate::fields::FieldsParser;
use crate::fields::{DefaultFieldMapper, FieldMapType, FieldMapper, FieldsContainer, FieldsFilter};
use proc_macro2::{TokenStream};
use quote::quote;
use crate::fields::fields_mapper::FieldsMapper;
use crate::fields::mappers::{ArgsConstructorMySql, ArgsConstructorPostgres, ArgsConstructorSqlite, NamesConstructor};

/**
和trait Entity 一一对应
*/
pub trait EntityParser: FieldsContainer + TableNameParser {
    fn get_insert_fields(&self) -> TokenStream;
    fn get_upsert_set_fields(&self) -> TokenStream;
    fn get_auto_increment_field(&self) -> TokenStream;
    fn set_auto_increment_field(&mut self) -> TokenStream;
    fn gen_insert_arguments_sqlite(&self) -> TokenStream;
    fn gen_upsert_arguments_sqlite(&self) -> TokenStream;
    fn gen_insert_arguments_mysql(&self) -> TokenStream;
    fn gen_upsert_arguments_mysql(&self) -> TokenStream;
    fn gen_insert_arguments_postgres(&self) -> TokenStream;
    fn gen_upsert_arguments_postgres(&self) -> TokenStream;
}

impl EntityParser for FieldsParser {
    fn get_insert_fields(&self) -> TokenStream {
        let insert_fields = self.get_insert_fields_vec();
        FieldsParser::from_vec(&insert_fields).of_maybe_option_names_vec()
    }

    fn get_upsert_set_fields(&self) -> TokenStream {
        let upsert_set_fields = self.get_upsert_set_fields_vec();
        FieldsParser::from_vec(&upsert_set_fields).of_maybe_option_names_vec()
    }

    fn get_auto_increment_field(&self) -> TokenStream {
        let auto_field_opt = self.get_auto_increment_field_opt();
        if auto_field_opt.is_none() {
            quote! { None }
        } else {
            let auto_field = auto_field_opt.clone().unwrap();
            let auto_field_name =
                DefaultFieldMapper::map_field(auto_field, FieldMapType::Str);
            quote! {
                Some(#auto_field_name)
            }
        }
    }

    fn set_auto_increment_field(&mut self) -> TokenStream {
        let auto_field_opt = self.get_auto_increment_field_opt();
        if auto_field_opt.is_none() {
            quote! { false }
        } else {
            let auto_field = auto_field_opt.unwrap();
            let auto_field_name = auto_field.ident.unwrap();
            // quote! {
            //     self.#auto_field_name = value;
            //     true
            // }
            quote! {
                true
            }
        }
    }

    fn gen_insert_arguments_sqlite(&self) -> TokenStream {
        let all_fields = self.get_insert_fields_vec();
        FieldsParser::from_vec(&all_fields).of_maybe_option_args_sqlite()
    }

    fn gen_upsert_arguments_sqlite(&self) -> TokenStream {
        let all_fields = self.get_upsert_fields_vec();
        FieldsParser::from_vec(&all_fields).of_maybe_option_args_sqlite()
    }

    fn gen_insert_arguments_mysql(&self) -> TokenStream {
        let all_fields = self.get_insert_fields_vec();
        FieldsParser::from_vec(&all_fields).of_maybe_option_args_mysql()
    }

    fn gen_upsert_arguments_mysql(&self) -> TokenStream {
        let all_fields = self.get_upsert_fields_vec();
        FieldsParser::from_vec(&all_fields).of_maybe_option_args_mysql()
    }

    fn gen_insert_arguments_postgres(&self) -> TokenStream {
        let all_fields = self.get_insert_fields_vec();
        FieldsParser::from_vec(&all_fields).of_maybe_option_args_postgres()
    }

    fn gen_upsert_arguments_postgres(&self) -> TokenStream {
        let all_fields = self.get_upsert_fields_vec();
        FieldsParser::from_vec(&all_fields).of_maybe_option_args_postgres()
    }
}
