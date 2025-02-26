use sqlx::Postgres;
use crate::{transaction_impl};
use crate::sql_generator::{PostgresGenerator};
use crate::prelude::{SqlExecutorMut, SqlGeneratorContainer, SqlGenericExecutor};
use crate::result::{CountResult};

#[derive(Debug)]
pub struct PostgresTransaction<'a> {
    transaction: sqlx::Transaction<'a, Postgres>,
    generator: &'a PostgresGenerator,
}

impl<'a> PostgresTransaction<'a> {
    pub fn new(trx: sqlx::Transaction<'a, Postgres>, generator: &'a PostgresGenerator) -> Self {
        Self {
            transaction: trx,
            generator,
        }
    }

    #[inline]
    pub async fn commit(self) -> crate::result::Result<()> {
        Ok(self.transaction.commit().await?)
    }

    #[inline]
    pub async fn rollback(self) -> crate::result::Result<()> {
        Ok(self.transaction.rollback().await?)
    }
}

impl<'t> SqlGenericExecutor for PostgresTransaction<'t> {
    type DB = Postgres;
    type CountType = CountResult;

    fn get_affected_rows(query_result: &<Self::DB as sqlx::Database>::QueryResult) -> u64 {
        query_result.rows_affected()
    }
}

impl<'t> SqlExecutorMut for PostgresTransaction<'t> {
    transaction_impl!(PgConnection);
}
impl<'a> SqlGeneratorContainer for PostgresTransaction<'a> {
    type G = PostgresGenerator;

    fn get_generator(&self) -> &Self::G {
        &self.generator
    }
}