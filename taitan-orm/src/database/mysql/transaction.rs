use sqlx::MySql;
use crate::{transaction_impl, CountResult,  SqlExecutorMut, SqlGeneratorContainer, SqlGenericExecutor};
use crate::sql_generator::MySqlGenerator;

#[derive(Debug)]
pub struct MySqlTransaction<'a> {
    transaction: sqlx::Transaction<'a, MySql>,
    generator: &'a MySqlGenerator,
}

impl<'a> MySqlTransaction<'a> {
    pub fn new(trx: sqlx::Transaction<'a, MySql>, generator: &'a MySqlGenerator) -> Self {
        Self {
            transaction: trx,
            generator,
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

impl<'t> SqlGenericExecutor for MySqlTransaction<'t> {
    type DB = MySql;
    type CountType = CountResult;

    fn get_affected_rows(query_result: &<Self::DB as sqlx::Database>::QueryResult) -> u64 {
        query_result.rows_affected()
    }
}

impl<'t> SqlExecutorMut for MySqlTransaction<'t> {
    transaction_impl!(MySqlConnection);
}
impl<'a> SqlGeneratorContainer for MySqlTransaction<'a> {
    type G = MySqlGenerator;

    fn get_generator(&self) -> &Self::G {
        &self.generator
    }
}