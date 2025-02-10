use crate::brave_new::ArgsExtractor;
use crate::brave_new::SqlGenerator;
use crate::brave_new::SqlExecutor;
use taitan_orm_trait::brave_new::{
    build_paged_list, PagedInfo, PagedList, Pagination, Selected, Template,
};
use tracing::debug;

impl<T> TemplateApi for T where T: SqlExecutor + SqlGenerator + ArgsExtractor {}

pub trait TemplateApi: SqlExecutor + SqlGenerator + ArgsExtractor {
    async fn execute_by_template(
        &self,
        template: &dyn Template<Self::DB>,
    ) -> crate::result::Result<u64> {
        debug!(target: "taitan_orm", command = "execute_by_template", template = ?template);
        let sql = template.get_sql(None);
        debug!(target: "taitan_orm", command = "execute_by_template", sql = ?sql);
        let args = Self::extract_template_arguments(template)?;
        let result = self.execute(&sql, args).await?;
        debug!(target: "taitan_orm", command = "execute_by_template", result = ?result);
        Ok(result)
    }

    async fn fetch_one_by_template<SE>(
        &self,
        template: &dyn Template<Self::DB>,
    ) -> crate::result::Result<SE>
    where
        SE: Selected<Self::DB> + Send + Unpin,
    {
        debug!(target: "taitan_orm", command = "procedure_by_template", template = ?template);
        let sql = template.get_sql(None);
        debug!(target: "taitan_orm", command = "procedure_by_template", sql = ?sql);
        let args = Self::extract_template_arguments(template)?;
        let result: SE = self.fetch_one_full(&sql, args).await?;
        debug!(target: "taitan_orm", command = "procedure_by_template", result = ?result);
        Ok(result)
    }

    async fn fetch_option_by_template<SE>(
        &self,
        template: &dyn Template<Self::DB>,
    ) -> crate::result::Result<Option<SE>>
    where
        SE: Selected<Self::DB> + Send + Unpin,
    {
        debug!(target: "taitan_orm", command = "select_by_template", template = ?template);
        let sql = template.get_sql(None);
        debug!(target: "taitan_orm", command = "select_by_template", sql = ?sql);
        let args = Self::extract_template_arguments(template)?;
        let result: Option<SE> = self.fetch_option_full(&sql, args).await?;
        debug!(target: "taitan_orm", command = "select_by_template", result = ?result);
        Ok(result)
    }

    async fn fetch_all_by_template<SE>(
        &self,
        template: &dyn Template<Self::DB>,
    ) -> crate::result::Result<Vec<SE>>
    where
        SE: Selected<Self::DB> + Send + Unpin,
    {
        debug!(target: "taitan_orm", command = "search_by_template", template = ?template);
        let sql = template.get_sql(None);
        debug!(target: "taitan_orm", command = "search_by_template", sql = ?sql);
        let args = Self::extract_template_arguments(template)?;
        let result: Vec<SE> = self.fetch_all_full(&sql, args).await?;
        debug!(target: "taitan_orm", command = "search_by_template", result = ?result);
        Ok(result)
    }

    async fn fetch_paged_by_template<SE>(
        &self,
        template: &dyn Template<Self::DB>,
    ) -> crate::result::Result<PagedList<Self::DB, SE>>
    where
        SE: Selected<Self::DB> + Send + Unpin,
    {
        debug!(target: "taitan_orm", command = "search_paged_by_template", template = ?template);
        let count_sql = template
            .get_count_sql()
            .ok_or(crate::error::TaitanOrmError::TemplatePagedNotHasCountSql)?;
        debug!(target: "taitan_orm", command = "search_paged_by_template", count_sql = ?count_sql);
        let page = template
            .get_pagination()
            .ok_or(crate::error::TaitanOrmError::TemplatePageFieldNotFound)?;

        let count_args = Self::extract_template_count_arguments(template)?;
        let record_count: u64 = self.fetch_count(&count_sql, count_args).await?;
        if record_count <= 0 {
            return Ok(PagedList::empty(page.page_size, page.page_num));
        }

        let sql = template.get_sql(Some(page));
        debug!(target: "taitan_orm", command = "search_paged_by_template", sql = ?sql);
        let args = Self::extract_template_arguments(template)?;
        let entity_list: Vec<SE> = self.fetch_all_full(&sql, args).await?;

        let paged_info = PagedInfo {
            page_size: page.page_size,
            page_num: page.page_num,
            page_total: record_count / page.page_size,
            total: record_count,
        };
        let result = PagedList {
            data: entity_list,
            page: paged_info,
            _phantom: std::marker::PhantomData,
        };
        debug!(target: "taitan_orm", command = "search_paged_by_template", result = ?result);
        Ok(result)
    }
}
