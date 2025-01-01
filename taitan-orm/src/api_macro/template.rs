#[macro_export]
macro_rules! execute_by_template_fn {
    ($gen_args_fn:path) => {
        async fn execute_by_template(&mut self, template: &dyn crate::traits::TemplateRecord) -> crate::Result<usize> {
            tracing::debug!(target: "taitan_orm", command = "execute_by_template", template = ?template);
            let sql = template.get_sql(None);
            let sql = self.get_generator().post_process(sql);
            tracing::debug!(target: "taitan_orm", command = "execute_by_template", sql = sql);
            let args = ($gen_args_fn)(template)?;
            let result = self.execute(&sql, args).await?;
            tracing::debug!(target: "taitan_orm", command = "execute_by_template", result = ?result);
            Ok(result as usize)
        }
    };
}


#[macro_export]
macro_rules! fetch_one_by_template_fn {
    ($gen_args_fn:path) => {
        async fn fetch_one_by_template<SE>(&mut self, template: &dyn crate::traits::TemplateRecord) -> crate::Result<SE>
        where
            SE: crate::traits::SelectedEntity<Self::DB> + Send + Unpin,
        {
            tracing::debug!(target: "taitan_orm", command = "procedure_by_template", template = ?template);
            let sql = template.get_sql(None);
            let sql = self.get_generator().post_process(sql);
            tracing::debug!(target: "taitan_orm", command = "procedure_by_template", sql = sql);
            let args = ($gen_args_fn)(template)?;
            let result: SE = self.fetch_one_full(&sql, args).await?;
            tracing::debug!(target: "taitan_orm", command = "procedure_by_template", result = ?result);
            Ok(result)
        }
    };
}

#[macro_export]
macro_rules! fetch_option_by_template_fn {
    ($gen_args_fn:path) => {
        async fn fetch_option_by_template<SE>(&mut self, template: &dyn crate::traits::TemplateRecord) -> crate::Result<Option<SE>>
        where
            SE: crate::traits::SelectedEntity<Self::DB> + Send + Unpin,
        {
            tracing::debug!(target: "taitan_orm", command = "select_by_template", template = ?template);
            let sql = template.get_sql(None);
            let sql = self.get_generator().post_process(sql);
            tracing::debug!(target: "taitan_orm", command = "select_by_template", sql = sql);
            let args = ($gen_args_fn)(template)?;
            let result: Option<SE> = self.fetch_option_full(&sql, args).await?;
            tracing::debug!(target: "taitan_orm", command = "select_by_template", result = ?result);
            Ok(result)
        }
    };
}


#[macro_export]
macro_rules! fetch_all_by_template_fn {
    ($gen_args_fn:path) => {
        async fn fetch_all_by_template<SE>(&mut self, template: &dyn crate::traits::TemplateRecord) -> crate::Result<Vec<SE>>
        where
            SE: crate::traits::SelectedEntity<Self::DB> + Send + Unpin,
        {
            tracing::debug!(target: "taitan_orm", command = "search_by_template", template = ?template);
            let sql = template.get_sql(None);
            let sql = self.get_generator().post_process(sql);
            tracing::debug!(target: "taitan_orm", command = "search_by_template", sql = sql);
            let args = ($gen_args_fn)(template)?;
            let result: Vec<SE> = self.fetch_all_full(&sql, args).await?;
            tracing::debug!(target: "taitan_orm", command = "search_by_template", result = ?result);
            Ok(result)
        }
    };
}


#[macro_export]
macro_rules! fetch_paged_by_template_fn {
    ($gen_count_args_fn:path, $gen_args_fn:path) => {
        async fn fetch_paged_by_template<SE>(
            &mut self,
            template: &dyn crate::traits::TemplateRecord,
        ) -> crate::Result<crate::page::PagedList<Self::DB, SE>>
        where
            SE: crate::traits::SelectedEntity<Self::DB> + Send + Unpin,
        {
            tracing::debug!(target: "taitan_orm", command = "search_paged_by_template", template = ?template);
            let count_sql = template
                .get_count_sql()
                .ok_or(crate::TaitanOrmError::TemplatePagedNotHasCountSql)?;
            let count_sql = self.get_generator().post_process(count_sql);
            tracing::debug!(target: "taitan_orm", command = "search_paged_by_template", count_sql = count_sql);
            let page = template
                .get_pagination()
                .ok_or(crate::TaitanOrmError::TemplatePageFieldNotFound)?;

            let count_args = ($gen_count_args_fn)(template)?;
            let count_result_opt: Option<crate::CountResult> =
                self.fetch_option_full(&count_sql, count_args).await?;
            let record_count = count_result_opt.unwrap_or_default().count;
            if record_count <= 0 {
                return Ok(crate::page::PagedList::empty(page.page_size, page.page_num));
            }

            let sql = template.get_sql(Some(page));
            let sql = self.get_generator().post_process(sql);
            tracing::debug!(target: "taitan_orm", command = "search_paged_by_template", sql = sql);
            let args = ($gen_args_fn)(template)?;
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
            tracing::debug!(target: "taitan_orm", command = "search_paged_by_template", result = ?result);
            Ok(result)
        }
    };
}


