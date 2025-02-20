use crate::{DatabaseType, FieldMapper, TableDef};
use proc_macro2::TokenStream;
use quote::quote;

#[derive(Debug, Default)]
pub struct SqlGenerator;
impl SqlGenerator {
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
        let fields = field_mapper.gen_names(&table_def.fields, &db_type);
        let primary_fields = table_def.get_primary_fields();
        

        let marks = field_mapper.gen_marks(&table_def.fields, &db_type);
        let sql_template = format!("INSERT INTO {table_name} ({{}}) VALUES({{}})");
        quote! {
            let fields = #fields;
            let marks = #marks;
            format!(#sql_template, fields, marks)
        }
    }
}