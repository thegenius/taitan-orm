use crate::extractor::Extractor;
use crate::{SqlExecutor, SqlGenerator, SqlGeneratorContainer};
use tracing::debug;

impl<T> TemplateApi for T where T: SqlExecutor + SqlGeneratorContainer + Extractor {}

pub trait TemplateApi: SqlExecutor + SqlGeneratorContainer + Extractor {
    async fn execute_by_template(
        &mut self,
        template: &dyn crate::traits::TemplateRecord,
    ) -> crate::Result<u64> {
        debug!(target: "taitan_orm", command = "execute_by_template", template = ?template);
        let sql = template.get_sql(None);
        let sql = self.get_generator().post_process(sql);
        debug!(target: "taitan_orm", command = "execute_by_template", sql = sql);
        let args = Self::extract_template_arguments(template)?;
        let result = self.execute(&sql, args).await?;
        debug!(target: "taitan_orm", command = "execute_by_template", result = ?result);
        Ok(result)
    }

    async fn fetch_one_by_template<SE>(
        &mut self,
        template: &dyn crate::traits::TemplateRecord,
    ) -> crate::Result<SE>
    where
        SE: crate::traits::SelectedEntity<Self::DB> + Send + Unpin,
    {
        debug!(target: "taitan_orm", command = "procedure_by_template", template = ?template);
        let sql = template.get_sql(None);
        let sql = self.get_generator().post_process(sql);
        debug!(target: "taitan_orm", command = "procedure_by_template", sql = sql);
        let args = Self::extract_template_arguments(template)?;
        let result: SE = self.fetch_one_full(&sql, args).await?;
        debug!(target: "taitan_orm", command = "procedure_by_template", result = ?result);
        Ok(result)
    }

    async fn fetch_option_by_template<SE>(
        &mut self,
        template: &dyn crate::traits::TemplateRecord,
    ) -> crate::Result<Option<SE>>
    where
        SE: crate::traits::SelectedEntity<Self::DB> + Send + Unpin,
    {
        debug!(target: "taitan_orm", command = "select_by_template", template = ?template);
        let sql = template.get_sql(None);
        let sql = self.get_generator().post_process(sql);
        debug!(target: "taitan_orm", command = "select_by_template", sql = sql);
        let args = Self::extract_template_arguments(template)?;
        let result: Option<SE> = self.fetch_option_full(&sql, args).await?;
        debug!(target: "taitan_orm", command = "select_by_template", result = ?result);
        Ok(result)
    }

    async fn fetch_all_by_template<SE>(
        &mut self,
        template: &dyn crate::traits::TemplateRecord,
    ) -> crate::Result<Vec<SE>>
    where
        SE: crate::traits::SelectedEntity<Self::DB> + Send + Unpin,
    {
        debug!(target: "taitan_orm", command = "search_by_template", template = ?template);
        let sql = template.get_sql(None);
        let sql = self.get_generator().post_process(sql);
        debug!(target: "taitan_orm", command = "search_by_template", sql = sql);
        let args = Self::extract_template_arguments(template)?;
        let result: Vec<SE> = self.fetch_all_full(&sql, args).await?;
        debug!(target: "taitan_orm", command = "search_by_template", result = ?result);
        Ok(result)
    }

    async fn fetch_paged_by_template<SE>(
        &mut self,
        template: &dyn crate::traits::TemplateRecord,
    ) -> crate::Result<crate::page::PagedList<Self::DB, SE>>
    where
        SE: crate::traits::SelectedEntity<Self::DB> + Send + Unpin,
    {
        debug!(target: "taitan_orm", command = "search_paged_by_template", template = ?template);
        let count_sql = template
            .get_count_sql()
            .ok_or(crate::TaitanOrmError::TemplatePagedNotHasCountSql)?;
        let count_sql = self.get_generator().post_process(count_sql);
        debug!(target: "taitan_orm", command = "search_paged_by_template", count_sql = count_sql);
        let page = template
            .get_pagination()
            .ok_or(crate::TaitanOrmError::TemplatePageFieldNotFound)?;

        let count_args = Self::extract_template_count_arguments(template)?;
        let record_count: u64 = self.fetch_count(&count_sql, count_args).await?;
        if record_count <= 0 {
            return Ok(crate::page::PagedList::empty(page.page_size, page.page_num));
        }

        let sql = template.get_sql(Some(page));
        let sql = self.get_generator().post_process(sql);
        debug!(target: "taitan_orm", command = "search_paged_by_template", sql = sql);
        let args = Self::extract_template_arguments(template)?;
        let entity_list: Vec<SE> = self.fetch_all_full(&sql, args).await?;

        let paged_info = crate::page::PagedInfo {
            page_size: page.page_size,
            page_num: page.page_num,
            page_total: record_count / page.page_size,
            total: record_count,
        };
        let result = crate::page::PagedList {
            data: entity_list,
            page: paged_info,
            _phantom: std::marker::PhantomData,
        };
        debug!(target: "taitan_orm", command = "search_paged_by_template", result = ?result);
        Ok(result)
    }
}
