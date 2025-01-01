#[macro_export]
macro_rules! executor_impl {
    ($conn_type:ty) => {
        // type Connection = $conn_type;
        //
        // #[inline(always)]
        // async fn get_connection(&mut self) -> crate::Result<sqlx::pool::PoolConnection<Self::DB>> {
        //     Ok(self.get_pool()?.acquire().await?)
        // }

        async fn execute<'a>(
            &'a mut self,
            stmt: &'a str,
            args: <Self::DB as sqlx::Database>::Arguments<'a>,
        ) -> crate::Result<u64> {
            let mut ex = self.get_connection().await?;
            Self::generic_execute(&mut *ex, stmt, args).await
        }

        async fn execute_plain<'a>(&'a mut self, stmt: &'a str) -> crate::Result<u64> {
            let args: std::marker::PhantomData<<Self::DB as sqlx::Database>::Arguments<'a>> =
                std::marker::PhantomData::default();
            let mut ex = self.get_pool()?.acquire().await?;
            Self::generic_execute_plain(&mut *ex, stmt, args).await
        }

        async fn fetch_count<'s, 'a>(
            &'a mut self,
            stmt: &'s str,
            args: <Self::DB as sqlx::Database>::Arguments<'a>,
        ) -> crate::Result<u64> where 'a: 's{
            let mut ex = self.get_connection().await?;
            let result = Self::generic_count(&mut *ex, stmt, args).await?;
            Ok(result.count)
        }

        async fn fetch_count_plain<'a>(&'a mut self, stmt: &'a str) -> crate::Result<u64> {
            let args: std::marker::PhantomData<<Self::DB as sqlx::Database>::Arguments<'a>> =
                std::marker::PhantomData::default();
            let mut ex = self.get_pool()?.acquire().await?;
            let result = Self::generic_count_plain(&mut *ex, stmt, args).await?;
            Ok(result.count)
        }


        async fn fetch_exists<'a>(
            &'a mut self,
            stmt: &'a str,
            args: <Self::DB as sqlx::Database>::Arguments<'a>,
        ) -> crate::Result<bool> {
            let mut ex = self.get_pool()?.acquire().await?;
            Self::generic_exists(&mut *ex, stmt, args).await
        }

        async fn fetch_exists_plain<'a, A>(&'a mut self, stmt: &'a str) -> crate::Result<bool> {
            let mut ex = self.get_pool()?.acquire().await?;
            let args: std::marker::PhantomData<<Self::DB as sqlx::Database>::Arguments<'a>> =
                std::marker::PhantomData::default();
            Self::generic_exists_plain(&mut *ex, stmt, args).await
        }

        async fn fetch_option<'a, SE>(
            &'a mut self,
            stmt: &'a str,
            selection: &'a SE::Selection,
            args: <Self::DB as sqlx::Database>::Arguments<'a>,
        ) -> crate::Result<Option<SE>>
        where
            SE: taitan_orm_trait::SelectedEntity<Self::DB> + Send + Unpin,
        {
            let mut ex = self.get_pool()?.acquire().await?;
            Self::generic_fetch_option(&mut *ex, stmt, selection, args).await
        }

        async fn fetch_option_<'a, SE>(
            &'a mut self,
            stmt: &'a str,
            selection: &'a SE,
            args: <Self::DB as sqlx::Database>::Arguments<'a>,
        ) -> crate::Result<Option<SE>>
        where
            SE: taitan_orm_trait::SelectedEntity<Self::DB> + Send + Unpin,
        {
            let mut ex = self.get_pool()?.acquire().await?;
            Self::generic_fetch_option_(&mut *ex, stmt, selection, args).await
        }

        async fn fetch_option_plain<'a, SE>(
            &'a mut self,
            stmt: &'a str,
            selection: &'a SE::Selection,
        ) -> crate::Result<Option<SE>>
        where
            SE: taitan_orm_trait::SelectedEntity<Self::DB> + Send + Unpin,
        {
            let mut ex = self.get_pool()?.acquire().await?;
            let args: std::marker::PhantomData<<Self::DB as sqlx::Database>::Arguments<'a>> =
                std::marker::PhantomData::default();
            Self::generic_fetch_option_plain(&mut *ex, stmt, selection, args).await
        }

        async fn fetch_option_plain_<'a, SE>(
            &'a mut self,
            stmt: &'a str,
            selection: &'a SE,
        ) -> crate::Result<Option<SE>>
        where
            SE: taitan_orm_trait::SelectedEntity<Self::DB> + Send + Unpin,
        {
            let mut ex = self.get_pool()?.acquire().await?;
            let args: std::marker::PhantomData<<Self::DB as sqlx::Database>::Arguments<'a>> =
                std::marker::PhantomData::default();
            Self::generic_fetch_option_plain_(&mut *ex, stmt, selection, args).await
        }

        async fn fetch_all<'a, SE>(
            &'a mut self,
            stmt: &'a str,
            selection: &'a SE::Selection,
            args: <Self::DB as sqlx::Database>::Arguments<'a>,
        ) -> crate::Result<Vec<SE>>
        where
            SE: taitan_orm_trait::SelectedEntity<Self::DB> + Send + Unpin,
        {
            let mut ex = self.get_pool()?.acquire().await?;
            Self::generic_fetch_all(&mut *ex, stmt, selection, args).await
        }

        async fn fetch_all_<'a, SE>(
            &'a mut self,
            stmt: &'a str,
            selection: &'a SE,
            args: <Self::DB as sqlx::Database>::Arguments<'a>,
        ) -> crate::Result<Vec<SE>>
        where
            SE: taitan_orm_trait::SelectedEntity<Self::DB> + Send + Unpin,
        {
            let mut ex = self.get_pool()?.acquire().await?;
            Self::generic_fetch_all_(&mut *ex, stmt, selection, args).await
        }

        async fn fetch_all_plain<'a, SE>(
            &'a mut self,
            stmt: &'a str,
            selection: &'a SE::Selection,
        ) -> crate::Result<Vec<SE>>
        where
            SE: taitan_orm_trait::SelectedEntity<Self::DB> + Send + Unpin,
        {
            let mut ex = self.get_pool()?.acquire().await?;
            let args: std::marker::PhantomData<<Self::DB as sqlx::Database>::Arguments<'a>> =
                std::marker::PhantomData::default();
            Self::generic_fetch_all_plain(&mut *ex, stmt, selection, args).await
        }

        async fn fetch_all_plain_<'a, SE>(
            &'a mut self,
            stmt: &'a str,
            selection: &'a SE,
        ) -> crate::Result<Vec<SE>>
        where
            SE: taitan_orm_trait::SelectedEntity<Self::DB> + Send + Unpin,
        {
            let mut ex = self.get_pool()?.acquire().await?;
            let args: std::marker::PhantomData<<Self::DB as sqlx::Database>::Arguments<'a>> =
                std::marker::PhantomData::default();
            Self::generic_fetch_all_plain_(&mut *ex, stmt, selection, args).await
        }

        async fn fetch_one_full<'a, SE>(
            &'a mut self,
            stmt: &'a str,
            args: <Self::DB as sqlx::Database>::Arguments<'a>,
        ) -> crate::Result<SE>
        where
            SE: taitan_orm_trait::SelectedEntity<Self::DB> + Send + Unpin,
        {
            let mut ex = self.get_pool()?.acquire().await?;
            Self::generic_fetch_one_full(&mut *ex, stmt, args).await
        }

        async fn fetch_one_full_plain<'a, SE>(&'a mut self, stmt: &'a str) -> crate::Result<SE>
        where
            SE: taitan_orm_trait::SelectedEntity<Self::DB> + Send + Unpin,
        {
            let mut ex = self.get_pool()?.acquire().await?;
            let args: std::marker::PhantomData<<Self::DB as sqlx::Database>::Arguments<'a>> =
                std::marker::PhantomData::default();
            Self::generic_fetch_one_full_plain(&mut *ex, stmt, args).await
        }

        async fn fetch_option_full<'a, SE>(
            &'a mut self,
            stmt: &'a str,
            args: <Self::DB as sqlx::Database>::Arguments<'a>,
        ) -> crate::Result<Option<SE>>
        where
            SE: taitan_orm_trait::SelectedEntity<Self::DB> + Send + Unpin,
        {
            let mut ex = self.get_pool()?.acquire().await?;
            Self::generic_fetch_option_full(&mut *ex, stmt, args).await
        }

        async fn fetch_option_full_plain<'a, SE>(
            &'a mut self,
            stmt: &'a str,
        ) -> crate::Result<Option<SE>>
        where
            SE: taitan_orm_trait::SelectedEntity<Self::DB> + Send + Unpin,
        {
            let mut ex = self.get_pool()?.acquire().await?;
            let args: std::marker::PhantomData<<Self::DB as sqlx::Database>::Arguments<'a>> =
                std::marker::PhantomData::default();
            Self::generic_fetch_option_full_plain(&mut *ex, stmt, args).await
        }

        async fn fetch_all_full<'a, SE>(
            &'a mut self,
            stmt: &'a str,
            args: <Self::DB as sqlx::Database>::Arguments<'a>,
        ) -> crate::Result<Vec<SE>>
        where
            SE: taitan_orm_trait::SelectedEntity<Self::DB> + Send + Unpin,
        {
            let mut ex = self.get_pool()?.acquire().await?;
            Self::generic_fetch_all_full(&mut *ex, stmt, args).await
        }

        async fn fetch_all_full_plain<'a, SE>(&'a mut self, stmt: &'a str) -> crate::Result<Vec<SE>>
        where
            SE: taitan_orm_trait::SelectedEntity<Self::DB> + Send + Unpin,
        {
            let mut ex = self.get_pool()?.acquire().await?;
            let args: std::marker::PhantomData<<Self::DB as sqlx::Database>::Arguments<'a>> =
                std::marker::PhantomData::default();
            Self::generic_fetch_all_full_plain(&mut *ex, stmt, args).await
        }
    };
}

