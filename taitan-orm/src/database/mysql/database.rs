use super::transaction::MySqlTransaction;

use sqlx::{MySql, MySqlPool};
use taitan_orm_trait::result::Result;
use crate::count::CountResult;
use crate::executors::SqlExecutor;
use crate::executors::SqlGenericExecutor;
use crate::{brave_new_executor_impl};
#[derive(Debug, Clone)]
pub struct MySqlDatabase {
    pub(crate) pool: MySqlPool,
}

impl MySqlDatabase {
    pub async fn transaction<'a>(&'a self) -> Result<MySqlTransaction<'a>> {
        let trx = self.get_pool()?.begin().await?;
        let transaction = MySqlTransaction::new(trx);
        Ok(transaction)
    }

    pub fn get_pool(&self) -> Result<&MySqlPool> {
        Ok(&self.pool)
    }
}

impl SqlGenericExecutor for MySqlDatabase {
    type DB = MySql;
    type CountType = CountResult;

    fn get_affected_rows(query_result: &<Self::DB as sqlx::Database>::QueryResult) -> u64 {
        query_result.rows_affected()
    }
}


impl SqlExecutor<MySql> for MySqlDatabase {
    brave_new_executor_impl!(sqlx::MySql);
}