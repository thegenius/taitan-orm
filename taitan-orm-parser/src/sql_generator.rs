use crate::{DatabaseType, FieldMapper, SqlType, TableDef};
use proc_macro2::TokenStream;
use quote::quote;


#[derive(Debug, Default)]
pub struct SqlGenerator;
impl SqlGenerator {

    pub fn gen_sql(&self, sql_type: &SqlType, table: &TableDef, db_type: &DatabaseType) -> TokenStream {
        match sql_type {
            SqlType::Insert => {
                self.gen_insert_sql(table, db_type)
            }
            SqlType::Upsert => {
                self.gen_upsert_sql(table, db_type)
            }
        }
    }

    pub fn gen_insert_sql(&self, table_def: &TableDef, db_type: &DatabaseType) -> TokenStream {
        let field_mapper = FieldMapper::new();
        let table_name = field_mapper.escape(&table_def.table_name, db_type);
        let fields = field_mapper.gen_names(&table_def.fields, &db_type);
        let marks = field_mapper.gen_marks(&table_def.fields, &db_type);
        let sql_template = format!("INSERT INTO {table_name} ({{}}) VALUES({{}})");
        quote! {
            let fields = #fields;
            let marks = #marks;
            format!(#sql_template, fields, marks)
        }
    }
    pub fn gen_upsert_sql(&self, table_def: &TableDef, db_type: &DatabaseType) -> TokenStream {
        let field_mapper = FieldMapper::new();
        let table_name = field_mapper.escape(&table_def.table_name, db_type);
        let fields = field_mapper.gen_names(&table_def.fields, db_type);
        let primary_fields = table_def.get_primary_fields();
        let primary_fields_stream = field_mapper.gen_names(primary_fields, db_type);
        let non_primary_fields = table_def.get_not_primary_fields();
        let upsert_sets_stream = field_mapper.gen_upsert_sets(non_primary_fields, db_type);

        let marks = field_mapper.gen_marks(&table_def.fields, db_type);
        match db_type {
            DatabaseType::MySql => {
                let sql= format!("INSERT INTO {table_name} ({{}}) VALUES({{}}) ON DUPLICATE KEY UPDATE {{}}");
                quote! {
                    let fields = #fields;
                    let marks = #marks;
                    let upsert_sets = #upsert_sets_stream;
                    format!(#sql, fields, marks, upsert_sets)
                }
            }
            DatabaseType::Postgres => {
                let sql = format!("INSERT INTO {table_name} ({{}}) VALUES({{}}) ON CONFLICT ({{}}) DO UPDATE SET {{}}");
                quote! {
                    let fields = #fields;
                    let marks = #marks;
                    let primarys = #primary_fields_stream;
                    let upsert_sets = #upsert_sets_stream;
                    format!(#sql, fields, marks, primarys, upsert_sets)
                }
            }
            DatabaseType::Sqlite => {
                let sql = format!("INSERT INTO {table_name} ({{}}) VALUES({{}}) ON CONFLICT ({{}}) DO UPDATE SET {{}}");
                quote! {
                    let fields = #fields;
                    let marks = #marks;
                    let primarys = #primary_fields_stream;
                    let upsert_sets = #upsert_sets_stream;
                    format!(#sql, fields, marks, primarys, upsert_sets)
                }
            }
        }


    }
}
