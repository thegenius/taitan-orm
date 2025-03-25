use crate::count::CountResult;
use crate::{brave_new_transaction_impl};
use crate::new_executor::SqlGenericExecutor;
use sqlx::MySql;
use taitan_orm_trait::result::Result;
use crate::new_executor::SqlExecutorMutNew;

#[derive(Debug)]
pub struct MySqlTransaction<'a> {
    transaction: sqlx::Transaction<'a, MySql>,
}

impl<'a> MySqlTransaction<'a> {
    pub fn new(trx: sqlx::Transaction<'a, MySql>) -> Self {
        Self { transaction: trx }
    }
    #[inline]
    pub async fn commit(self) -> Result<()> {
        Ok(self.transaction.commit().await?)
    }

    #[inline]
    pub async fn rollback(self) -> Result<()> {
        Ok(self.transaction.rollback().await?)
    }
}

// impl<'t> ArgsExtractor for MySqlTransaction<'t> {
//     fn extract_pagination_arguments(
//         page: &Pagination,
//     ) -> Result<<Self::DB as Database>::Arguments<'_>> {
//         Ok(<Pagination as Parameter<MySql>>::gen_args(page)?)
//     }
// }

impl<'t> SqlGenericExecutor for MySqlTransaction<'t> {
    type DB = MySql;
    type CountType = CountResult;

    fn get_affected_rows(query_result: &<Self::DB as sqlx::Database>::QueryResult) -> u64 {
        query_result.rows_affected()
    }
}

// impl<'t> SqlExecutorMut for MySqlTransaction<'t> {
//     new_transaction_impl! {}
// }

impl<'t> SqlExecutorMutNew<sqlx::MySql> for MySqlTransaction<'t> {
    brave_new_transaction_impl!(sqlx::MySql);
}