#[macro_export]
macro_rules! execute_fn {
    () => {
        async fn execute<'a, A>(&'a mut self, stmt: &'a str, args: A) -> Result<u64>
        where
            A: IntoArguments<'a, Self::DB> + 'a,
        {
            let mut ex = self.get_connection().await?;
            Self::generic_execute(&mut *ex, stmt, args).await
        }
    };
}

#[macro_export]
macro_rules! execute_plain_fn {
    () => {
        async fn execute_plain<'a>(&'a mut self, stmt: &'a str) -> crate::Result<u64> {
            let args: std::marker::PhantomData<SqliteArguments> =
                std::marker::PhantomData::default();
            let mut ex = self.get_pool()?.acquire().await?;
            Self::generic_execute_plain(&mut *ex, stmt, args).await
        }
    };
}

#[macro_export]
macro_rules! fetch_exists_fn {
    () => {
        async fn fetch_exists<'a>(
            &'a mut self,
            stmt: &'a str,
            args: <Self::DB as Database>::Arguments<'a>,
        ) -> Result<bool> {
            let mut ex = self.get_pool()?.acquire().await?;
            Self::generic_exists(&mut *ex, stmt, args).await
        }
    };
}

#[macro_export]
macro_rules! fetch_exists_plain_fn {
    () => {
        async fn fetch_exists_plain<'a, A>(&'a mut self, stmt: &'a str) -> Result<bool>
        where
            A: IntoArguments<'a, crate::result::Result::DB> + 'a + Default,
        {
            let mut ex = self.get_pool()?.acquire().await?;
            let args: std::marker::PhantomData<SqliteArguments> =
                std::marker::PhantomData::default();
            Self::generic_exists_plain(&mut *ex, stmt, args).await
        }
    };
}

