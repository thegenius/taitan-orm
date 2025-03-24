

#[doc(hidden)]
#[macro_export]
macro_rules! brave_new_transaction_impl {
    ($db: ty) => {
        async fn execute<'a>(
            &'a mut self,
            stmt: &'a str,
            args: <$db as sqlx::Database>::Arguments<'a>,
        ) -> taitan_orm_trait::result::Result<u64> {
            let ex = &mut *self.transaction;
            Self::generic_execute(ex, stmt, args).await
        }

        async fn execute_plain<'a>(&'a mut self, stmt: &'a str) -> taitan_orm_trait::result::Result<u64> {
            let args: std::marker::PhantomData<<$db as sqlx::Database>::Arguments<'a>> =
                std::marker::PhantomData::default();
            let ex = &mut *self.transaction;
            Self::generic_execute_plain(ex, stmt, args).await
        }

        async fn fetch_count<'s, 'a>(
            &'a mut self,
            stmt: &'s str,
            args: <$db as sqlx::Database>::Arguments<'a>,
        ) -> taitan_orm_trait::result::Result<u64>
        where
            'a: 's,
        {
            let ex = &mut *self.transaction;
            let result = Self::generic_count(ex, stmt, args).await?;
            Ok(result.count)
        }

        async fn fetch_count_plain<'a>(&'a mut self, stmt: &'a str) -> taitan_orm_trait::result::Result<u64> {
            let args: std::marker::PhantomData<<$db as sqlx::Database>::Arguments<'a>> =
                std::marker::PhantomData::default();
            let ex = &mut *self.transaction;
            let result = Self::generic_count_plain(ex, stmt, args).await?;
            Ok(result.count)
        }

        async fn fetch_exists<'a>(
            &'a mut self,
            stmt: &'a str,
            args: <$db as sqlx::Database>::Arguments<'a>,
        ) -> taitan_orm_trait::result::Result<bool> {
            let ex = &mut *self.transaction;
            Self::generic_exists(ex, stmt, args).await
        }

        async fn fetch_exists_plain<'a, A>(&'a mut self, stmt: &'a str) -> taitan_orm_trait::result::Result<bool> {
            let ex = &mut *self.transaction;
            let args: std::marker::PhantomData<<$db as sqlx::Database>::Arguments<'a>> =
                std::marker::PhantomData::default();
            Self::generic_exists_plain(ex, stmt, args).await
        }

        async fn fetch_option<'a, SE>(
            &'a mut self,
            stmt: &'a str,
            selection: &'a SE,
            args: <$db as sqlx::Database>::Arguments<'a>,
        ) -> taitan_orm_trait::result::Result<Option<SE>>
        where
            SE:  taitan_orm_trait::traits::Selected<$db> + Send + Unpin,
        {
            let ex = &mut *self.transaction;
            Self::generic_fetch_option(ex, stmt, selection, args).await
        }

        async fn fetch_option_<'a, SE>(
            &'a mut self,
            stmt: &'a str,
            selection: &'a SE,
            args: <$db as sqlx::Database>::Arguments<'a>,
        ) -> taitan_orm_trait::result::Result<Option<SE>>
        where
            SE:  taitan_orm_trait::traits::Selected<$db> + Send + Unpin,
        {
            let ex = &mut *self.transaction;
            Self::generic_fetch_option_(ex, stmt, selection, args).await
        }

        async fn fetch_option_plain<'a, SE>(
            &'a mut self,
            stmt: &'a str,
            selection: &'a SE,
        ) -> taitan_orm_trait::result::Result<Option<SE>>
        where
            SE:  taitan_orm_trait::traits::Selected<$db> + Send + Unpin,
        {
            let ex = &mut *self.transaction;
            let args: std::marker::PhantomData<<$db as sqlx::Database>::Arguments<'a>> =
                std::marker::PhantomData::default();
            Self::generic_fetch_option_plain(ex, stmt, selection, args).await
        }

        async fn fetch_option_plain_<'a, SE>(
            &'a mut self,
            stmt: &'a str,
            selection: &'a SE,
        ) -> taitan_orm_trait::result::Result<Option<SE>>
        where
            SE:  taitan_orm_trait::traits::Selected<$db> + Send + Unpin,
        {
            let ex = &mut *self.transaction;
            let args: std::marker::PhantomData<<$db as sqlx::Database>::Arguments<'a>> =
                std::marker::PhantomData::default();
            Self::generic_fetch_option_plain_(ex, stmt, selection, args).await
        }

        async fn fetch_all<'a, SE>(
            &'a mut self,
            stmt: &'a str,
            selection: &'a SE,
            args: <$db as sqlx::Database>::Arguments<'a>,
        ) -> taitan_orm_trait::result::Result<Vec<SE>>
        where
            SE:  taitan_orm_trait::traits::Selected<$db> + Send + Unpin,
        {
            let ex = &mut *self.transaction;
            Self::generic_fetch_all(ex, stmt, selection, args).await
        }

        async fn fetch_all_<'a, SE>(
            &'a mut self,
            stmt: &'a str,
            selection: &'a SE,
            args: <$db as sqlx::Database>::Arguments<'a>,
        ) -> taitan_orm_trait::result::Result<Vec<SE>>
        where
            SE:  taitan_orm_trait::traits::Selected<$db> + Send + Unpin,
        {
            let ex = &mut *self.transaction;
            Self::generic_fetch_all_(ex, stmt, selection, args).await
        }

        async fn fetch_all_plain<'a, SE>(
            &'a mut self,
            stmt: &'a str,
            selection: &'a SE,
        ) -> taitan_orm_trait::result::Result<Vec<SE>>
        where
            SE:  taitan_orm_trait::traits::Selected<$db> + Send + Unpin,
        {
            let ex = &mut *self.transaction;
            let args: std::marker::PhantomData<<$db as sqlx::Database>::Arguments<'a>> =
                std::marker::PhantomData::default();
            Self::generic_fetch_all_plain(ex, stmt, selection, args).await
        }

        async fn fetch_all_plain_<'a, SE>(
            &'a mut self,
            stmt: &'a str,
            selection: &'a SE,
        ) -> taitan_orm_trait::result::Result<Vec<SE>>
        where
            SE:  taitan_orm_trait::traits::Selected<$db> + Send + Unpin,
        {
            let ex = &mut *self.transaction;
            let args: std::marker::PhantomData<<$db as sqlx::Database>::Arguments<'a>> =
                std::marker::PhantomData::default();
            Self::generic_fetch_all_plain_(ex, stmt, selection, args).await
        }

        async fn fetch_one_full<'a, SE>(
            &'a mut self,
            stmt: &'a str,
            args: <$db as sqlx::Database>::Arguments<'a>,
        ) -> taitan_orm_trait::result::Result<SE>
        where
            SE:  taitan_orm_trait::traits::Selected<$db> + Send + Unpin,
        {
            let ex = &mut *self.transaction;
            Self::generic_fetch_one_full(ex, stmt, args).await
        }

        async fn fetch_one_full_plain<'a, SE>(&'a mut self, stmt: &'a str) -> taitan_orm_trait::result::Result<SE>
        where
            SE:  taitan_orm_trait::traits::Selected<$db> + Send + Unpin,
        {
            let ex = &mut *self.transaction;
            let args: std::marker::PhantomData<<$db as sqlx::Database>::Arguments<'a>> =
                std::marker::PhantomData::default();
            Self::generic_fetch_one_full_plain(ex, stmt, args).await
        }

        async fn fetch_option_full<'a, SE>(
            &'a mut self,
            stmt: &'a str,
            args: <$db as sqlx::Database>::Arguments<'a>,
        ) -> taitan_orm_trait::result::Result<Option<SE>>
        where
            SE:  taitan_orm_trait::traits::Selected<$db> + Send + Unpin,
        {
            let ex = &mut *self.transaction;
            Self::generic_fetch_option_full(ex, stmt, args).await
        }

        async fn fetch_option_full_plain<'a, SE>(
            &'a mut self,
            stmt: &'a str,
        ) -> taitan_orm_trait::result::Result<Option<SE>>
        where
            SE:  taitan_orm_trait::traits::Selected<$db> + Send + Unpin,
        {
            let ex = &mut *self.transaction;
            let args: std::marker::PhantomData<<$db as sqlx::Database>::Arguments<'a>> =
                std::marker::PhantomData::default();
            Self::generic_fetch_option_full_plain(ex, stmt, args).await
        }

        async fn fetch_all_full<'a, SE>(
            &'a mut self,
            stmt: &'a str,
            args: <$db as sqlx::Database>::Arguments<'a>,
        ) -> taitan_orm_trait::result::Result<Vec<SE>>
        where
            SE:  taitan_orm_trait::traits::Selected<$db> + Send + Unpin,
        {
            let ex = &mut *self.transaction;
            Self::generic_fetch_all_full(ex, stmt, args).await
        }

        async fn fetch_all_full_plain<'a, SE>(
            &'a mut self,
            stmt: &'a str,
        ) -> taitan_orm_trait::result::Result<Vec<SE>>
        where
            SE:  taitan_orm_trait::traits::Selected<$db> + Send + Unpin,
        {
            let ex = &mut *self.transaction;
            let args: std::marker::PhantomData<<$db as sqlx::Database>::Arguments<'a>> =
                std::marker::PhantomData::default();
            Self::generic_fetch_all_full_plain(ex, stmt, args).await
        }
    };
}
