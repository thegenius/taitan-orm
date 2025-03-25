use case::CaseExt;
use crate::fields::mappers::ArgsAddConstructor;
use crate::fields::{FieldsContainer, FieldsParser};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::Field;

pub trait ArgsConstructorSqlite: FieldsContainer + ArgsAddConstructor {

    //fn gen_location_arguments_sqlite(
    //         &self,
    //     ) -> Result<sqlx::sqlite::SqliteArguments<'_>, sqlx::error::BoxDynError> {
    //         let mut args = SqliteArguments::default();
    //         match self {
    //             UserIndexUser::Id { id, .. } => {
    //                 args.add(&id.val)?;
    //             }
    //             UserIndexUser::IdName { id, name, .. } => {
    //                 args.add(&id.val)?;
    //                 args.add(&name.val)?;
    //             }
    //         }
    //         Ok(args)
    //     }
    fn of_index_enum_args(
        &self,
        struct_name: &str,
    ) -> TokenStream {
        let struct_ident = format_ident!("{}", struct_name);
        let fields = self.get_fields();
        let mut variants :Vec<TokenStream> = Vec::new();
        for i in 0..fields.len() {
            let enum_fields = fields[0..=i].to_vec();

            let enum_fields_clone = enum_fields.clone();
            let variant_fields = enum_fields_clone.iter().map(|field|field.ident.as_ref().unwrap()).collect::<Vec<_>>();

            let variant_name = enum_fields.clone()
                .iter()
                .map(|field| field.ident.as_ref().unwrap().to_string().to_camel())
                .collect::<Vec<String>>()
                .join("");
            let variant_ident = format_ident!("{}", variant_name);

            let add_stmts = enum_fields
                .into_iter()
                .map(Self::of_location_not_optional)
                .collect::<Vec<TokenStream>>();

            let variant = quote! {
                #struct_ident::#variant_ident{#(#variant_fields,)*.. } =>{
                    #(#add_stmts)*
                }
            };
            variants.push(variant);
        };
        quote!{
            #(#variants,)*
        }
    }

    //fn gen_location_arguments_sqlite(
    //         &self,
    //     ) -> Result<sqlx::sqlite::SqliteArguments<'_>, sqlx::error::BoxDynError> {
    //         let mut args = SqliteArguments::default();
    //         match self {
    //             #(#()*)
    //         }
    //         Ok(args)
    //     }
    fn of_index_enum_args_sqlite(
        &self,
        struct_name: &str,
    ) -> TokenStream {
        let variant_stream = self.of_index_enum_args(struct_name);
        quote! {
            fn gen_location_arguments_sqlite(&self) -> Result<sqlx::sqlite::SqliteArguments<'_>, sqlx::error::BoxDynError> {
                let mut args = sqlx::sqlite::SqliteArguments::default();
                match self {
                    #variant_stream
                }
                Ok(args)
            }
        }
    }

    fn of_index_enum_args_mysql(
        &self,
        struct_name: &str,
    ) -> TokenStream {
        let variant_stream = self.of_index_enum_args(struct_name);
        quote! {
            fn gen_location_arguments_mysql(&self) -> Result<sqlx::mysql::MySqlArguments, sqlx::error::BoxDynError> {
                let mut args = sqlx::mysql::MySqlArguments::default();
                match self {
                    #variant_stream
                }
                Ok(args)
            }
        }
    }

    fn of_index_enum_args_postgres(
        &self,
        struct_name: &str,
    ) -> TokenStream {
        let variant_stream = self.of_index_enum_args(struct_name);
        quote! {
            fn gen_location_arguments_postgres(&self) -> Result<sqlx::postgres::PgArguments, sqlx::error::BoxDynError> {
                let mut args = sqlx::postgres::PgArguments::default();
                match self {
                    #variant_stream
                }
                Ok(args)
            }
        }
    }

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

    fn of_location_enum_args_sqlite(&self) -> TokenStream {
        let args_add_clause = self.map_field_vec(&<Self as ArgsAddConstructor>::of_location_enum);
        quote! {
            let mut args = sqlx::sqlite::SqliteArguments::default();
            match self {
                #(#args_add_clause)*
            }
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

    fn of_location_enum_args_mysql(&self) -> TokenStream {
        let args_add_clause = self.map_field_vec(&<Self as ArgsAddConstructor>::of_location_enum);
        quote! {
            let mut args = sqlx::mysql::MySqlArguments::default();
            match self {
                #(#args_add_clause)*
            }
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

    fn of_location_enum_args_postgres(&self) -> TokenStream {
        let args_add_clause = self.map_field_vec(&<Self as ArgsAddConstructor>::of_location_enum);
        quote! {
            let mut args = sqlx::postgres::PgArguments::default();
            match self {
                #(#args_add_clause)*
            }
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
