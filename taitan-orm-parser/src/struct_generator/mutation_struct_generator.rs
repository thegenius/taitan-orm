use crate::condition_def::ConditionDef;
use crate::{DatabaseType, FieldMapper, SqlGenerator, TableDef};
use case::CaseExt;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::borrow::Cow;
use std::fmt::Debug;

#[derive(Debug, Default)]
pub struct MutationStructGenerator;

impl MutationStructGenerator {
    pub fn generate(&self, table_def: &TableDef) -> TokenStream {
        let struct_name = &table_def.struct_name;
        let struct_ident = format_ident!("{}Mutation", struct_name);

        let field_mapper = FieldMapper::new();
        // let table_name =  field_mapper.escape(&table_def.table_name, db_type);
        let sql_generator = SqlGenerator::default();
        let fields = table_def.get_not_primary_fields();
        let fields_stream = field_mapper.gen_struct_fields(fields, true);
        quote! {
            #[derive(Clone, Debug, taitan_orm_macro::Parameter, taitan_orm_macro::MutationNew)]
            pub struct #struct_ident {
                #fields_stream
            }
        }
    }
}
