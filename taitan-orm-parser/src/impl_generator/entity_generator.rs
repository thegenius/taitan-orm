use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use crate::TableDef;

pub struct  EntityGenerator;

// pub trait Entity<DB: Database>: Debug {
//     fn gen_insert_sql<'a>(&self) -> std::borrow::Cow<'a, str>;
//     fn gen_upsert_sql<'a>(&self) -> std::borrow::Cow<'a, str>;
//     fn gen_create_sql<'a>(&self) -> std::borrow::Cow<'a, str>;
//     fn add_insert_args<'a>(&'a self, args: &mut DB::Arguments<'a>) -> taitan_orm::result::Result<()>;
//     fn add_upsert_args<'a>(&'a self, args: &mut DB::Arguments<'a>) -> taitan_orm::result::Result<()>;
// }
impl EntityGenerator {
    pub fn generator(table_def: TableDef) -> TokenStream {
        let struct_ident = format_ident!("{}", table_def.struct_name);

        quote! {
            impl taitan_orm::traits::Entity for #struct_ident {
                fn gen_insert_sql<'a>(&self) -> std::borrow::Cow<'a, str> {
                    todo!()
                }
                fn gen_upsert_sql<'a>(&self) -> std::borrow::Cow<'a, str>{
                    todo!()
                }
                fn gen_create_sql<'a>(&self) -> std::borrow::Cow<'a, str>{
                    todo!()
                }
                fn add_insert_args<'a>(&'a self, args: &mut DB::Arguments<'a>) -> taitan_orm::result::Result<()>{
                    todo!()
                }
                fn add_upsert_args<'a>(&'a self, args: &mut DB::Arguments<'a>) -> taitan_orm::result::Result<()>{
                    todo!()
                }
            }
        }
    }
}