#[macro_export]
macro_rules! fetch_option_fn {
    () => {
        async fn fetch_option<'a, SE>(
            &'a mut self,
            stmt: &'a str,
            selection: &'a SE::Selection,
            args: SqliteArguments<'a>,
        ) -> Result<Option<SE>>
        where
            SE: taitan_orm_trait::SelectedEntity<Self::DB> + Send + Unpin,
        {
            let mut ex = self.get_pool()?.acquire().await?;
            Self::generic_fetch_option(&mut *ex, stmt, selection, args).await
        }
    };
}

#[macro_export]
macro_rules! fetch_option_plain_fn {
    () => {
        async fn fetch_option_plain<'a, SE>(
            &'a mut self,
            stmt: &'a str,
            selection: &'a SE::Selection,
        ) -> Result<Option<SE>>
        where
            SE: taitan_orm_trait::SelectedEntity<Self::DB> + Send + Unpin,
        {
            let mut ex = self.get_pool()?.acquire().await?;
            let args: std::marker::PhantomData<SqliteArguments> =
                std::marker::PhantomData::default();
            Self::generic_fetch_option_plain(&mut *ex, stmt, selection, args).await
        }
    };
}

#[macro_export]
macro_rules! fetch_all_fn {
    () => {
        async fn fetch_all<'a, SE>(
            &'a mut self,
            stmt: &'a str,
            selection: &'a SE::Selection,
            args: SqliteArguments<'a>,
        ) -> Result<Vec<SE>>
        where
            SE: taitan_orm_trait::SelectedEntity<Self::DB> + Send + Unpin,
        {
            let mut ex = self.get_pool()?.acquire().await?;
            Self::generic_fetch_all(&mut *ex, stmt, selection, args).await
        }
    };
}

