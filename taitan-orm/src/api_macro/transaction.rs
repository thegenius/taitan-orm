#[macro_export]
macro_rules! transaction_impl {
    ($conn_type:ty) => {
        // type Connection = $conn_type;

        async fn execute<'a>(
            &'a mut self,
            stmt: &'a str,
            args: <Self::DB as sqlx::Database>::Arguments<'a>,
        ) -> crate::Result<u64> {
            Self::generic_execute(&mut *self.transaction, stmt, args).await
        }

        async fn execute_plain<'a>(&'a mut self, stmt: &'a str) -> crate::Result<u64> {
            let args: std::marker::PhantomData<<Self::DB as sqlx::Database>::Arguments<'a>> =
                std::marker::PhantomData::default();
            Self::generic_execute_plain(&mut *(self.transaction), stmt, args).await
        }

        async fn fetch_exists<'a>(
            &'a mut self,
            stmt: &'a str,
            args: <Self::DB as sqlx::Database>::Arguments<'a>,
        ) -> crate::Result<bool> {
            Self::generic_exists(&mut *self.transaction, stmt, args).await
        }

        async fn fetch_exists_plain<'a, A>(&'a mut self, stmt: &'a str) -> crate::Result<bool> {
            let args: std::marker::PhantomData<<Self::DB as sqlx::Database>::Arguments<'a>> =
                std::marker::PhantomData::default();
            Self::generic_exists_plain(&mut *self.transaction, stmt, args).await
        }

         async fn fetch_count<'s, 'a>(
            &'a mut self,
            stmt: &'s str,
            args: <Self::DB as sqlx::Database>::Arguments<'a>,
        ) -> crate::Result<u64> where 'a: 's {
            let result = Self::generic_count(&mut *self.transaction, stmt, args).await?;
            Ok(result.count)
        }

        async fn fetch_count_plain<'a>(&'a mut self, stmt: &'a str) -> crate::Result<u64> {
            let args: std::marker::PhantomData<<Self::DB as sqlx::Database>::Arguments<'a>> =
                std::marker::PhantomData::default();
            let result = Self::generic_count_plain(&mut *self.transaction, stmt, args).await?;
            Ok(result.count)
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
            Self::generic_fetch_option(&mut *self.transaction, stmt, selection, args).await
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
            Self::generic_fetch_option_(&mut *self.transaction, stmt, selection, args).await
        }


        async fn fetch_option_plain<'a, SE>(
            &'a mut self,
            stmt: &'a str,
            selection: &'a SE::Selection,
        ) -> crate::Result<Option<SE>>
        where
            SE: taitan_orm_trait::SelectedEntity<Self::DB> + Send + Unpin,
        {
            let args: std::marker::PhantomData<<Self::DB as sqlx::Database>::Arguments<'a>> =
                std::marker::PhantomData::default();
            Self::generic_fetch_option_plain(&mut *self.transaction, stmt, selection, args).await
        }

        async fn fetch_option_plain_<'a, SE>(
            &'a mut self,
            stmt: &'a str,
            selection: &'a SE,
        ) -> crate::Result<Option<SE>>
        where
            SE: taitan_orm_trait::SelectedEntity<Self::DB> + Send + Unpin,
        {
            let args: std::marker::PhantomData<<Self::DB as sqlx::Database>::Arguments<'a>> =
                std::marker::PhantomData::default();
            Self::generic_fetch_option_plain_(&mut *self.transaction, stmt, selection, args).await
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
            Self::generic_fetch_all(&mut *self.transaction, stmt, selection, args).await
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
            Self::generic_fetch_all_(&mut *self.transaction, stmt, selection, args).await
        }

        async fn fetch_all_plain<'a, SE>(
            &'a mut self,
            stmt: &'a str,
            selection: &'a SE::Selection,
        ) -> crate::Result<Vec<SE>>
        where
            SE: taitan_orm_trait::SelectedEntity<Self::DB> + Send + Unpin,
        {
            let args: std::marker::PhantomData<<Self::DB as sqlx::Database>::Arguments<'a>> =
                std::marker::PhantomData::default();
            Self::generic_fetch_all_plain(&mut *self.transaction, stmt, selection, args).await
        }

        async fn fetch_all_plain_<'a, SE>(
            &'a mut self,
            stmt: &'a str,
            selection: &'a SE,
        ) -> crate::Result<Vec<SE>>
        where
            SE: taitan_orm_trait::SelectedEntity<Self::DB> + Send + Unpin,
        {
            let args: std::marker::PhantomData<<Self::DB as sqlx::Database>::Arguments<'a>> =
                std::marker::PhantomData::default();
            Self::generic_fetch_all_plain_(&mut *self.transaction, stmt, selection, args).await
        }

        async fn fetch_one_full<'a, SE>(
            &'a mut self,
            stmt: &'a str,
            args: <Self::DB as sqlx::Database>::Arguments<'a>,
        ) -> crate::Result<SE>
        where
            SE: taitan_orm_trait::SelectedEntity<Self::DB> + Send + Unpin,
        {
            Self::generic_fetch_one_full(&mut *self.transaction, stmt, args).await
        }

        async fn fetch_one_full_plain<'a, SE>(&'a mut self, stmt: &'a str) -> crate::Result<SE>
        where
            SE: taitan_orm_trait::SelectedEntity<Self::DB> + Send + Unpin,
        {
            let args: std::marker::PhantomData<<Self::DB as sqlx::Database>::Arguments<'a>> =
                std::marker::PhantomData::default();
            Self::generic_fetch_one_full_plain(&mut *self.transaction, stmt, args).await
        }

        async fn fetch_option_full<'a, SE>(
            &'a mut self,
            stmt: &'a str,
            args: <Self::DB as sqlx::Database>::Arguments<'a>,
        ) -> crate::Result<Option<SE>>
        where
            SE: taitan_orm_trait::SelectedEntity<Self::DB> + Send + Unpin,
        {
            Self::generic_fetch_option_full(&mut *self.transaction, stmt, args).await
        }

        async fn fetch_option_full_plain<'a, SE>(
            &'a mut self,
            stmt: &'a str,
        ) -> crate::Result<Option<SE>>
        where
            SE: taitan_orm_trait::SelectedEntity<Self::DB> + Send + Unpin,
        {
            let args: std::marker::PhantomData<<Self::DB as sqlx::Database>::Arguments<'a>> =
                std::marker::PhantomData::default();
            Self::generic_fetch_option_full_plain(&mut *self.transaction, stmt, args).await
        }

        async fn fetch_all_full<'a, SE>(
            &'a mut self,
            stmt: &'a str,
            args: <Self::DB as sqlx::Database>::Arguments<'a>,
        ) -> crate::Result<Vec<SE>>
        where
            SE: taitan_orm_trait::SelectedEntity<Self::DB> + Send + Unpin,
        {
            Self::generic_fetch_all_full(&mut *self.transaction, stmt, args).await
        }

        async fn fetch_all_full_plain<'a, SE>(&'a mut self, stmt: &'a str) -> crate::Result<Vec<SE>>
        where
            SE: taitan_orm_trait::SelectedEntity<Self::DB> + Send + Unpin,
        {
            let args: std::marker::PhantomData<<Self::DB as sqlx::Database>::Arguments<'a>> =
                std::marker::PhantomData::default();
            Self::generic_fetch_all_full_plain(&mut *self.transaction, stmt, args).await
        }
    };
}
