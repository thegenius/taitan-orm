use std::fmt::Debug;
use crate::{DatabaseType, FieldMapper, SqlGenerator, TableDef};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

#[derive(Debug, Default)]
pub struct MutationTraitImplGenerator;


impl MutationTraitImplGenerator {

    // pub trait Mutation<DB: Database>: Parameter<DB> +  Debug {
    //     fn gen_update_set_sql<'a>(&self) -> Cow<'a, str>;
    //     fn all_none(&self) -> bool;
    // }
    pub fn generate(
        &self,
        db_type: &DatabaseType,
        table_def: &TableDef,
    ) -> TokenStream {

        let struct_name = &table_def.struct_name;
        let struct_ident = format_ident!("{}", &struct_name);

        let field_mapper = FieldMapper::new();
        // let table_name =  field_mapper.escape(&table_def.table_name, db_type);
        let sql_generator = SqlGenerator::default();
        let stream = sql_generator.gen_update_set_sql(table_def, db_type);
        let db_ident = db_type.gen_ident();
        quote! {
            impl taitan_orm_trait::traits::Mutation<sqlx::#db_ident> for #struct_ident {
                fn gen_update_set_sql<'a>(&self) -> std::borrow::Cow<'a, str> {
                    let s = #stream;
                    std::borrow::Cow::Owned(s)
                }
                fn all_none(&self) -> bool {
                    false
                }
            }
        }
    }
}
