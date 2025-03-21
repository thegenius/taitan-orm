

// impl SqlGenericExecutor for SqliteDatabase {
//     type DB = Sqlite;
//
//     fn get_affected_rows(query_result: &<Self::DB as sqlx::Database>::QueryResult) -> u64 {
//         query_result.rows_affected()
//     }
// }
// impl crate::SqlExecutor for SqliteDatabase {
//     executor_impl!(SqliteConnection);
// }

//
// impl crate::SqlExecutor for SqliteDatabase {
//     type Connection = SqliteConnection;
//
//     #[inline(always)]
//     async fn get_connection(&mut self) -> crate::Result<sqlx::pool::PoolConnection<Self::DB>> {
//         Ok(self.get_pool()?.acquire().await?)
//     }
//
//     async fn execute<'a>(&'a mut self, stmt: &'a str, args: <Self::DB as sqlx::Database>::Arguments<'a>) -> crate::Result<u64>
//     {
//         let mut ex = self.get_connection().await?;
//         Self::generic_execute(&mut *ex, stmt, args).await
//     }
//
//     async fn execute_plain<'a>(&'a mut self, stmt: &'a str) -> crate::Result<u64> {
//         let args: std::marker::PhantomData<<Self::DB as sqlx::Database>::Arguments<'a>> =
//             std::marker::PhantomData::default();
//         let mut ex = self.get_pool()?.acquire().await?;
//         Self::generic_execute_plain(&mut *ex, stmt, args).await
//     }
//
//     async fn fetch_exists<'a>(
//         &'a mut self,
//         stmt: &'a str,
//         args: <Self::DB as sqlx::Database>::Arguments<'a>,
//     ) -> crate::Result<bool> {
//         let mut ex = self.get_pool()?.acquire().await?;
//         Self::generic_exists(&mut *ex, stmt, args).await
//     }
//
//     async fn fetch_exists_plain<'a, A>(&'a mut self, stmt: &'a str) -> crate::Result<bool>
//     {
//         let mut ex = self.get_pool()?.acquire().await?;
//         let args: std::marker::PhantomData<<Self::DB as sqlx::Database>::Arguments<'a>> =
//             std::marker::PhantomData::default();
//         Self::generic_exists_plain(&mut *ex, stmt, args).await
//     }
//
//     async fn fetch_option<'a, SE>(
//         &'a mut self,
//         stmt: &'a str,
//         selection: &'a SE::Selection,
//         args: <Self::DB as sqlx::Database>::Arguments<'a>,
//     ) -> crate::Result<Option<SE>>
//     where
//         SE: taitan_orm_trait::SelectedEntity<Self::DB> + Send + Unpin,
//     {
//         let mut ex = self.get_pool()?.acquire().await?;
//         Self::generic_fetch_option(&mut *ex, stmt, selection, args).await
//     }
//
//     async fn fetch_option_plain<'a, SE>(
//         &'a mut self,
//         stmt: &'a str,
//         selection: &'a SE::Selection,
//     ) -> crate::Result<Option<SE>>
//     where
//         SE: taitan_orm_trait::SelectedEntity<Self::DB> + Send + Unpin,
//     {
//         let mut ex = self.get_pool()?.acquire().await?;
//         let args: std::marker::PhantomData<<Self::DB as sqlx::Database>::Arguments<'a>> =
//             std::marker::PhantomData::default();
//         Self::generic_fetch_option_plain(&mut *ex, stmt, selection, args).await
//     }
//
//     async fn fetch_all<'a, SE>(
//         &'a mut self,
//         stmt: &'a str,
//         selection: &'a SE::Selection,
//         args: <Self::DB as sqlx::Database>::Arguments<'a>,
//     ) -> crate::Result<Vec<SE>>
//     where
//         SE: taitan_orm_trait::SelectedEntity<Self::DB> + Send + Unpin,
//     {
//         let mut ex = self.get_pool()?.acquire().await?;
//         Self::generic_fetch_all(&mut *ex, stmt, selection, args).await
//     }
//
//     async fn fetch_all_plain<'a, SE>(
//         &'a mut self,
//         stmt: &'a str,
//         selection: &'a SE::Selection,
//     ) -> crate::Result<Vec<SE>>
//     where
//         SE: taitan_orm_trait::SelectedEntity<Self::DB> + Send + Unpin,
//     {
//         let mut ex = self.get_pool()?.acquire().await?;
//         let args: std::marker::PhantomData<<Self::DB as sqlx::Database>::Arguments<'a>> =
//             std::marker::PhantomData::default();
//         Self::generic_fetch_all_plain(&mut *ex, stmt, selection, args).await
//     }
//
//     async fn fetch_one_full<'a, SE>(
//         &'a mut self,
//         stmt: &'a str,
//         args: <Self::DB as sqlx::Database>::Arguments<'a>,
//     ) -> crate::Result<SE>
//     where
//         SE: taitan_orm_trait::SelectedEntity<Self::DB> + Send + Unpin,
//     {
//         let mut ex = self.get_pool()?.acquire().await?;
//         Self::generic_fetch_one_full(&mut *ex, stmt, args).await
//     }
//
//     async fn fetch_one_full_plain<'a, SE>(&'a mut self, stmt: &'a str) -> crate::Result<SE>
//     where
//         SE: taitan_orm_trait::SelectedEntity<Self::DB> + Send + Unpin,
//     {
//         let mut ex = self.get_pool()?.acquire().await?;
//         let args: std::marker::PhantomData<<Self::DB as sqlx::Database>::Arguments<'a>> =
//             std::marker::PhantomData::default();
//         Self::generic_fetch_one_full_plain(&mut *ex, stmt, args).await
//     }
//
//     async fn fetch_option_full<'a, SE>(
//         &'a mut self,
//         stmt: &'a str,
//         args: <Self::DB as sqlx::Database>::Arguments<'a>,
//     ) -> crate::Result<Option<SE>>
//     where
//         SE: taitan_orm_trait::SelectedEntity<Self::DB> + Send + Unpin,
//     {
//         let mut ex = self.get_pool()?.acquire().await?;
//         Self::generic_fetch_option_full(&mut *ex, stmt, args).await
//     }
//
//     async fn fetch_option_full_plain<'a, SE>(
//         &'a mut self,
//         stmt: &'a str,
//     ) -> crate::Result<Option<SE>>
//     where
//         SE: taitan_orm_trait::SelectedEntity<Self::DB> + Send + Unpin,
//     {
//         let mut ex = self.get_pool()?.acquire().await?;
//         let args: std::marker::PhantomData<<Self::DB as sqlx::Database>::Arguments<'a>> =
//             std::marker::PhantomData::default();
//         Self::generic_fetch_option_full_plain(&mut *ex, stmt, args).await
//     }
//
//     async fn fetch_all_full<'a, SE>(
//         &'a mut self,
//         stmt: &'a str,
//         args: <Self::DB as sqlx::Database>::Arguments<'a>,
//     ) -> crate::Result<Vec<SE>>
//     where
//         SE: taitan_orm_trait::SelectedEntity<Self::DB> + Send + Unpin,
//     {
//         let mut ex = self.get_pool()?.acquire().await?;
//         Self::generic_fetch_all_full(&mut *ex, stmt, args).await
//     }
//
//     async fn fetch_all_full_plain<'a, SE>(&'a mut self, stmt: &'a str) -> crate::Result<Vec<SE>>
//     where
//         SE: taitan_orm_trait::SelectedEntity<Self::DB> + Send + Unpin,
//     {
//         let mut ex = self.get_pool()?.acquire().await?;
//         let args: std::marker::PhantomData<<Self::DB as sqlx::Database>::Arguments<'a>> =
//             std::marker::PhantomData::default();
//         Self::generic_fetch_all_full_plain(&mut *ex, stmt, args).await
//     }
// }
