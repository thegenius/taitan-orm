#[doc(hidden)]
#[macro_export]
macro_rules! reader_mut_impl {
    ($db: ty, $gen: ident, $se: path, $mutation: path, $location: path, $unique: ident) => {
        async fn select<SE, M>(
            &mut self,
            selection: &SE,
            unique: &dyn $unique<Mutation = M>,
        ) -> Result<Option<SE>>
        where
            M: $mutation,
            SE: $se + Send + Unpin,
        {
            debug!(target: "taitan_orm", command = "select", primary = ?unique, selection = ?selection);
            let sql = $gen::gen_select_sql(selection, unique);
            debug!(target: "taitan_orm", command = "select", sql = sql);
            let args = unique.gen_args()?;
            let result: Option<SE> =
                self.fetch_option_(&sql, selection, args).await?;
            debug!(target: "taitan_orm", command = "select", result = ?result);
            Ok(result)
        }

        async fn search<SE>(
            &mut self,
            selection: &SE,
            location: &dyn $location,
            order_by: &dyn OrderBy,
            page: &Pagination,
        ) -> Result<Vec<SE>>
        where
            SE: $se + Send + Unpin,
        {
            debug!(target: "taitan_orm", command = "search", location = ?location, order_by = ?order_by, selection = ?selection);
            let sql = $gen::gen_search_sql(selection, location, order_by, page);
            debug!(target: "taitan_orm", command = "search", sql = sql);
            let mut args = location.gen_args()?;
            <Pagination as Parameter<$db>>::add_to_args(page, &mut args)?;
            let result: Vec<SE> =
                self.fetch_all_(&sql, selection, args).await?;
            debug!(target: "taitan_orm", command = "search", result = ?result);
            Ok(result)
        }

        async fn search_all<SE>(
            &mut self,
            selection: &SE,
            location: &dyn $location,
            order_by: &dyn OrderBy,
        ) -> Result<Vec<SE>>
        where
            SE: $se + Send + Unpin,
        {
            debug!(target: "taitan_orm", command = "search", location = ?location, order_by = ?order_by, selection = ?selection);
            let sql = $gen::gen_search_all_sql(selection, location, order_by);
            debug!(target: "taitan_orm", command = "search", sql = sql);
            let args = location.gen_args()?;
            let result: Vec<SE> =
                self.fetch_all_(&sql, selection, args).await?;
            debug!(target: "taitan_orm", command = "search", result = ?result);
            Ok(result)
        }

        async fn search_paged<SE>(
            &mut self,
            selection: &SE,
            location: &dyn $location,
            order_by: &dyn OrderBy,
            page: &Pagination,
        ) -> Result<PagedList<$db, SE>>
        where
            SE: $se + Send + Unpin,
        {
            debug!(target: "taitan_orm", command = "search_paged", location = ?location, order_by = ?order_by, selection = ?selection, page = ?page);
            let record_count = self.count(location).await?;
            if record_count <= 0 {
                return Ok(PagedList::empty(page.page_size, page.page_num));
            }

            let sql = $gen::gen_search_sql(selection, location, order_by, page);
            debug!(target: "taitan_orm", command = "search_paged", sql = sql);
            let mut args = location.gen_args()?;
            <Pagination as Parameter<$db>>::add_to_args(page, &mut args)?;
            let entity_list: Vec<SE> =
                self.fetch_all_(&sql, selection, args).await?;
            let result = build_paged_list(entity_list, record_count, page);
            debug!(target: "taitan_orm", command = "search_paged", result = ?result);
            Ok(result)
        }

        async fn exists<M: $mutation>(
            &mut self,
            unique: &dyn $unique<Mutation = M>,
        ) -> Result<bool> {
            debug!(target: "taitan_orm", command = "exists", unique = ?unique);
            let sql = $gen::gen_unique_count_sql(unique);
            debug!(target: "taitan_orm", command = "exists", sql = sql);
            let args = unique.gen_args()?;
            let result: bool = self.fetch_exists(&sql, args).await?;
            debug!(target: "taitan_orm", command = "exists", result = ?result);
            Ok(result)
        }

        async fn count(&mut self, location: &dyn $location) -> Result<u64> {
            debug!(target: "taitan_orm", command = "count", location = ?location);
            let args = location.gen_args()?;
            let count_sql = $gen::gen_location_count_sql(location);
            debug!(target: "taitan_orm", command = "count", sql = count_sql);
            let record_count: u64 =
                self.fetch_count(&count_sql, args).await?;
            debug!(target: "taitan_orm", command = "count", result = ?record_count);
            Ok(record_count)
        }

        // async fn count_all(&mut self, table_name: &str) -> Result<u64> {
        //     debug!(target: "taitan_orm", command = "count", table_name = ?table_name);
        //     let count_sql = self.get_generator().get_count_table_sql(table_name);
        //     debug!(target: "taitan_orm", command = "count", sql = count_sql);
        //     let record_count: u64 = self.fetch_count_plain(&count_sql).await?;
        //     debug!(target: "taitan_orm", command = "count", result = ?record_count);
        //     Ok(record_count)
        // }

        async fn select_full<SE, M>(
            &mut self,
            unique: &dyn $unique<Mutation = M>,
        ) -> Result<Option<SE>>
        where
            M: $mutation,
            SE: $se + Send + Unpin,
        {
            debug!(target: "taitan_orm", command = "select_full", primary = ?unique);
            let selection = SE::default();
            let sql = $gen::gen_select_sql(&selection, unique);
            debug!(target: "taitan_orm", command = "select_full", sql = sql);
            let args = unique.gen_args()?;
            let result: Option<SE> =
                self.fetch_option_(&sql, &selection, args).await?;
            debug!(target: "taitan_orm", command = "select_full", result = ?result);
            Ok(result)
        }

        async fn search_full<SE>(
            &mut self,
            location: &dyn $location,
            order_by: &dyn OrderBy,
            page: &Pagination,
        ) -> Result<Vec<SE>>
        where
            SE: $se + Send + Unpin,
        {
            debug!(target: "taitan_orm", command = "search_full", location = ?location, order_by = ?order_by);
            let selection = SE::default();
            let sql = $gen::gen_search_sql(&selection, location, order_by, page);
            debug!(target: "taitan_orm", command = "search_full", sql = sql);
            let mut args = location.gen_args()?;
            <Pagination as Parameter<$db>>::add_to_args(page, &mut args)?;
            let result: Vec<SE> =
                self.fetch_all_(&sql, &selection, args).await?;
            debug!(target: "taitan_orm", command = "search_full", result = ?result);
            Ok(result)
        }

        async fn search_full_all<SE>(
            &mut self,
            location: &dyn $location,
            order_by: &dyn OrderBy,
        ) -> Result<Vec<SE>>
        where
            SE: $se + Send + Unpin,
        {
            debug!(target: "taitan_orm", command = "search_full_all", location = ?location, order_by = ?order_by);
            let selection = SE::default();
            let sql = $gen::gen_search_all_sql(&selection, location, order_by);
            debug!(target: "taitan_orm", command = "search_full_all", sql = sql);
            let args = location.gen_args()?;
            let result: Vec<SE> =
                self.fetch_all_(&sql, &selection, args).await?;
            debug!(target: "taitan_orm", command = "search_full_all", result = ?result);
            Ok(result)
        }

        async fn search_full_paged<SE>(
            &mut self,
            location: &dyn $location,
            order_by: &dyn OrderBy,
            page: &Pagination,
        ) -> Result<PagedList<$db, SE>>
        where
            SE: $se + Send + Unpin,
        {
            debug!(target: "taitan_orm", command = "search_full_paged", location = ?location, order_by = ?order_by, page = ?page);
            let selection = SE::default();
            let record_count = self.count(location).await?;
            if record_count <= 0 {
                return Ok(PagedList::empty(page.page_size, page.page_num));
            }

            let sql = $gen::gen_search_sql(&selection, location, order_by, page);
            debug!(target: "taitan_orm", command = "search_full_paged", sql = sql);
            let mut args = location.gen_args()?;
            <Pagination as Parameter<$db>>::add_to_args(page, &mut args)?;
            let entity_list: Vec<SE> =
                self.fetch_all_(&sql, &selection, args).await?;
            let result = build_paged_list(entity_list, record_count, page);
            debug!(target: "taitan_orm", command = "search_full_paged", result = ?result);
            Ok(result)
        }
    }
}