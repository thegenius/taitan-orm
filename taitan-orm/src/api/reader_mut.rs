use crate::extractor::Extractor;
use crate::Result;
use crate::{SqlExecutorMut, SqlGenerator,  SqlGeneratorContainer};
use taitan_orm_trait::{Location, Mutation, OrderBy, SelectedEntity, Selection, Unique};

impl<T> ReaderMutApi for T where T: SqlExecutorMut + SqlGeneratorContainer + Extractor {}

pub trait ReaderMutApi: SqlExecutorMut + SqlGeneratorContainer + Extractor {
    async fn exists<M: Mutation>(&mut self, unique: &dyn Unique<Mutation = M>) -> Result<bool> {
        tracing::debug!(target: "taitan_orm", command = "exists", unique = ?unique);
        let sql = self.get_generator().get_exists_sql(unique);
        tracing::debug!(target: "taitan_orm", command = "exists", sql = sql);
        let args = Self::extract_unique_arguments(unique)?;
        let result: bool = self.fetch_exists(&sql, args).await?;
        tracing::debug!(target: "taitan_orm", command = "exists", result = ?result);
        Ok(result)
    }

    async fn count(&mut self, location: &dyn Location) -> Result<u64> {
        tracing::debug!(target: "taitan_orm", command = "count", location = ?location);
        let args = Self::extract_location_arguments(location)?;
        let count_sql = self.get_generator().get_count_sql(location);
        tracing::debug!(target: "taitan_orm", command = "count", sql = count_sql);
        let record_count: u64 = self.fetch_count(&count_sql, args).await?;
        tracing::debug!(target: "taitan_orm", command = "count", result = ?record_count);
        Ok(record_count)
    }

    async fn count_all(&mut self, table_name: &str) -> Result<u64> {
        tracing::debug!(target: "taitan_orm", command = "count", table_name = ?table_name);
        let count_sql = self.get_generator().get_count_table_sql(table_name);
        tracing::debug!(target: "taitan_orm", command = "count", sql = count_sql);
        let record_count: u64 = self.fetch_count_plain(&count_sql).await?;
        tracing::debug!(target: "taitan_orm", command = "count", result = ?record_count);
        Ok(record_count)
    }

    async fn __select<SE, M>(
        &mut self,
        selection: &SE::Selection,
        unique: &dyn Unique<Mutation = M>,
    ) -> Result<Option<SE>>
    where
        M: Mutation,
        SE: SelectedEntity<Self::DB> + Send + Unpin,
    {
        tracing::debug!(target: "taitan_orm", command = "select", primary = ?unique, selection = ?selection);
        let sql = self.get_generator().get_select_sql(selection, unique);
        tracing::debug!(target: "taitan_orm", command = "select", sql = sql);
        let args = Self::extract_unique_arguments(unique)?;
        let result: Option<SE> = self.fetch_option(&sql, selection, args).await?;
        tracing::debug!(target: "taitan_orm", command = "select", result = ?result);
        Ok(result)
    }

    async fn select<SE, M>(
        &mut self,
        selection: &SE,
        unique: &dyn Unique<Mutation = M>,
    ) -> Result<Option<SE>>
    where
        M: Mutation,
        SE: SelectedEntity<Self::DB> + Send + Unpin + Selection,
    {
        tracing::debug!(target: "taitan_orm", command = "select", primary = ?unique, selection = ?selection);
        let sql = self.get_generator().get_select_sql(selection, unique);
        tracing::debug!(target: "taitan_orm", command = "select", sql = sql);
        let args = Self::extract_unique_arguments(unique)?;
        let result: Option<SE> = self.fetch_option_(&sql, selection, args).await?;
        tracing::debug!(target: "taitan_orm", command = "select", result = ?result);
        Ok(result)
    }

    async fn __search<SE>(
        &mut self,
        selection: &SE::Selection,
        location: &dyn Location,
        order_by: &Option<&dyn OrderBy>,
        page: &Option<&crate::page::Pagination>,
    ) -> Result<Vec<SE>>
    where
        SE: SelectedEntity<Self::DB> + Send + Unpin,
    {
        tracing::debug!(target: "taitan_orm", command = "search", location = ?location, order_by = ?order_by, selection = ?selection);
        let sql =
            self.get_generator()
                .get_search_paged_sql(selection, &Some(location), order_by, page);
        tracing::debug!(target: "taitan_orm", command = "search", sql = sql);
        let args = Self::extract_location_arguments(location)?;
        let result: Vec<SE> = self.fetch_all(&sql, selection, args).await?;
        tracing::debug!(target: "taitan_orm", command = "search", result = ?result);
        Ok(result)
    }

