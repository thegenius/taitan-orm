use crate::args_extractor::ArgsExtractor;
use crate::prelude::SqlGenericExecutor;
use crate::sql_executor::SqlExecutor;
use crate::sql_generator::SqlGenerator;
use sqlx::{Sqlite, Type};
use taitan_orm_trait::error::TaitanOrmError;
use taitan_orm_trait::page::{build_paged_list, PagedInfo, PagedList, Pagination};
use taitan_orm_trait::result::Result;
use taitan_orm_trait::traits::{Selected, SqliteSelected, SqliteTemplate, Template};
use tracing::debug;
use crate::new_executor::{SqlExecutorMutNew, SqlExecutorNew};
use crate::{template_impl, template_mut_impl};

impl<T> TemplateApiMutNew for T
where
    T: SqlExecutorMutNew<Sqlite>
{
}

pub trait TemplateApiMutNew: SqlExecutorMutNew<Sqlite>
{
    template_mut_impl!(sqlx::Sqlite, SqliteSelected, SqliteTemplate);
    // async fn execute_by_template(&self, template: &dyn SqliteTemplate) -> Result<u64> {
    //     debug!(target: "taitan_orm", command = "execute_by_template", template = ?template);
    //     let (sql, args) = template.get_sql()?;
    //     debug!(target: "taitan_orm", command = "execute_by_template", sql = ?sql);
    //     // let args = Self::extract_template_arguments(template)?;
    //     let result = self.execute(&sql, args).await?;
    //     debug!(target: "taitan_orm", command = "execute_by_template", result = ?result);
    //     Ok(result)
    // }
    //
    // async fn fetch_one_by_template<SE>(&self, template: &dyn SqliteTemplate) -> Result<SE>
    // where
    //     SE: SqliteSelected + Send + Unpin,
    // {
    //     debug!(target: "taitan_orm", command = "procedure_by_template", template = ?template);
    //     let (sql, args) = template.get_sql()?;
    //     debug!(target: "taitan_orm", command = "procedure_by_template", sql = ?sql);
    //     // let args = Self::extract_template_arguments(template)?;
    //     let result: SE = self.fetch_one_full(&sql, args).await?;
    //     debug!(target: "taitan_orm", command = "procedure_by_template", result = ?result);
    //     Ok(result)
    // }
    //
    // async fn fetch_option_by_template<SE>(
    //     &self,
    //     template: &dyn SqliteTemplate,
    // ) -> Result<Option<SE>>
    // where
    //     SE: SqliteSelected + Send + Unpin,
    // {
    //     debug!(target: "taitan_orm", command = "select_by_template", template = ?template);
    //     let (sql, args) = template.get_sql()?;
    //     debug!(target: "taitan_orm", command = "select_by_template", sql = ?sql);
    //     // let args = Self::extract_template_arguments(template)?;
    //     let result: Option<SE> = self.fetch_option_full(&sql, args).await?;
    //     debug!(target: "taitan_orm", command = "select_by_template", result = ?result);
    //     Ok(result)
    // }
    //
    // async fn fetch_all_by_template<SE>(&self, template: &dyn SqliteTemplate) -> Result<Vec<SE>>
    // where
    //     SE: SqliteSelected + Send + Unpin,
    // {
    //     debug!(target: "taitan_orm", command = "search_by_template", template = ?template);
    //     let (sql, args) = template.get_sql()?;
    //     debug!(target: "taitan_orm", command = "search_by_template", sql = ?sql);
    //     // let args = Self::extract_template_arguments(template)?;
    //     let result: Vec<SE> = self.fetch_all_full(&sql, args).await?;
    //     debug!(target: "taitan_orm", command = "search_by_template", result = ?result);
    //     Ok(result)
    // }
    //
    // async fn fetch_paged_by_template<SE>(
    //     &self,
    //     template: &dyn SqliteTemplate,
    //     page: &Pagination,
    // ) -> Result<PagedList<Sqlite, SE>>
    // where
    //     SE: SqliteSelected + Send + Unpin,
    // {
    //     debug!(target: "taitan_orm", command = "search_paged_by_template", template = ?template);
    //     let (count_sql, count_args) = template.get_count_sql()?;
    //     debug!(target: "taitan_orm", command = "search_paged_by_template", count_sql = ?count_sql);
    //     // let page = template
    //     //     .get_pagination()
    //     //     .ok_or(TaitanOrmError::TemplatePageFieldNotFound)?;
    //
    //     // let count_args = Self::extract_template_count_arguments(template)?;
    //     let record_count: u64 = self.fetch_count(&count_sql, count_args).await?;
    //     if record_count <= 0 {
    //         return Ok(PagedList::empty(page.page_size, page.page_num));
    //     }
    //
    //     let (sql, args) = template.get_paged_sql(page)?;
    //     debug!(target: "taitan_orm", command = "search_paged_by_template", sql = ?sql);
    //     // let args = Self::extract_template_arguments(template)?;
    //     let entity_list: Vec<SE> = self.fetch_all_full(&sql, args).await?;
    //
    //     let paged_info = PagedInfo {
    //         page_size: page.page_size,
    //         page_num: page.page_num,
    //         page_total: record_count / page.page_size,
    //         total: record_count,
    //     };
    //     let result = PagedList {
    //         data: entity_list,
    //         page: paged_info,
    //         _phantom: std::marker::PhantomData,
    //     };
    //     debug!(target: "taitan_orm", command = "search_paged_by_template", result = ?result);
    //     Ok(result)
    // }
}
