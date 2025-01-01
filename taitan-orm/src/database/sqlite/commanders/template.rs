

use crate::{execute_by_template_fn, fetch_all_by_template_fn, fetch_one_by_template_fn, fetch_option_by_template_fn, fetch_paged_by_template_fn};
use crate::{SqlExecutor, SqlGenerator,SqlGeneratorContainer};


pub trait SqliteTemplateCommander: SqlExecutor<DB = sqlx::Sqlite> + SqlGeneratorContainer {
    // async fn execute_by_template(&mut self, template: &dyn TemplateRecord) -> Result<usize> {
    //     debug!(target: "taitan_orm", command = "execute_by_template", template = ?template);
    //     let sql = template.get_sql(None);
    //     let sql = self.get_generator().post_process(sql);
    //     debug!(target: "taitan_orm", command = "execute_by_template", sql = sql);
    //     let args = template.gen_template_arguments_sqlite()?;
    //     let result = self.execute::<SqliteArguments>(&sql, args).await?;
    //     debug!(target: "taitan_orm", command = "execute_by_template", result = ?result);
    //     Ok(result as usize)
    // }
    execute_by_template_fn!(crate::traits::TemplateRecord::gen_template_arguments_sqlite);

    // async fn fetch_one_by_template<SE>(&mut self, template: &dyn TemplateRecord) -> Result<SE>
    // where
    //     SE: SelectedEntity<Self::DB> + Send + Unpin,
    // {
    //     debug!(target: "taitan_orm", command = "procedure_by_template", template = ?template);
    //     let sql = template.get_sql(None);
    //     let sql = self.get_generator().post_process(sql);
    //     debug!(target: "taitan_orm", command = "procedure_by_template", sql = sql);
    //     let args = template.gen_template_arguments_sqlite()?;
    //     let result: SE = self.fetch_execute(&sql, args).await?;
    //     debug!(target: "taitan_orm", command = "procedure_by_template", result = ?result);
    //     Ok(result)
    // }
    fetch_one_by_template_fn!(crate::traits::TemplateRecord::gen_template_arguments_sqlite);

    // async fn fetch_option_by_template<SE>(&mut self, template: &dyn TemplateRecord) -> Result<Option<SE>>
    // where
    //     SE: SelectedEntity<Self::DB> + Send + Unpin,
    // {
    //     debug!(target: "taitan_orm", command = "select_by_template", template = ?template);
    //     let sql = template.get_sql(None);
    //     let sql = self.get_generator().post_process(sql);
    //     debug!(target: "taitan_orm", command = "select_by_template", sql = sql);
    //     let args = template.gen_template_arguments_sqlite()?;
    //     let result: Option<SE> = self.fetch_execute_option(&sql, args).await?;
    //     debug!(target: "taitan_orm", command = "select_by_template", result = ?result);
    //     Ok(result)
    // }
    fetch_option_by_template_fn!(crate::traits::TemplateRecord::gen_template_arguments_sqlite);

    // async fn fetch_all_by_template<SE>(&mut self, template: &dyn TemplateRecord) -> Result<Vec<SE>>
    // where
    //     SE: SelectedEntity<Self::DB> + Send + Unpin,
    // {
    //     debug!(target: "taitan_orm", command = "search_by_template", template = ?template);
    //     let sql = template.get_sql(None);
    //     let sql = self.get_generator().post_process(sql);
    //     debug!(target: "taitan_orm", command = "search_by_template", sql = sql);
    //     let args = template.gen_template_arguments_sqlite()?;
    //     let result: Vec<SE> = self.fetch_execute_all(&sql, args).await?;
    //     debug!(target: "taitan_orm", command = "search_by_template", result = ?result);
    //     Ok(result)
    // }
    fetch_all_by_template_fn!(crate::traits::TemplateRecord::gen_template_arguments_sqlite);

    // async fn fetch_paged_by_template<SE>(
    //     &mut self,
    //     template: &dyn TemplateRecord,
    // ) -> Result<PagedList<Self::DB, SE>>
    // where
    //     SE: SelectedEntity<Self::DB> + Send + Unpin,
    // {
    //     debug!(target: "taitan_orm", command = "search_paged_by_template", template = ?template);
    //     let count_sql = template
    //         .get_count_sql()
    //         .ok_or(TaitanOrmError::TemplatePagedNotHasCountSql)?;
    //     let count_sql = self.get_generator().post_process(count_sql);
    //     debug!(target: "taitan_orm", command = "search_paged_by_template", count_sql = count_sql);
    //     let page = template
    //         .get_pagination()
    //         .ok_or(TaitanOrmError::TemplatePageFieldNotFound)?;
    //
    //     let count_args = template.gen_template_count_arguments_sqlite()?;
    //     let count_result_opt: Option<CountResult> =
    //         self.fetch_execute_option(&count_sql, count_args).await?;
    //     let record_count = count_result_opt.unwrap_or_default().count;
    //     if record_count <= 0 {
    //         return Ok(PagedList::empty(page.page_size, page.page_num));
    //     }
    //
    //     let sql = template.get_sql(Some(page));
    //     let sql = self.get_generator().post_process(sql);
    //     debug!(target: "taitan_orm", command = "search_paged_by_template", sql = sql);
    //     let args = template.gen_template_arguments_sqlite()?;
    //     let entity_list: Vec<SE> = self.fetch_execute_all(&sql, args).await?;
    //
    //     let page_info = PageInfo {
    //         page_size: page.page_size,
    //         page_num: page.page_num,
    //         page_total: record_count / page.page_size,
    //         total: record_count,
    //     };
    //     let result = PagedList {
    //         data: entity_list,
    //         page: page_info,
    //         _phantom: PhantomData,
    //     };
    //     debug!(target: "taitan_orm", command = "search_paged_by_template", result = ?result);
    //     Ok(result)
    // }
    fetch_paged_by_template_fn!(crate::traits::TemplateRecord::gen_template_count_arguments_sqlite, crate::traits::TemplateRecord::gen_template_arguments_sqlite);
}
