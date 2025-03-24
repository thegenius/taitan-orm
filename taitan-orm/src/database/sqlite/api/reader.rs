use super::super::args_extractor::SqliteArgsExtractor;
use crate::args_extractor::ArgsExtractor;
use crate::database::sqlite::sql_generator::SqliteSqlGenerator;
use crate::new_executor::{SqlExecutorNew, SqlGeneratorNew};
use crate::prelude::SqlGenericExecutor;
use crate::sql_executor::SqlExecutor;
use crate::sql_generator::SqlGenerator;
use sqlx::{Arguments, Database, Sqlite, Type};
use taitan_orm_trait::order::OrderBy;
use taitan_orm_trait::page::{build_paged_list, PagedInfo, PagedList, Pagination};
use taitan_orm_trait::result::Result;
use taitan_orm_trait::traits::{
    Location, Mutation, Parameter, Selected, SqliteLocation, SqliteMutation, SqliteSelected,
    SqliteUnique, Unique,
};
impl<T> ReaderApiNew for T where T: SqlExecutorNew<Sqlite> {}

// ### 4 Main Read API
// Because unique only locate to 0-1 record, so order and page is not needed.
// ```
// select       (selection, unique               ) -> Optional<SE>
// search       (selection, location, order, page) -> Vec<SE>
// search_all   (selection, location, order      ) -> Vec<SE>
// search_paged (selection, location, order, page) -> PagedList<SE>
// ```
//
// ### 6 Sugar Read API
// Other read APIs are just syntactic sugar and maybe some performance optimize.
// ```
// # equals to: select(selection::full, unique).is_some()
// exists(unique) -> bool
//
// # equals to: select_all(selection::full, location).len()
// count(location) -> u64
//
// # equals to: select(selection::full, unique) -> Optional<SE>
// select_full(unique) -> Optional<SE>
//
// # equals to: search(selection::full, location, order, page) -> Vec<SE>
// search_full(location, order, page) -> Vec<SE>
//
// # equals to: search_all(selection::full, order) -> Vec<SE>
// search_full_all(location, order) -> Vec<SE>
//
// # equals to: search_paged(selection::full, location, order, page) -> PagedList<SE>
// search_full_paged(location, order, page) -> PagedList<SE>
// ```
pub trait ReaderApiNew: SqlExecutorNew<Sqlite> {
    async fn select<SE, M>(
        &self,
        selection: &SE,
        unique: &dyn SqliteUnique<Mutation = M>,
    ) -> Result<Option<SE>>
    where
        M: SqliteMutation,
        SE: SqliteSelected + Send + Unpin,
    {
        tracing::debug!(target: "taitan_orm", command = "select", primary = ?unique, selection = ?selection);
        let sql = SqliteSqlGenerator::gen_select_sql(selection, unique);
        tracing::debug!(target: "taitan_orm", command = "select", sql = sql);
        let args = unique.gen_args()?;
        let result: Option<SE> =
            <Self as SqlExecutorNew<Sqlite>>::fetch_option_(self, &sql, selection, args).await?;
        tracing::debug!(target: "taitan_orm", command = "select", result = ?result);
        Ok(result)
    }

    async fn search<SE>(
        &self,
        selection: &SE,
        location: &dyn SqliteLocation,
        order_by: &dyn OrderBy,
        page: &Pagination,
    ) -> Result<Vec<SE>>
    where
        SE: SqliteSelected + Send + Unpin,
    {
        tracing::debug!(target: "taitan_orm", command = "search", location = ?location, order_by = ?order_by, selection = ?selection);
        let sql = SqliteSqlGenerator::gen_search_sql(selection, location, order_by, page);
        tracing::debug!(target: "taitan_orm", command = "search", sql = sql);
        let mut args = location.gen_args()?;
        <Pagination as Parameter<Sqlite>>::add_to_args(page, &mut args)?;
        let result: Vec<SE> =
            <Self as SqlExecutorNew<Sqlite>>::fetch_all_(self, &sql, selection, args).await?;
        tracing::debug!(target: "taitan_orm", command = "search", result = ?result);
        Ok(result)
    }