#[macro_export]
macro_rules! fetch_all_plain_fn {
    () => {
        async fn fetch_all_plain<'a, SE>(
            &'a mut self,
            stmt: &'a str,
            selection: &'a SE::Selection,
        ) -> Result<Vec<SE>>
        where
            SE: taitan_orm_trait::SelectedEntity<Self::DB> + Send + Unpin,
        {
            let mut ex = self.get_pool()?.acquire().await?;
            let args: std::marker::PhantomData<SqliteArguments> =
                std::marker::PhantomData::default();
            Self::generic_fetch_all_plain(&mut *ex, stmt, selection, args).await
        }
    };
}

#[macro_export]
macro_rules! fetch_one_full_fn {
    () => {
        async fn fetch_one_full<'a, SE>(
            &'a mut self,
            stmt: &'a str,
            args: SqliteArguments<'a>,
        ) -> Result<SE>
        where
            SE: taitan_orm_trait::SelectedEntity<Self::DB> + Send + Unpin,
        {
            let mut ex = self.get_pool()?.acquire().await?;
            Self::generic_fetch_one_full(&mut *ex, stmt, args).await
        }
    };
}

#[macro_export]
macro_rules! fetch_one_full_plain_fn {
    () => {
        async fn fetch_one_full_plain<'a, SE>(&'a mut self, stmt: &'a str) -> Result<SE>
        where
            SE: taitan_orm_trait::SelectedEntity<Self::DB> + Send + Unpin,
        {
            let mut ex = self.get_pool()?.acquire().await?;
            let args: std::marker::PhantomData<SqliteArguments> =
                std::marker::PhantomData::default();
            Self::generic_fetch_one_full_plain(&mut *ex, stmt, args).await
        }
    };
}

#[macro_export]
macro_rules! fetch_option_full_fn {
    () => {
        async fn fetch_option_full<'a, SE>(
            &'a mut self,
            stmt: &'a str,
            args: <Self::DB as Database>::Arguments<'a>,
        ) -> Result<Option<SE>>
        where
            SE: taitan_orm_trait::SelectedEntity<Self::DB> + Send + Unpin,
        {
            let mut ex = self.get_pool()?.acquire().await?;
            Self::generic_fetch_option_full(&mut *ex, stmt, args).await
        }
    };
}

#[macro_export]
macro_rules! fetch_option_full_plain_fn {
    () => {
        async fn fetch_option_full_plain<'a, SE>(&'a mut self, stmt: &'a str) -> Result<Option<SE>>
        where
            SE: taitan_orm_trait::SelectedEntity<Self::DB> + Send + Unpin,
        {
            let mut ex = self.get_pool()?.acquire().await?;
            let args: std::marker::PhantomData<SqliteArguments> =
                std::marker::PhantomData::default();
            Self::generic_fetch_option_full_plain(&mut *ex, stmt, args).await
        }
    };
}

#[macro_export]
macro_rules! fetch_all_full_fn {
    () => {
        async fn fetch_all_full<'a, SE>(
            &'a mut self,
            stmt: &'a str,
            args: <Self::DB as Database>::Arguments<'a>,
        ) -> Result<Vec<SE>>
        where
            SE: taitan_orm_trait::SelectedEntity<Self::DB> + Send + Unpin,
        {
            let mut ex = self.get_pool()?.acquire().await?;
            Self::generic_fetch_all_full(&mut *ex, stmt, args).await
        }
    };
}

#[macro_export]
macro_rules! fetch_all_full_plain_fn {
    () => {
        async fn fetch_all_full_plain<'a, SE>(&'a mut self, stmt: &'a str) -> Result<Vec<SE>>
        where
            SE: taitan_orm_trait::SelectedEntity<Self::DB> + Send + Unpin,
        {
            let mut ex = self.get_pool()?.acquire().await?;
            let args: std::marker::PhantomData<SqliteArguments> =
                std::marker::PhantomData::default();
            Self::generic_fetch_all_full_plain(&mut *ex, stmt, args).await
        }
    };
}
