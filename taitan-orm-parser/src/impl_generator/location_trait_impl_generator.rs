use crate::{DatabaseType, FieldMapper, SqlGenerator};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use crate::condition_def::ConditionDef;

#[derive(Debug, Default)]
pub struct LocationTraitImplGenerator;

// impl Location<MySql> for UserLocation {
//     fn table_name(&self) -> Cow<'static, str> {
//         Cow::Borrowed("user")
//     }
//     fn gen_where_sql<'a>(&self) -> Cow<'a, str> {
//         let mut sql = String::from("WHERE ");
//         if self.name.is_some() {
//             sql.push_str(" name = ?");
//         }
//         if self.created.is_some() {
//             sql.push_str(" created = ?");
//         }
//         Cow::from(sql)
//     }
//
//
//     fn all_none(&self) -> bool {
//         self.name.is_none() && self.created.is_none()
//     }
// }
impl LocationTraitImplGenerator {
    pub fn generate(
        &self,
        db_type: &DatabaseType,
        condition_def: &ConditionDef,
    ) -> TokenStream {

        let struct_name = &condition_def.struct_name;
        let struct_ident = format_ident!("{}", &struct_name);


        let field_mapper = FieldMapper::new();
        let table_name =  field_mapper.escape(&condition_def.table_name, db_type);
        let sql_generator = SqlGenerator::default();
        let stream = sql_generator.gen_where_sql(condition_def, db_type);
        let db_ident = db_type.gen_ident();
        quote! {
            impl taitan_orm::traits::Location<sqlx::#db_ident> for #struct_ident {
                fn table_name(&self) -> std::borrow::Cow<'static, str> {
                    std::borrow::Cow::Borrowed(#table_name)
                }
                fn gen_where_sql<'a>(&self) -> std::borrow::Cow<'a, str> {
                    #stream
                }
                fn all_none(&self) -> bool {
                    false
                }
            }
        }
    }
}