    async fn search<SE>(
        &mut self,
        selection: &SE,
        location: &dyn Location,
        order_by: &Option<&dyn OrderBy>,
        page: &Option<&crate::page::Pagination>,
    ) -> Result<Vec<SE>>
    where
        SE: SelectedEntity<Self::DB> + Send + Unpin  + Selection,
    {
        tracing::debug!(target: "taitan_orm", command = "search", location = ?location, order_by = ?order_by, selection = ?selection);
        let sql =
            self.get_generator()
                .get_search_paged_sql(selection, &Some(location), order_by, page);
        tracing::debug!(target: "taitan_orm", command = "search", sql = sql);
        let args = Self::extract_location_arguments(location)?;
        let result: Vec<SE> = self.fetch_all_(&sql, selection, args).await?;
        tracing::debug!(target: "taitan_orm", command = "search", result = ?result);
        Ok(result)
    }

    async fn __search_paged<SE>(
        &mut self,
        selection: &SE::Selection,
        location: &dyn Location,
        order_by: &dyn OrderBy,
        page: &crate::page::Pagination,
    ) -> Result<crate::page::PagedList<Self::DB, SE>>
    where
        SE: SelectedEntity<Self::DB> + Send + Unpin,
    {
        tracing::debug!(target: "taitan_orm", command = "search_paged", location = ?location, order_by = ?order_by, selection = ?selection, page = ?page);
        let record_count = self.count(location).await?;
        if record_count <= 0 {
            return Ok(crate::page::PagedList::empty(page.page_size, page.page_num));
        }

        let sql = self.get_generator().get_search_paged_sql(
            selection,
            &Some(location),
            &Some(order_by),
            &Some(&page),
        );
        tracing::debug!(target: "taitan_orm", command = "search_paged", sql = sql);
        let args = Self::extract_location_arguments(location)?;
        let entity_list: Vec<SE> = self.fetch_all(&sql, selection, args).await?;
        let result = taitan_orm_trait::build_paged_list(entity_list, record_count, page);
        tracing::debug!(target: "taitan_orm", command = "search_paged", result = ?result);
        Ok(result)
    }

    async fn search_paged<SE>(
        &mut self,
        selection: &SE,
        location: &dyn Location,
        order_by: &dyn OrderBy,
        page: &crate::page::Pagination,
    ) -> Result<crate::page::PagedList<Self::DB, SE>>
    where
        SE: SelectedEntity<Self::DB> + Send + Unpin + Selection,
    {
        tracing::debug!(target: "taitan_orm", command = "search_paged", location = ?location, order_by = ?order_by, selection = ?selection, page = ?page);
        let record_count = self.count(location).await?;
        if record_count <= 0 {
            return Ok(crate::page::PagedList::empty(page.page_size, page.page_num));
        }

        let sql = self.get_generator().get_search_paged_sql(
            selection,
            &Some(location),
            &Some(order_by),
            &Some(&page),
        );
        tracing::debug!(target: "taitan_orm", command = "search_paged", sql = sql);
        let args = Self::extract_location_arguments(location)?;
        let entity_list: Vec<SE> = self.fetch_all_(&sql, selection, args).await?;
        let result = taitan_orm_trait::build_paged_list(entity_list, record_count, page);
        tracing::debug!(target: "taitan_orm", command = "search_paged", result = ?result);
        Ok(result)
    }

