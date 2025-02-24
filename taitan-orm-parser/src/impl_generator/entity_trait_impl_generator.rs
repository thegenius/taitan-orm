use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use crate::{DatabaseType, SqlGenerator, TableDef};

#[derive(Clone, Debug, Default)]
pub struct  EntityTraitImplGenerator;

// fn gen_insert_sql<'a>(&self) -> Cow<'a, str> {
//     let sql = "insert into users (name, created) values (?, ?)";
//     Cow::from(sql)
// }
//
// fn gen_upsert_sql<'a>(&self) -> Cow<'a, str> {
//     let sql = "insert into users (name, created) values (?, ?) on conflict (name) do update set created = ?";
//     Cow::from(sql)
// }
//
// fn gen_create_sql<'a>(&self) -> Cow<'a, str> {
//     let sql = "insert into users (name, created) values (?, ?)";
//     Cow::from(sql)
// }
impl EntityTraitImplGenerator {
    pub fn generate(&self, db_type: &DatabaseType, table_def: &TableDef) -> TokenStream {
        let struct_ident = format_ident!("{}", table_def.struct_name);
        let db_ident = db_type.gen_ident();
        let generator = SqlGenerator::default();
        let insert_sql = generator.gen_insert_sql(&table_def, &db_type);
        let upsert_sql = generator.gen_upsert_sql(&table_def, &db_type);

        quote! {
            impl taitan_orm_trait::brave_new::Entity<sqlx::#db_ident> for #struct_ident {
                fn gen_insert_sql<'a>(&self) -> std::borrow::Cow<'a, str> {
                    #insert_sql
                }
                fn gen_upsert_sql<'a>(&self) -> std::borrow::Cow<'a, str>{
                    #upsert_sql
                }
                fn gen_create_sql<'a>(&self) -> std::borrow::Cow<'a, str>{
                    todo!()
                }
            }
        }
    }
}