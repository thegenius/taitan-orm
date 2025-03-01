use std::borrow::Cow;
use std::fmt::Debug;
use crate::{DatabaseType, FieldMapper, SqlGenerator, TableDef};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use crate::condition_def::ConditionDef;

#[derive(Debug, Default)]
pub struct SelectedTraitImplGenerator;


impl SelectedTraitImplGenerator {

    // pub trait Selected<DB: Database>: Sized + Default + Debug {
    //
    //     fn gen_select_sql<'a>(&self) -> Cow<'a, str>;
    //
    //     fn gen_select_full_sql<'a>(&self) -> Cow<'a, str>;
    //
    //     fn from_row(selection: &Self, row: DB::Row) -> Result<Self>;
    //
    //     fn from_row_full(row: DB::Row) -> Result<Self>;
    //
    //     fn full_fields() -> Self;
    // }

    // let mut selected = Self::default();
    // let mut i = 0;
    // selected.age = sqlx::Row::try_get(&row, i)?;
    // i += 1;
    // if selection.name.is_selected() {
    // selected.name = taitan_orm::result::Optional::Some(sqlx::Row::try_get(&row, i)?);
    // i += 1;
    // };
    // Ok(selected)
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
        let select_stream = sql_generator.gen_select_sql(table_def, db_type);
        let row_get_stream = field_mapper.gen_row_try_get(&table_def.fields);

        let db_ident = db_type.gen_ident();
        quote! {
            impl taitan_orm_trait::brave_new::selected::Selected<sqlx::#db_ident> for #struct_ident {
                fn gen_select_sql<'a>(&self) -> std::borrow::Cow<'a, str> {
                    let s = #select_stream;
                    std::borrow::Cow::Owned(s)
                }

                fn from_row(selection: &Self, row: <sqlx::#db_ident as sqlx::Database>::Row)
                    -> taitan_orm_trait::brave_new::result::Result<Self> {
                    let mut selected = Self::default();
                    let mut i = 0;
                    #row_get_stream
                    Ok(selected)
                }
            }
        }
    }
}
