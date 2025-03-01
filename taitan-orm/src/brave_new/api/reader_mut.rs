use crate::brave_new::SqlGenerator;
use crate::brave_new::ArgsExtractor;
use crate::brave_new::{SqlExecutorMut};
use taitan_orm_trait::brave_new::result::Result;
use taitan_orm_trait::brave_new::{build_paged_list, PagedList, Pagination, Selected};
use taitan_orm_trait::brave_new::{Location, Mutation, OrderBy, Unique};

impl<T> ReaderMutApi for T where T: SqlExecutorMut + SqlGenerator + ArgsExtractor {}

pub trait ReaderMutApi: SqlExecutorMut + SqlGenerator + ArgsExtractor {
    async fn select<SE, M>(
        &mut self,
        selection: &SE,
        unique: &dyn Unique<Self::DB, Mutation = M>,
    ) -> Result<Option<SE>>
    where
        M: Mutation<Self::DB>,
        SE: Selected<Self::DB> + Send + Unpin,
    {
        tracing::debug!(target: "taitan_orm", command = "select", primary = ?unique, selection = ?selection);
        let sql = self.gen_select_sql(selection, unique);
        tracing::debug!(target: "taitan_orm", command = "select", sql = sql);
        let args = Self::extract_unique_arguments(unique)?;
        let result: Option<SE> = self.fetch_option_(&sql, selection, args).await?;
        tracing::debug!(target: "taitan_orm", command = "select", result = ?result);
        Ok(result)
    }

    async fn search<SE, M>(
        &mut self,
        selection: &SE,
        location: &dyn Location<Self::DB>,
        order_by: &dyn OrderBy,
        page: &Pagination,
    ) -> Result<Vec<SE>>
    where
        SE: Selected<Self::DB> + Send + Unpin,
    {
        tracing::debug!(target: "taitan_orm", command = "search", location = ?location, order_by = ?order_by, selection = ?selection);
        let sql = self.gen_search_sql(selection, location, order_by, page);
        tracing::debug!(target: "taitan_orm", command = "search", sql = sql);
        let args = Self::extract_location_arguments(location)?;
        let result: Vec<SE> = self.fetch_all_(&sql, selection, args).await?;
        tracing::debug!(target: "taitan_orm", command = "search", result = ?result);
        Ok(result)
    }

    async fn search_all<SE, M>(
        &mut self,
        selection: &SE,
        location: &dyn Location<Self::DB>,
        order_by: &dyn OrderBy,
    ) -> Result<Vec<SE>>
    where
        SE: Selected<Self::DB> + Send + Unpin,
    {
        tracing::debug!(target: "taitan_orm", command = "search", location = ?location, order_by = ?order_by, selection = ?selection);
        let sql = self.gen_search_all_sql(selection, location, order_by);
        tracing::debug!(target: "taitan_orm", command = "search", sql = sql);
        let args = Self::extract_location_arguments(location)?;
        let result: Vec<SE> = self.fetch_all_(&sql, selection, args).await?;
        tracing::debug!(target: "taitan_orm", command = "search", result = ?result);
        Ok(result)
    }

    async fn search_paged<SE>(
        &mut self,
        selection: &SE,
        location: &dyn Location<Self::DB>,
        order_by: &dyn OrderBy,
        page: &Pagination,
    ) -> Result<PagedList<Self::DB, SE>>
    where
        SE: Selected<Self::DB> + Send + Unpin,
    {
        tracing::debug!(target: "taitan_orm", command = "search_paged", location = ?location, order_by = ?order_by, selection = ?selection, page = ?page);
        let record_count = self.count(location).await?;
        if record_count <= 0 {
            return Ok(PagedList::empty(page.page_size, page.page_num));
        }

        let sql = self.gen_search_sql(selection, location, order_by, page);
        tracing::debug!(target: "taitan_orm", command = "search_paged", sql = sql);
        let args = Self::extract_location_arguments(location)?;
        let entity_list: Vec<SE> = self.fetch_all_(&sql, selection, args).await?;
        let result = build_paged_list(entity_list, record_count, page);
        tracing::debug!(target: "taitan_orm", command = "search_paged", result = ?result);
        Ok(result)
    }

    async fn exists<M: Mutation<Self::DB>>(
        &mut self,
        unique: &dyn Unique<Self::DB, Mutation = M>,
    ) -> Result<bool> {
        tracing::debug!(target: "taitan_orm", command = "exists", unique = ?unique);
        let sql = self.gen_unique_count_sql(unique);
        tracing::debug!(target: "taitan_orm", command = "exists", sql = sql);
        let args = Self::extract_unique_arguments(unique)?;
        let result: bool = self.fetch_exists(&sql, args).await?;
        tracing::debug!(target: "taitan_orm", command = "exists", result = ?result);
        Ok(result)
    }

