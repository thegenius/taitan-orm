#[doc(hidden)]
#[macro_export]
macro_rules! executor_impl {
    ($conn_type:ty) => {
        async fn execute<'a>(
            &'a self,
            stmt: &'a str,
            args: <Self::DB as sqlx::Database>::Arguments<'a>,
        ) -> crate::result::Result<u64> {
            let ex = self.get_pool()?;
            Self::generic_execute(ex, stmt, args).await
        }

        async fn execute_plain<'a>(&'a self, stmt: &'a str) -> crate::result::Result<u64> {
            let args: std::marker::PhantomData<<Self::DB as sqlx::Database>::Arguments<'a>> =
                std::marker::PhantomData::default();
            let ex = self.get_pool()?;
            Self::generic_execute_plain(ex, stmt, args).await
        }

        async fn fetch_count<'s, 'a>(
            &'a self,
            stmt: &'s str,
            args: <Self::DB as sqlx::Database>::Arguments<'a>,
        ) -> crate::result::Result<u64>
        where
            'a: 's,
        {
            let ex = self.get_pool()?;
            let result = Self::generic_count(ex, stmt, args).await?;
            Ok(result.count)
        }

        async fn fetch_count_plain<'a>(&'a self, stmt: &'a str) -> crate::result::Result<u64> {
            let args: std::marker::PhantomData<<Self::DB as sqlx::Database>::Arguments<'a>> =
                std::marker::PhantomData::default();
            let ex = self.get_pool()?;
            let result = Self::generic_count_plain(ex, stmt, args).await?;
            Ok(result.count)
        }

        async fn fetch_exists<'a>(
            &'a self,
            stmt: &'a str,
            args: <Self::DB as sqlx::Database>::Arguments<'a>,
        ) -> crate::result::Result<bool> {
            let ex = self.get_pool()?;
            Self::generic_exists(ex, stmt, args).await
        }

        async fn fetch_exists_plain<'a, A>(&'a self, stmt: &'a str) -> crate::result::Result<bool> {
            let ex = self.get_pool()?;
            let args: std::marker::PhantomData<<Self::DB as sqlx::Database>::Arguments<'a>> =
                std::marker::PhantomData::default();
            Self::generic_exists_plain(ex, stmt, args).await
        }

        async fn fetch_option<'a, SE>(
            &'a self,
            stmt: &'a str,
            selection: &'a SE::Selection,
            args: <Self::DB as sqlx::Database>::Arguments<'a>,
        ) -> crate::result::Result<Option<SE>>
        where
            SE: taitan_orm_trait::SelectedEntity<Self::DB> + Send + Unpin,
        {
            let ex = self.get_pool()?;
            Self::generic_fetch_option(ex, stmt, selection, args).await
        }

        async fn fetch_option_<'a, SE>(
            &'a self,
            stmt: &'a str,
            selection: &'a SE,
            args: <Self::DB as sqlx::Database>::Arguments<'a>,
        ) -> crate::result::Result<Option<SE>>
        where
            SE: taitan_orm_trait::SelectedEntity<Self::DB> + Send + Unpin,
        {
            let ex = self.get_pool()?;
            Self::generic_fetch_option_(ex, stmt, selection, args).await
        }

        async fn fetch_option_plain<'a, SE>(
            &'a self,
            stmt: &'a str,
            selection: &'a SE::Selection,
        ) -> crate::result::Result<Option<SE>>
        where
            SE: taitan_orm_trait::SelectedEntity<Self::DB> + Send + Unpin,
        {
            let ex = self.get_pool()?;
            let args: std::marker::PhantomData<<Self::DB as sqlx::Database>::Arguments<'a>> =
                std::marker::PhantomData::default();
            Self::generic_fetch_option_plain(ex, stmt, selection, args).await
        }

        async fn fetch_option_plain_<'a, SE>(
            &'a self,
            stmt: &'a str,
            selection: &'a SE,
        ) -> crate::result::Result<Option<SE>>
        where
            SE: taitan_orm_trait::SelectedEntity<Self::DB> + Send + Unpin,
        {
            let ex = self.get_pool()?;
            let args: std::marker::PhantomData<<Self::DB as sqlx::Database>::Arguments<'a>> =
                std::marker::PhantomData::default();
            Self::generic_fetch_option_plain_(ex, stmt, selection, args).await
        }

        async fn fetch_all<'a, SE>(
            &'a self,
            stmt: &'a str,
            selection: &'a SE::Selection,
            args: <Self::DB as sqlx::Database>::Arguments<'a>,
        ) -> crate::result::Result<Vec<SE>>
        where
            SE: taitan_orm_trait::SelectedEntity<Self::DB> + Send + Unpin,
        {
            let ex = self.get_pool()?;
            Self::generic_fetch_all(ex, stmt, selection, args).await
        }

        async fn fetch_all_<'a, SE>(
            &'a self,
            stmt: &'a str,
            selection: &'a SE,
            args: <Self::DB as sqlx::Database>::Arguments<'a>,
        ) -> crate::result::Result<Vec<SE>>
        where
            SE: taitan_orm_trait::SelectedEntity<Self::DB> + Send + Unpin,
        {
            let ex = self.get_pool()?;
            Self::generic_fetch_all_(ex, stmt, selection, args).await
        }

        async fn fetch_all_plain<'a, SE>(
            &'a self,
            stmt: &'a str,
            selection: &'a SE::Selection,
        ) -> crate::result::Result<Vec<SE>>
        where
            SE: taitan_orm_trait::SelectedEntity<Self::DB> + Send + Unpin,
        {
            let ex = self.get_pool()?;
            let args: std::marker::PhantomData<<Self::DB as sqlx::Database>::Arguments<'a>> =
                std::marker::PhantomData::default();
            Self::generic_fetch_all_plain(ex, stmt, selection, args).await
        }

        async fn fetch_all_plain_<'a, SE>(
            &'a self,
            stmt: &'a str,
            selection: &'a SE,
        ) -> crate::result::Result<Vec<SE>>
        where
            SE: taitan_orm_trait::SelectedEntity<Self::DB> + Send + Unpin,
        {
            let ex = self.get_pool()?;
            let args: std::marker::PhantomData<<Self::DB as sqlx::Database>::Arguments<'a>> =
                std::marker::PhantomData::default();
            Self::generic_fetch_all_plain_(ex, stmt, selection, args).await
        }

        async fn fetch_one_full<'a, SE>(
            &'a self,
            stmt: &'a str,
            args: <Self::DB as sqlx::Database>::Arguments<'a>,
        ) -> crate::result::Result<SE>
        where
            SE: taitan_orm_trait::SelectedEntity<Self::DB> + Send + Unpin,
        {
            let ex = self.get_pool()?;
            Self::generic_fetch_one_full(ex, stmt, args).await
        }

        async fn fetch_one_full_plain<'a, SE>(&'a self, stmt: &'a str) -> crate::result::Result<SE>
        where
            SE: taitan_orm_trait::SelectedEntity<Self::DB> + Send + Unpin,
        {
            let ex = self.get_pool()?;
            let args: std::marker::PhantomData<<Self::DB as sqlx::Database>::Arguments<'a>> =
                std::marker::PhantomData::default();
            Self::generic_fetch_one_full_plain(ex, stmt, args).await
        }

        async fn fetch_option_full<'a, SE>(
            &'a self,
            stmt: &'a str,
            args: <Self::DB as sqlx::Database>::Arguments<'a>,
        ) -> crate::result::Result<Option<SE>>
        where
            SE: taitan_orm_trait::SelectedEntity<Self::DB> + Send + Unpin,
        {
            let ex = self.get_pool()?;
            Self::generic_fetch_option_full(ex, stmt, args).await
        }

        async fn fetch_option_full_plain<'a, SE>(
            &'a self,
            stmt: &'a str,
        ) -> crate::result::Result<Option<SE>>
        where
            SE: taitan_orm_trait::SelectedEntity<Self::DB> + Send + Unpin,
        {
            let ex = self.get_pool()?;
            let args: std::marker::PhantomData<<Self::DB as sqlx::Database>::Arguments<'a>> =
                std::marker::PhantomData::default();
            Self::generic_fetch_option_full_plain(ex, stmt, args).await
        }

        async fn fetch_all_full<'a, SE>(
            &'a self,
            stmt: &'a str,
            args: <Self::DB as sqlx::Database>::Arguments<'a>,
        ) -> crate::result::Result<Vec<SE>>
        where
            SE: taitan_orm_trait::SelectedEntity<Self::DB> + Send + Unpin,
        {
            let ex = self.get_pool()?;
            Self::generic_fetch_all_full(ex, stmt, args).await
        }

        async fn fetch_all_full_plain<'a, SE>(
            &'a self,
            stmt: &'a str,
        ) -> crate::result::Result<Vec<SE>>
        where
            SE: taitan_orm_trait::SelectedEntity<Self::DB> + Send + Unpin,
        {
            let ex = self.get_pool()?;
            let args: std::marker::PhantomData<<Self::DB as sqlx::Database>::Arguments<'a>> =
                std::marker::PhantomData::default();
            Self::generic_fetch_all_full_plain(ex, stmt, args).await
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! execute_fn {
    () => {
        async fn execute<'a, A>(&'a self, stmt: &'a str, args: A) -> Result<u64>
        where
            A: IntoArguments<'a, Self::DB> + 'a,
        {
            let mut ex = self.get_connection().await?;
            Self::generic_execute(ex, stmt, args).await
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! execute_plain_fn {
    () => {
        async fn execute_plain<'a>(&'a self, stmt: &'a str) -> crate::result::Result<u64> {
            let args: std::marker::PhantomData<SqliteArguments> =
                std::marker::PhantomData::default();
            let ex = self.get_pool()?;
            Self::generic_execute_plain(ex, stmt, args).await
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! fetch_exists_fn {
    () => {
        async fn fetch_exists<'a>(
            &'a self,
            stmt: &'a str,
            args: <Self::DB as Database>::Arguments<'a>,
        ) -> Result<bool> {
            let ex = self.get_pool()?;
            Self::generic_exists(ex, stmt, args).await
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! fetch_exists_plain_fn {
    () => {
        async fn fetch_exists_plain<'a, A>(&'a self, stmt: &'a str) -> Result<bool>
        where
            A: IntoArguments<'a, crate::result::Result::DB> + 'a + Default,
        {
            let ex = self.get_pool()?;
            let args: std::marker::PhantomData<SqliteArguments> =
                std::marker::PhantomData::default();
            Self::generic_exists_plain(ex, stmt, args).await
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! fetch_option_fn {
    () => {
        async fn fetch_option<'a, SE>(
            &'a self,
            stmt: &'a str,
            selection: &'a SE::Selection,
            args: SqliteArguments<'a>,
        ) -> Result<Option<SE>>
        where
            SE: taitan_orm_trait::SelectedEntity<Self::DB> + Send + Unpin,
        {
            let ex = self.get_pool()?;
            Self::generic_fetch_option(ex, stmt, selection, args).await
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! fetch_option_plain_fn {
    () => {
        async fn fetch_option_plain<'a, SE>(
            &'a self,
            stmt: &'a str,
            selection: &'a SE::Selection,
        ) -> Result<Option<SE>>
        where
            SE: taitan_orm_trait::SelectedEntity<Self::DB> + Send + Unpin,
        {
            let ex = self.get_pool()?;
            let args: std::marker::PhantomData<SqliteArguments> =
                std::marker::PhantomData::default();
            Self::generic_fetch_option_plain(ex, stmt, selection, args).await
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! fetch_all_fn {
    () => {
        async fn fetch_all<'a, SE>(
            &'a self,
            stmt: &'a str,
            selection: &'a SE::Selection,
            args: SqliteArguments<'a>,
        ) -> Result<Vec<SE>>
        where
            SE: taitan_orm_trait::SelectedEntity<Self::DB> + Send + Unpin,
        {
            let ex = self.get_pool()?;
            Self::generic_fetch_all(ex, stmt, selection, args).await
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! fetch_all_plain_fn {
    () => {
        async fn fetch_all_plain<'a, SE>(
            &'a self,
            stmt: &'a str,
            selection: &'a SE::Selection,
        ) -> Result<Vec<SE>>
        where
            SE: taitan_orm_trait::SelectedEntity<Self::DB> + Send + Unpin,
        {
            let ex = self.get_pool()?;
            let args: std::marker::PhantomData<SqliteArguments> =
                std::marker::PhantomData::default();
            Self::generic_fetch_all_plain(ex, stmt, selection, args).await
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! fetch_one_full_fn {
    () => {
        async fn fetch_one_full<'a, SE>(
            &'a self,
            stmt: &'a str,
            args: SqliteArguments<'a>,
        ) -> Result<SE>
        where
            SE: taitan_orm_trait::SelectedEntity<Self::DB> + Send + Unpin,
        {
            let ex = self.get_pool()?;
            Self::generic_fetch_one_full(ex, stmt, args).await
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! fetch_one_full_plain_fn {
    () => {
        async fn fetch_one_full_plain<'a, SE>(&'a self, stmt: &'a str) -> Result<SE>
        where
            SE: taitan_orm_trait::SelectedEntity<Self::DB> + Send + Unpin,
        {
            let ex = self.get_pool()?;
            let args: std::marker::PhantomData<SqliteArguments> =
                std::marker::PhantomData::default();
            Self::generic_fetch_one_full_plain(ex, stmt, args).await
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! fetch_option_full_fn {
    () => {
        async fn fetch_option_full<'a, SE>(
            &'a self,
            stmt: &'a str,
            args: <Self::DB as Database>::Arguments<'a>,
        ) -> Result<Option<SE>>
        where
            SE: taitan_orm_trait::SelectedEntity<Self::DB> + Send + Unpin,
        {
            let ex = self.get_pool()?;
            Self::generic_fetch_option_full(ex, stmt, args).await
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! fetch_option_full_plain_fn {
    () => {
        async fn fetch_option_full_plain<'a, SE>(&'a self, stmt: &'a str) -> Result<Option<SE>>
        where
            SE: taitan_orm_trait::SelectedEntity<Self::DB> + Send + Unpin,
        {
            let ex = self.get_pool()?;
            let args: std::marker::PhantomData<SqliteArguments> =
                std::marker::PhantomData::default();
            Self::generic_fetch_option_full_plain(ex, stmt, args).await
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! fetch_all_full_fn {
    () => {
        async fn fetch_all_full<'a, SE>(
            &'a self,
            stmt: &'a str,
            args: <Self::DB as Database>::Arguments<'a>,
        ) -> Result<Vec<SE>>
        where
            SE: taitan_orm_trait::SelectedEntity<Self::DB> + Send + Unpin,
        {
            let ex = self.get_pool()?;
            Self::generic_fetch_all_full(ex, stmt, args).await
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! fetch_all_full_plain_fn {
    () => {
        async fn fetch_all_full_plain<'a, SE>(&'a self, stmt: &'a str) -> Result<Vec<SE>>
        where
            SE: taitan_orm_trait::SelectedEntity<Self::DB> + Send + Unpin,
        {
            let ex = self.get_pool()?;
            let args: std::marker::PhantomData<SqliteArguments> =
                std::marker::PhantomData::default();
            Self::generic_fetch_all_full_plain(ex, stmt, args).await
        }
    };
}
