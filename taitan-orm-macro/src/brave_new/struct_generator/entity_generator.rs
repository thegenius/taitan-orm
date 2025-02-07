use proc_macro2::TokenStream;
use crate::brave_new::table_def::TableDef;

pub trait EntityGenerator {

    fn generate(table_def: &TableDef) -> TokenStream;
}