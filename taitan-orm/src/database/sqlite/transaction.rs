
use crate::database::sqlite::commanders::read::SqliteReadCommander;
use crate::database::sqlite::{SqliteWriteCommander};
use crate::sql_generator::DefaultSqlGenerator;
use crate::sql_generator_container::SqlGeneratorContainer;
use sqlx::{Sqlite, SqliteConnection};
use crate::database::sqlite::commanders::template::SqliteTemplateCommander;
use crate::sql_generic_executor::SqlGenericExecutor;
use crate::{transaction_impl, CountResult};

#[derive(Debug)]
pub struct SqliteTransaction<'a> {
    transaction: sqlx::Transaction<'a, Sqlite>,
    sql_generator: &'a DefaultSqlGenerator,
}

impl<'a> SqliteTransaction<'a> {
    pub fn new(trx: sqlx::Transaction<'a, Sqlite>, sql_generator: &'a DefaultSqlGenerator) -> Self {
        Self {
            transaction: trx,
            sql_generator,
        }
    }

    #[inline]
    pub async fn commit(self) -> crate::Result<()> {
        Ok(self.transaction.commit().await?)
    }

    #[inline]
    pub async fn rollback(self) -> crate::Result<()> {
        Ok(self.transaction.rollback().await?)
    }
}

impl<'t> SqlGenericExecutor for SqliteTransaction<'t> {
    type DB = Sqlite;
    type CountType = CountResult;

    fn get_affected_rows(query_result: &<Self::DB as sqlx::Database>::QueryResult) -> u64 {
        query_result.rows_affected()
    }
}

impl<'t> crate::SqlExecutor for SqliteTransaction<'t> {
    transaction_impl!(SqliteConnection);
}
impl<'a> SqlGeneratorContainer for SqliteTransaction<'a> {
    type G = DefaultSqlGenerator;

    fn get_generator(&mut self) -> &Self::G {
        &self.sql_generator
    }
}

impl<'a> SqliteWriteCommander for SqliteTransaction<'a> {}

impl<'a> SqliteReadCommander for SqliteTransaction<'a> {}

impl<'a> SqliteTemplateCommander for SqliteTransaction<'a> {}




//
// impl<'s> crate::SqlExecutor for SqliteTransaction<'s> {
//
//     type Connection = SqliteConnection;
//
//     async fn execute<'a>(&'a mut self, stmt: &'a str, args: <Self::DB as sqlx::Database>::Arguments<'a>) -> crate::Result<u64>
//     {
//         Self::generic_execute(&mut *self.transaction, stmt, args).await
//     }
//
//
//     async fn execute_plain<'a>(&'a mut self, stmt: &'a str) -> crate::Result<u64> {
//         let args: std::marker::PhantomData<<Self::DB as sqlx::Database>::Arguments<'a>> =
//             std::marker::PhantomData::default();
//         Self::generic_execute_plain(&mut *(self.transaction), stmt, args).await
//     }
//
//     async fn fetch_exists<'a>(&'a mut self, stmt: &'a str, args: <Self::DB as sqlx::Database>::Arguments<'a>) -> crate::Result<bool> {
//         Self::generic_exists(&mut *self.transaction, stmt, args).await
//     }
//
//
//     async fn fetch_exists_plain<'a, A>(&'a mut self, stmt: &'a str) -> crate::Result<bool> {
//         let args: std::marker::PhantomData<<Self::DB as sqlx::Database>::Arguments<'a>> =
//             std::marker::PhantomData::default();
//         Self::generic_exists_plain(&mut *self.transaction, stmt, args).await
//     }
//
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
//         Self::generic_fetch_option(&mut *self.transaction, stmt, selection, args).await
//     }
//
//     async fn fetch_option_plain<'a, SE>(
//         &'a mut self,
//         stmt: &'a str,
//         selection: &'a SE::Selection,
//     ) -> crate::Result<Option<SE>>
//     where
//         SE: taitan_orm_trait::SelectedEntity<Self::DB> + Send + Unpin
//     {
//         let args: std::marker::PhantomData<<Self::DB as sqlx::Database>::Arguments<'a>> =
//             std::marker::PhantomData::default();
//         Self::generic_fetch_option_plain(&mut *self.transaction, stmt, selection, args).await
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
//         Self::generic_fetch_all(&mut *self.transaction, stmt, selection, args).await
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
//         let args: std::marker::PhantomData<<Self::DB as sqlx::Database>::Arguments<'a>> =
//             std::marker::PhantomData::default();
//         Self::generic_fetch_all_plain(&mut *self.transaction, stmt, selection, args).await
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
//         Self::generic_fetch_one_full(&mut *self.transaction, stmt, args).await
//     }
//
//     async fn fetch_one_full_plain<'a, SE>(&'a mut self, stmt: &'a str) -> crate::Result<SE>
//     where
//         SE: taitan_orm_trait::SelectedEntity<Self::DB> + Send + Unpin,
//     {
//         let args: std::marker::PhantomData<<Self::DB as sqlx::Database>::Arguments<'a>> =
//             std::marker::PhantomData::default();
//         Self::generic_fetch_one_full_plain(&mut *self.transaction, stmt, args).await
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
//         Self::generic_fetch_option_full(&mut *self.transaction, stmt, args).await
//     }
//
//     async fn fetch_option_full_plain<'a, SE>(&'a mut self, stmt: &'a str) -> crate::Result<Option<SE>>
//     where
//         SE: taitan_orm_trait::SelectedEntity<Self::DB> + Send + Unpin,
//     {
//         let args: std::marker::PhantomData<<Self::DB as sqlx::Database>::Arguments<'a>> =
//             std::marker::PhantomData::default();
//         Self::generic_fetch_option_full_plain(&mut *self.transaction, stmt, args).await
//     }
//
//
//     async fn fetch_all_full<'a, SE>(&'a mut self, stmt: &'a str, args: <Self::DB as sqlx::Database>::Arguments<'a>) -> crate::Result<Vec<SE>>
//     where
//         SE: taitan_orm_trait::SelectedEntity<Self::DB> + Send + Unpin
//     {
//         Self::generic_fetch_all_full(&mut *self.transaction, stmt, args).await
//     }
//
//     async fn fetch_all_full_plain<'a, SE>(&'a mut self, stmt: &'a str) -> crate::Result<Vec<SE>>
//     where
//         SE: taitan_orm_trait::SelectedEntity<Self::DB> + Send + Unpin
//     {
//         let args: std::marker::PhantomData<<Self::DB as sqlx::Database>::Arguments<'a>> =
//             std::marker::PhantomData::default();
//         Self::generic_fetch_all_full_plain(&mut *self.transaction, stmt, args).await
//     }
// }


