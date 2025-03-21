use crate::attrs::{AttrParser, DefaultAttrParser};
use crate::fields::StructConstructor;
use crate::fields::{
    ArgsConstructorMySql, ArgsConstructorPostgres, ArgsConstructorSqlite, FieldsContainer, FieldsParser, NamesConstructor,
};
use case::CaseExt;
use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote};
// use syn::spanned::Spanned;
use syn::{Attribute, Field, FieldsNamed};
use crate::types::{DefaultTypeChecker, DefaultTypeExtractor, TypeChecker, TypeExtractor};

pub fn generate_location_expr_enum_and_impl(
    ident: &Ident,
    attrs: &Vec<Attribute>,
    fields: &FieldsNamed,
    should_serde: bool,
) -> TokenStream {
    let parser = FieldsParser::from_named(fields);

    let where_clause = FieldsParser::from_named(fields).get_enum_where_clause();
    let location_fields_name = parser.of_enum_names_vec();
    let location_arguments_sqlite = FieldsParser::from_named(fields).of_location_enum_args_sqlite();
    let location_arguments_mysql = FieldsParser::from_named(fields).of_location_enum_args_mysql();
    let location_arguments_postgres =
        FieldsParser::from_named(fields).of_location_enum_args_postgres();

    let table_name = DefaultAttrParser::extract_table_name(ident, attrs);
    let struct_name = format!("{}LocationExpr", table_name.to_camel());
    let struct_ident = Ident::new(&struct_name, Span::call_site());
    let struct_stream =
        FieldsParser::from_named(fields).of_location_expr(&struct_name, should_serde);

    // impl UserLocationExpr {
    //     pub fn id(cmp: &str, val: i64) -> Result<Self, NotValidCmpError> {
    //         Ok(Self::Id(taitan_orm::traits::LocationExpr::from(cmp, val)?))
    //     }
    // }

    let expr_builder_fns = parser.map_field_vec(&|field: Field| {
        let field_ident = field.ident.clone().unwrap();
        let field_ident_name = &field_ident.to_string();
        let enum_field_ident = format_ident!("{}", field_ident_name.to_camel());

        let field_ty = field.ty;
        if DefaultTypeChecker::type_is_option(&field_ty) {
            let inner_type = DefaultTypeExtractor::get_option_inner_type(&field_ty).unwrap();
            quote! {
                pub fn #field_ident(cmp: &str, val: #inner_type) -> Result<Self, taitan_orm::error::TaitanOrmError> {
                    Ok(Self::#enum_field_ident(taitan_orm::traits::LocationExpr::from(cmp, val)?))
                }
            }
        } else {
            quote! {
                pub fn #field_ident(cmp: &str, val: #field_ty) -> Result<Self, taitan_orm::error::TaitanOrmError> {
                    Ok(Self::#enum_field_ident(taitan_orm::traits::LocationExpr::from(cmp, val)?))
                }
            }
        }

    });

    let output = quote! {

        #struct_stream

        impl #struct_ident {
            #(#expr_builder_fns)*
        }

        impl taitan_orm::traits::Location for #struct_ident {

            fn get_table_name(&self) -> &'static str {
                #table_name
            }

            // fn get_location_fields_name(&self) -> Vec<taitan_orm::prelude::FieldName> {
            //     #location_fields_name
            // }

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
