use crate::{DatabaseType, FieldMapper, TableDef};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

#[derive(Debug, Default)]
pub struct TemplateTraitImplGenerator;
impl TemplateTraitImplGenerator {
    pub fn generate(&self, db_type: &DatabaseType, table_def: &TableDef) -> TokenStream {
        let struct_name = &table_def.struct_name;
        let struct_ident = format_ident!("{}", &struct_name);
        let db_ident = db_type.gen_ident();
        match db_type {
            DatabaseType::Postgres => quote! {
                impl taitan_orm::traits::Template<sqlx::#db_ident> for #struct_ident {
                    fn get_sql(
                        &self,
                    ) -> taitan_orm::result::Result<(String, <sqlx::#db_ident as sqlx::Database>::Arguments<'_>)>
                    {
                        taitan_orm::traits::TemplateRenderTrait::<sqlx::#db_ident>::gen_indexed_sql(self)
                    }

                    fn get_paged_sql(
                        &self,
                        pagination: &taitan_orm_trait::page::Pagination,
                    ) -> taitan_orm::result::Result<(String, <sqlx::#db_ident as sqlx::Database>::Arguments<'_>)>
                    {
                        taitan_orm::traits::TemplateRenderTrait::<sqlx::#db_ident>::gen_indexed_paged_sql(self, pagination)
                    }

                    fn get_count_sql(
                        &self,
                    ) -> taitan_orm::result::Result<(String, <sqlx::#db_ident as sqlx::Database>::Arguments<'_>)>
                    {
                        taitan_orm::traits::TemplateRenderTrait::<sqlx::#db_ident>::gen_indexed_count_sql(self)
                    }
                }
            },
            DatabaseType::MySql => quote! {
                impl taitan_orm::traits::Template<sqlx::#db_ident> for #struct_ident {
                    fn get_sql(
                        &self,
                    ) -> taitan_orm::result::Result<(String, <sqlx::#db_ident as sqlx::Database>::Arguments<'_>)>
                    {
                        taitan_orm::traits::TemplateRenderTrait::<sqlx::#db_ident>::gen_sql(self)
                    }

                    fn get_paged_sql(
                        &self,
                        pagination: &taitan_orm_trait::page::Pagination,
                    ) -> taitan_orm::result::Result<(String, <sqlx::#db_ident as sqlx::Database>::Arguments<'_>)>
                    {
                        taitan_orm::traits::TemplateRenderTrait::<sqlx::#db_ident>::gen_paged_sql(self, pagination)
                    }

                    fn get_count_sql(
                        &self,
                    ) -> taitan_orm::result::Result<(String, <sqlx::#db_ident as sqlx::Database>::Arguments<'_>)>
                    {
                        taitan_orm::traits::TemplateRenderTrait::<sqlx::#db_ident>::gen_count_sql(self)
                    }
                }
            },
            // Sqlite的ge_paged_sql的生命周期需要特殊处理
            DatabaseType::Sqlite => quote! {
                impl taitan_orm::traits::Template<sqlx::#db_ident> for #struct_ident {
                    fn get_sql(
                        &self,
                    ) -> taitan_orm::result::Result<(String, <sqlx::#db_ident as sqlx::Database>::Arguments<'_>)>
                    {
                        taitan_orm::traits::TemplateRenderTrait::<sqlx::#db_ident>::gen_sql(self)
                    }

                    fn get_paged_sql<'a>(
                        &'a self,
                        pagination: &'a taitan_orm_trait::page::Pagination,
                    ) -> taitan_orm::result::Result<(String, <sqlx::#db_ident as sqlx::Database>::Arguments<'a>)>
                    {
                        taitan_orm::traits::TemplateRenderTrait::<sqlx::#db_ident>::gen_paged_sql(self, pagination)
                    }

                    fn get_count_sql(
                        &self,
                    ) -> taitan_orm::result::Result<(String, <sqlx::#db_ident as sqlx::Database>::Arguments<'_>)>
                    {
                        taitan_orm::traits::TemplateRenderTrait::<sqlx::#db_ident>::gen_count_sql(self)
                    }
                }
            },
        }
    }
}