    async fn count(&mut self, location: &dyn Location<Self::DB>) -> Result<u64> {
        tracing::debug!(target: "taitan_orm", command = "count", location = ?location);
        let args = Self::extract_location_arguments(location)?;
        let count_sql = self.gen_location_count_sql(location);
        tracing::debug!(target: "taitan_orm", command = "count", sql = count_sql);
        let record_count: u64 = self.fetch_count(&count_sql, args).await?;
        tracing::debug!(target: "taitan_orm", command = "count", result = ?record_count);
        Ok(record_count)
    }

    // async fn count_all(&mut self, table_name: &str) -> Result<u64> {
    //     tracing::debug!(target: "taitan_orm", command = "count", table_name = ?table_name);
    //     let count_sql = self.get_generator().get_count_table_sql(table_name);
    //     tracing::debug!(target: "taitan_orm", command = "count", sql = count_sql);
    //     let record_count: u64 = self.fetch_count_plain(&count_sql).await?;
    //     tracing::debug!(target: "taitan_orm", command = "count", result = ?record_count);
    //     Ok(record_count)
    // }

    async fn select_full<SE, M>(
        &mut self,
        unique: &dyn Unique<Self::DB, Mutation = M>,
    ) -> Result<Option<SE>>
    where
        M: Mutation<Self::DB>,
        SE: Selected<Self::DB> + Send + Unpin,
    {
        tracing::debug!(target: "taitan_orm", command = "select_full", primary = ?unique);
        let selection = SE::default();
        let sql = self.gen_select_sql(&selection, unique);
        tracing::debug!(target: "taitan_orm", command = "select_full", sql = sql);
        let args = Self::extract_unique_arguments(unique)?;
        let result: Option<SE> = self.fetch_option_(&sql, &selection, args).await?;
        tracing::debug!(target: "taitan_orm", command = "select_full", result = ?result);
        Ok(result)
    }

    async fn search_full<SE, M>(
        &mut self,
        location: &dyn Location<Self::DB>,
        order_by: &dyn OrderBy,
        page: &Pagination,
    ) -> Result<Vec<SE>>
    where
        SE: Selected<Self::DB> + Send + Unpin,
    {
        tracing::debug!(target: "taitan_orm", command = "search_full", location = ?location, order_by = ?order_by);
        let selection = SE::default();
        let sql = self.gen_search_sql(&selection, location, order_by, page);
        tracing::debug!(target: "taitan_orm", command = "search_full", sql = sql);
        let args = Self::extract_location_arguments(location)?;
        let result: Vec<SE> = self.fetch_all_(&sql, &selection, args).await?;
        tracing::debug!(target: "taitan_orm", command = "search_full", result = ?result);
        Ok(result)
    }

    async fn search_full_all<SE, M>(
        &mut self,
        location: &dyn Location<Self::DB>,
        order_by: &dyn OrderBy,
    ) -> Result<Vec<SE>>
    where
        SE: Selected<Self::DB> + Send + Unpin,
    {
        tracing::debug!(target: "taitan_orm", command = "search_full_all", location = ?location, order_by = ?order_by);
        let selection = SE::default();
        let sql = self.gen_search_all_sql(&selection, location, order_by);
        tracing::debug!(target: "taitan_orm", command = "search_full_all", sql = sql);
        let args = Self::extract_location_arguments(location)?;
        let result: Vec<SE> = self.fetch_all_(&sql, &selection, args).await?;
        tracing::debug!(target: "taitan_orm", command = "search_full_all", result = ?result);
        Ok(result)
    }

    async fn search_full_paged<SE>(
        &mut self,
        location: &dyn Location<Self::DB>,
        order_by: &dyn OrderBy,
        page: &Pagination,
    ) -> Result<PagedList<Self::DB, SE>>
    where
        SE: Selected<Self::DB> + Send + Unpin,
    {
        tracing::debug!(target: "taitan_orm", command = "search_full_paged", location = ?location, order_by = ?order_by, page = ?page);
        let selection = SE::default();
        let record_count = self.count(location).await?;
        if record_count <= 0 {
            return Ok(PagedList::empty(page.page_size, page.page_num));
        }

        let sql = self.gen_search_sql(&selection, location, order_by, page);
        tracing::debug!(target: "taitan_orm", command = "search_full_paged", sql = sql);
        let args = Self::extract_location_arguments(location)?;
        let entity_list: Vec<SE> = self.fetch_all_(&sql, &selection, args).await?;
        let result = build_paged_list(entity_list, record_count, page);
        tracing::debug!(target: "taitan_orm", command = "search_full_paged", result = ?result);
        Ok(result)
    }
}
