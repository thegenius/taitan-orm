use crate::fields::mappers::ArgsAddConstructor;
use crate::fields::{FieldsContainer, FieldsParser};
use proc_macro2::TokenStream;
use quote::quote;
use syn::Field;

pub trait ArgsConstructorSqlite: FieldsContainer + ArgsAddConstructor {
    fn of_maybe_option_args_sqlite(&self) -> TokenStream {
        let args_add_clause = self.map_field_vec(&<Self as ArgsAddConstructor>::of_maybe_option);
        quote! {
            let mut args = sqlx::sqlite::SqliteArguments::default();
            #(#args_add_clause)*
            Ok(args)
        }
    }

    fn of_not_option_args_sqlite(&self) -> TokenStream {
        let args_add_clause = self.map_field_vec(&<Self as ArgsAddConstructor>::of_not_option);
        quote! {
            let mut args = sqlx::sqlite::SqliteArguments::default();
            #(#args_add_clause)*
            Ok(args)
        }
    }

    fn of_option_args_sqlite(&self) -> TokenStream {
        let args_add_clause = self.map_field_vec(&<Self as ArgsAddConstructor>::of_option);
        quote! {
            let mut args = sqlx::sqlite::SqliteArguments::default();
            #(#args_add_clause)*
            Ok(args)
        }
    }

    fn of_location_args_sqlite(&self) -> TokenStream {
        let args_add_clause = self.map_field_vec(&<Self as ArgsAddConstructor>::of_location);
        quote! {
            let mut args = sqlx::sqlite::SqliteArguments::default();
            #(#args_add_clause)*
            Ok(args)
        }
    }

    fn of_unique_update_args_sqlite(&self, mutation_fields: &Vec<Field>) -> TokenStream {
        let unique_add_clause = self.map_field_vec(&<Self as ArgsAddConstructor>::of_not_option);
        let mutation_add_clause =
            FieldsParser::from_vec(mutation_fields).map_field_vec(&|field: Field| {
                <Self as ArgsAddConstructor>::of_option_with("mutation", field)
            });
        quote! {
            let mut args = sqlx::sqlite::SqliteArguments::default();
            #(#mutation_add_clause)*
            #(#unique_add_clause)*
            Ok(args)
        }
    }

    fn of_update_args_sqlite(&self, primary_fields: &Vec<Field>) -> TokenStream {
        let mutation_add_clause = self.map_field_vec(&<Self as ArgsAddConstructor>::of_option);
        let primary_add_clause =
            FieldsParser::from_vec(primary_fields).map_field_vec(&|field: Field| {
                <Self as ArgsAddConstructor>::of_not_option_with("primary", field)
            });
        quote! {
            let mut args = sqlx::sqlite::SqliteArguments::default();
            #(#mutation_add_clause)*
            #(#primary_add_clause)*
            Ok(args)
        }
    }
    fn of_change_args_sqlite(&self, location_fields: &Vec<Field>) -> TokenStream {
        let mutation_add_clause = self.map_field_vec(&<Self as ArgsAddConstructor>::of_option);
        let location_add_clause =
            FieldsParser::from_vec(location_fields).map_field_vec(&|field: Field| {
                <Self as ArgsAddConstructor>::of_location_with("location", field)
            });
        quote! {
            let mut args = sqlx::sqlite::SqliteArguments::default();
            #(#mutation_add_clause)*
            #(#location_add_clause)*
            Ok(args)
        }
    }
}
pub trait ArgsConstructorMySql: FieldsContainer + ArgsAddConstructor {
    fn of_maybe_option_args_mysql(&self) -> TokenStream {
        let args_add_clause = self.map_field_vec(&<Self as ArgsAddConstructor>::of_maybe_option);
        quote! {
            let mut args = sqlx::mysql::MySqlArguments::default();
            #(#args_add_clause)*
            Ok(args)
        }
    }

    fn of_not_option_args_mysql(&self) -> TokenStream {
        let args_add_clause = self.map_field_vec(&<Self as ArgsAddConstructor>::of_not_option);
        quote! {
            let mut args = sqlx::mysql::MySqlArguments::default();
            #(#args_add_clause)*
            Ok(args)
        }
    }

    fn of_option_args_mysql(&self) -> TokenStream {
        let args_add_clause = self.map_field_vec(&<Self as ArgsAddConstructor>::of_option);
        quote! {
            let mut args = sqlx::mysql::MySqlArguments::default();
            #(#args_add_clause)*
            Ok(args)
        }
    }

    fn of_location_args_mysql(&self) -> TokenStream {
        let args_add_clause = self.map_field_vec(&<Self as ArgsAddConstructor>::of_location);
        quote! {
            let mut args = sqlx::mysql::MySqlArguments::default();
            #(#args_add_clause)*
            Ok(args)
        }
    }