    async fn search_all<SE>(
        &self,
        selection: &SE,
        location: &dyn SqliteLocation,
        order_by: &dyn OrderBy,
    ) -> Result<Vec<SE>>
    where
        SE: SqliteSelected + Send + Unpin,
    {
        tracing::debug!(target: "taitan_orm", command = "search", location = ?location, order_by = ?order_by, selection = ?selection);
        let sql = SqliteSqlGenerator::gen_search_all_sql(selection, location, order_by);
        tracing::debug!(target: "taitan_orm", command = "search", sql = sql);
        let args = location.gen_args()?;
        let result: Vec<SE> =
            <Self as SqlExecutorNew<Sqlite>>::fetch_all_(self, &sql, selection, args).await?;
        tracing::debug!(target: "taitan_orm", command = "search", result = ?result);
        Ok(result)
    }

    async fn search_paged<SE>(
        &self,
        selection: &SE,
        location: &dyn SqliteLocation,
        order_by: &dyn OrderBy,
        page: &Pagination,
    ) -> Result<PagedList<Sqlite, SE>>
    where
        SE: SqliteSelected + Send + Unpin,
    {
        tracing::debug!(target: "taitan_orm", command = "search_paged", location = ?location, order_by = ?order_by, selection = ?selection, page = ?page);
        let record_count = self.count(location).await?;
        if record_count <= 0 {
            return Ok(PagedList::empty(page.page_size, page.page_num));
        }

        let sql = SqliteSqlGenerator::gen_search_sql(selection, location, order_by, page);
        tracing::debug!(target: "taitan_orm", command = "search_paged", sql = sql);
        let mut args = location.gen_args()?;
        <Pagination as Parameter<Sqlite>>::add_to_args(page, &mut args)?;
        let entity_list: Vec<SE> =
            <Self as SqlExecutorNew<Sqlite>>::fetch_all_(self, &sql, selection, args).await?;
        let result = build_paged_list(entity_list, record_count, page);
        tracing::debug!(target: "taitan_orm", command = "search_paged", result = ?result);
        Ok(result)
    }

    async fn exists<M: SqliteMutation>(
        &self,
        unique: &dyn SqliteUnique<Mutation = M>,
    ) -> Result<bool> {
        tracing::debug!(target: "taitan_orm", command = "exists", unique = ?unique);
        let sql = SqliteSqlGenerator::gen_unique_count_sql(unique);
        tracing::debug!(target: "taitan_orm", command = "exists", sql = sql);
        let args = unique.gen_args()?;
        let result: bool = <Self as SqlExecutorNew<Sqlite>>::fetch_exists(self, &sql, args).await?;
        tracing::debug!(target: "taitan_orm", command = "exists", result = ?result);
        Ok(result)
    }

    async fn count(&self, location: &dyn SqliteLocation) -> Result<u64> {
        tracing::debug!(target: "taitan_orm", command = "count", location = ?location);
        let args = location.gen_args()?;
        let count_sql = SqliteSqlGenerator::gen_location_count_sql(location);
        tracing::debug!(target: "taitan_orm", command = "count", sql = count_sql);
        let record_count: u64 =
            <Self as SqlExecutorNew<Sqlite>>::fetch_count(self, &count_sql, args).await?;
        tracing::debug!(target: "taitan_orm", command = "count", result = ?record_count);
        Ok(record_count)
    }

    // async fn count_all(&self, table_name: &str) -> Result<u64> {
    //     tracing::debug!(target: "taitan_orm", command = "count", table_name = ?table_name);
    //     let count_sql = self.get_generator().get_count_table_sql(table_name);
    //     tracing::debug!(target: "taitan_orm", command = "count", sql = count_sql);
    //     let record_count: u64 = self.fetch_count_plain(&count_sql).await?;
    //     tracing::debug!(target: "taitan_orm", command = "count", result = ?record_count);
    //     Ok(record_count)
    // }