    async fn __devour<SE>(
        &mut self,
        selection: &SE::Selection,
        order_by: &Option<&dyn OrderBy>,
        page: &Option<&crate::page::Pagination>,
    ) -> Result<Vec<SE>>
    where
        SE: SelectedEntity<Self::DB> + Send + Unpin,
    {
        tracing::debug!(target: "taitan_orm", command = "devour", selection = ?selection);
        let sql = self
            .get_generator()
            .get_search_paged_sql(selection, &None, order_by, page);
        tracing::debug!(target: "taitan_orm", command = "devour", sql = sql);
        match page {
            None => {
                let result: Vec<SE> = self.fetch_all_plain(&sql, selection).await?;
                tracing::debug!(target: "taitan_orm", command = "devour", result = ?result);
                Ok(result)
            }
            Some(page) => {
                let args = Self::extract_pagination_arguments(page)?;
                let result: Vec<SE> = self.fetch_all(&sql, selection, args).await?;
                tracing::debug!(target: "taitan_orm", command = "devour", result = ?result);
                Ok(result)
            }
        }
    }

    async fn devour<SE>(
        &mut self,
        selection: &SE,
        order_by: &Option<&dyn OrderBy>,
        page: &Option<&crate::page::Pagination>,
    ) -> Result<Vec<SE>>
    where
        SE: SelectedEntity<Self::DB> + Send + Unpin + Selection,
    {
        tracing::debug!(target: "taitan_orm", command = "devour", selection = ?selection);
        let sql = self
            .get_generator()
            .get_search_paged_sql(selection, &None, order_by, page);
        tracing::debug!(target: "taitan_orm", command = "devour", sql = sql);
        match page {
            None => {
                let result: Vec<SE> = self.fetch_all_plain_(&sql, selection).await?;
                tracing::debug!(target: "taitan_orm", command = "devour", result = ?result);
                Ok(result)
            }
            Some(page) => {
                let args = Self::extract_pagination_arguments(page)?;
                let result: Vec<SE> = self.fetch_all_(&sql, selection, args).await?;
                tracing::debug!(target: "taitan_orm", command = "devour", result = ?result);
                Ok(result)
            }
        }
    }

    async fn __devour_paged<SE>(
        &mut self,
        selection: &SE::Selection,
        order_by: &dyn OrderBy,
        page: &crate::page::Pagination,
    ) -> Result<crate::page::PagedList<Self::DB, SE>>
    where
        SE: SelectedEntity<Self::DB> + Send + Unpin,
    {
        tracing::debug!(target: "taitan_orm", command = "devour_paged", order_by = ?order_by, selection = ?selection, page = ?page);
        let record_count = self.count_all(selection.get_table_name()).await?;
        if record_count <= 0 {
            return Ok(crate::page::PagedList::empty(page.page_size, page.page_num));
        }

        tracing::debug!(target: "taitan_orm", command = "devour_paged", selection = ?selection);
        let sql = self.get_generator().get_search_paged_sql(
            selection,
            &None,
            &Some(order_by),
            &Some(page),
        );
        tracing::debug!(target: "taitan_orm", command = "devour_paged", sql = sql);
        let args = Self::extract_pagination_arguments(page)?;
        let entity_list: Vec<SE> = self.fetch_all(&sql, selection, args).await?;
        let result = taitan_orm_trait::build_paged_list(entity_list, record_count, page);
        tracing::debug!(target: "taitan_orm", command = "devour_paged", result = ?result);
        Ok(result)
    }

    async fn devour_paged<SE>(
        &mut self,
        selection: &SE,
        order_by: &dyn OrderBy,
        page: &crate::page::Pagination,
    ) -> Result<crate::page::PagedList<Self::DB, SE>>
    where
        SE: SelectedEntity<Self::DB> + Send + Unpin + Selection,
    {
        tracing::debug!(target: "taitan_orm", command = "devour_paged", order_by = ?order_by, selection = ?selection, page = ?page);
        let record_count = self.count_all(selection.get_table_name()).await?;
        if record_count <= 0 {
            return Ok(crate::page::PagedList::empty(page.page_size, page.page_num));
        }

        tracing::debug!(target: "taitan_orm", command = "devour_paged", selection = ?selection);
        let sql = self.get_generator().get_search_paged_sql(
            selection,
            &None,
            &Some(order_by),
            &Some(page),
        );
        tracing::debug!(target: "taitan_orm", command = "devour_paged", sql = sql);
        let args = Self::extract_pagination_arguments(page)?;
        let entity_list: Vec<SE> = self.fetch_all_(&sql, selection, args).await?;
        let result = taitan_orm_trait::build_paged_list(entity_list, record_count, page);
        tracing::debug!(target: "taitan_orm", command = "devour_paged", result = ?result);
        Ok(result)
    }
}