    fn of_update_args_mysql(&self, primary_fields: &Vec<Field>) -> TokenStream {
        let mutation_add_clause = self.map_field_vec(&<Self as ArgsAddConstructor>::of_option);
        let primary_add_clause =
            FieldsParser::from_vec(primary_fields).map_field_vec(&|field: Field| {
                <Self as ArgsAddConstructor>::of_not_option_with("primary", field)
            });
        quote! {
            let mut args = sqlx::mysql::MySqlArguments::default();
            #(#mutation_add_clause)*
            #(#primary_add_clause)*
            Ok(args)
        }
    }

    fn of_unique_update_args_mysql(&self, mutation_fields: &Vec<Field>) -> TokenStream {
        let unique_add_clause = self.map_field_vec(&<Self as ArgsAddConstructor>::of_not_option);
        let mutation_add_clause =
            FieldsParser::from_vec(mutation_fields).map_field_vec(&|field: Field| {
                <Self as ArgsAddConstructor>::of_option_with("mutation", field)
            });
        quote! {
            let mut args = sqlx::mysql::MySqlArguments::default();
            #(#mutation_add_clause)*
            #(#unique_add_clause)*
            Ok(args)
        }
    }

    fn of_change_args_mysql(&self, location_fields: &Vec<Field>) -> TokenStream {
        let mutation_add_clause = self.map_field_vec(&<Self as ArgsAddConstructor>::of_option);
        let location_add_clause =
            FieldsParser::from_vec(location_fields).map_field_vec(&|field: Field| {
                <Self as ArgsAddConstructor>::of_location_with("location", field)
            });
        quote! {
            let mut args = sqlx::mysql::MySqlArguments::default();
            #(#mutation_add_clause)*
            #(#location_add_clause)*
            Ok(args)
        }
    }
}

pub trait ArgsConstructorPostgres: FieldsContainer + ArgsAddConstructor {
    fn of_maybe_option_args_postgres(&self) -> TokenStream {
        let args_add_clause = self.map_field_vec(&<Self as ArgsAddConstructor>::of_maybe_option);
        quote! {
            let mut args = sqlx::postgres::PgArguments::default();
            #(#args_add_clause)*
            Ok(args)
        }
    }

    fn of_not_option_args_postgres(&self) -> TokenStream {
        let args_add_clause = self.map_field_vec(&<Self as ArgsAddConstructor>::of_not_option);
        quote! {
            let mut args = sqlx::postgres::PgArguments::default();
            #(#args_add_clause)*
            Ok(args)
        }
    }

    fn of_option_args_postgres(&self) -> TokenStream {
        let args_add_clause = self.map_field_vec(&<Self as ArgsAddConstructor>::of_option);
        quote! {
            let mut args = sqlx::postgres::PgArguments::default();
            #(#args_add_clause)*
            Ok(args)
        }
    }

    fn of_location_args_postgres(&self) -> TokenStream {
        let args_add_clause = self.map_field_vec(&<Self as ArgsAddConstructor>::of_location);
        quote! {
            let mut args = sqlx::postgres::PgArguments::default();
            #(#args_add_clause)*
            Ok(args)
        }
    }

    fn of_update_args_postgres(&self, primary_fields: &Vec<Field>) -> TokenStream {
        let mutation_add_clause = self.map_field_vec(&<Self as ArgsAddConstructor>::of_option);
        let primary_add_clause =
            FieldsParser::from_vec(primary_fields).map_field_vec(&|field: Field| {
                <Self as ArgsAddConstructor>::of_not_option_with("primary", field)
            });
        quote! {
            let mut args = sqlx::postgres::PgArguments::default();
            #(#mutation_add_clause)*
            #(#primary_add_clause)*
            Ok(args)
        }
    }

    fn of_unique_update_args_postgres(&self, mutation_fields: &Vec<Field>) -> TokenStream {
        let unique_add_clause = self.map_field_vec(&<Self as ArgsAddConstructor>::of_not_option);
        let mutation_add_clause =
            FieldsParser::from_vec(mutation_fields).map_field_vec(&|field: Field| {
                <Self as ArgsAddConstructor>::of_option_with("mutation", field)
            });
        quote! {
            let mut args = sqlx::postgres::PgArguments::default();
            #(#mutation_add_clause)*
            #(#unique_add_clause)*
            Ok(args)
        }
    }

    fn of_change_args_postgres(&self, location_fields: &Vec<Field>) -> TokenStream {
        let mutation_add_clause = self.map_field_vec(&<Self as ArgsAddConstructor>::of_option);
        let location_add_clause =
            FieldsParser::from_vec(location_fields).map_field_vec(&|field: Field| {
                <Self as ArgsAddConstructor>::of_location_with("location", field)
            });
        quote! {
            let mut args = sqlx::postgres::PgArguments::default();
            #(#mutation_add_clause)*
            #(#location_add_clause)*
            Ok(args)
        }
    }
}