    async fn select_full<SE, M>(
        &self,
        unique: &dyn SqliteUnique<Mutation = M>,
    ) -> Result<Option<SE>>
    where
        M: SqliteMutation,
        SE: SqliteSelected + Send + Unpin,
    {
        tracing::debug!(target: "taitan_orm", command = "select_full", primary = ?unique);
        let selection = SE::default();
        let sql = SqliteSqlGenerator::gen_select_sql(&selection, unique);
        tracing::debug!(target: "taitan_orm", command = "select_full", sql = sql);
        let args = unique.gen_args()?;
        let result: Option<SE> =
            <Self as SqlExecutorNew<Sqlite>>::fetch_option_(self, &sql, &selection, args).await?;
        tracing::debug!(target: "taitan_orm", command = "select_full", result = ?result);
        Ok(result)
    }

    async fn search_full<SE>(
        &self,
        location: &dyn SqliteLocation,
        order_by: &dyn OrderBy,
        page: &Pagination,
    ) -> Result<Vec<SE>>
    where
        SE: SqliteSelected + Send + Unpin,
    {
        tracing::debug!(target: "taitan_orm", command = "search_full", location = ?location, order_by = ?order_by);
        let selection = SE::default();
        let sql = SqliteSqlGenerator::gen_search_sql(&selection, location, order_by, page);
        tracing::debug!(target: "taitan_orm", command = "search_full", sql = sql);
        let mut args = location.gen_args()?;
        <Pagination as Parameter<Sqlite>>::add_to_args(page, &mut args)?;
        let result: Vec<SE> =
            <Self as SqlExecutorNew<Sqlite>>::fetch_all_(self, &sql, &selection, args).await?;
        tracing::debug!(target: "taitan_orm", command = "search_full", result = ?result);
        Ok(result)
    }

    async fn search_full_all<SE>(
        &self,
        location: &dyn SqliteLocation,
        order_by: &dyn OrderBy,
    ) -> Result<Vec<SE>>
    where
        SE: SqliteSelected + Send + Unpin,
    {
        tracing::debug!(target: "taitan_orm", command = "search_full_all", location = ?location, order_by = ?order_by);
        let selection = SE::default();
        let sql = SqliteSqlGenerator::gen_search_all_sql(&selection, location, order_by);
        tracing::debug!(target: "taitan_orm", command = "search_full_all", sql = sql);
        let args = location.gen_args()?;
        let result: Vec<SE> =
            <Self as SqlExecutorNew<Sqlite>>::fetch_all_(self, &sql, &selection, args).await?;
        tracing::debug!(target: "taitan_orm", command = "search_full_all", result = ?result);
        Ok(result)
    }

    async fn search_full_paged<SE>(
        &self,
        location: &dyn SqliteLocation,
        order_by: &dyn OrderBy,
        page: &Pagination,
    ) -> Result<PagedList<Sqlite, SE>>
    where
        SE: SqliteSelected + Send + Unpin,
    {
        tracing::debug!(target: "taitan_orm", command = "search_full_paged", location = ?location, order_by = ?order_by, page = ?page);
        let selection = SE::default();
        let record_count = self.count(location).await?;
        if record_count <= 0 {
            return Ok(PagedList::empty(page.page_size, page.page_num));
        }

        let sql = SqliteSqlGenerator::gen_search_sql(&selection, location, order_by, page);
        tracing::debug!(target: "taitan_orm", command = "search_full_paged", sql = sql);
        let mut args = location.gen_args()?;
        <Pagination as Parameter<Sqlite>>::add_to_args(page, &mut args)?;
        let entity_list: Vec<SE> =
            <Self as SqlExecutorNew<Sqlite>>::fetch_all_(self, &sql, &selection, args).await?;
        let result = build_paged_list(entity_list, record_count, page);
        tracing::debug!(target: "taitan_orm", command = "search_full_paged", result = ?result);
        Ok(result)
    }
}
