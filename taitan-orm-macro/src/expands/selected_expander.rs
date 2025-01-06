use crate::attrs::{AttrParser, DefaultAttrParser};
use crate::fields::{FieldsFilter, FieldsParser, NamesConstructor, RowConstructor, StructConstructor};
use case::CaseExt;
use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote};
use syn::{Attribute, FieldsNamed};


// TODO: from_select_row中还使用了ok()函数，应该转化为Error
pub fn generate_selected_impl(
    ident: &Ident,
    attrs: &Vec<Attribute>,
    fields: &FieldsNamed,
) -> TokenStream {

    let table_name = DefaultAttrParser::extract_table_name(ident, attrs);
    let selected_ident = ident;
    let selection_ident = format_ident!("{}Selection", table_name.to_camel());

    let parser = FieldsParser::from_named(fields);
    let sqlite_ident = format_ident!("Sqlite");
    let mysql_ident = format_ident!("MySql");
    let postgres_ident = format_ident!("Postgres");


    let sqlite_impl =
        generate_selected_trait_impl(fields, &selected_ident, &selection_ident, &sqlite_ident);
    let mysql_impl =
        generate_selected_trait_impl(fields, &selected_ident, &selection_ident, &mysql_ident);
    let postgres_impl =
        generate_selected_trait_impl(fields, &selected_ident, &selection_ident, &postgres_ident);

    let bool_names_vec = parser.of_self_optional_names_vec();

    let optional_fields = parser.filter_optional();
    let full_fields_parser = FieldsParser::from_vec(&optional_fields);
    let full_fields_stream = full_fields_parser.of_optional_selected();

    let output = quote! {

        #sqlite_impl

        #mysql_impl

        #postgres_impl

        impl taitan_orm::traits::Selection for #selected_ident {

            fn get_table_name(&self) -> &'static str {
                #table_name
            }

            fn get_selected_fields(&self) -> Vec<String> {
                #bool_names_vec
            }

            fn full_fields() -> Self
                where Self: Sized + Default,
            {
                #full_fields_stream
            }
        }
    };

    output
}

fn generate_selected_trait_impl(
    fields: &FieldsNamed,
    selected_ident: &Ident,
    selection_ident: &Ident,
    db_ident: &Ident,
) -> TokenStream {
    let parser = FieldsParser::from_named(fields);
    let selected_row_construct = parser.gen_selected_row();
    let selected_self_row_construct = parser.gen_selected_self_row();
    let full_row_construct = parser.gen_full_row();

    let output = quote! {
        impl taitan_orm::traits::SelectedEntity<sqlx::#db_ident> for #selected_ident {

            fn select_from_row(selection: &Self, row: <sqlx::#db_ident as sqlx::Database>::Row) -> Result<Self, sqlx::Error>
                where
                    Self: Sized {
                    #selected_self_row_construct
            }

            fn from_row_full(row: <sqlx::#db_ident as sqlx::Database>::Row) -> Result<Self, sqlx::Error>
            where
                Self: Sized,
            {
                #full_row_construct
            }
        }
    };
    output
}
