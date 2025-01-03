use sqlx::{PgPool, Postgres};
use crate::sql_generator::PostgresGenerator;
use crate::{executor_impl, CountResult, SqlExecutor, SqlGeneratorContainer, SqlGenericExecutor};

#[derive(Debug, Clone)]
pub struct PostgresDatabase {
    generator: PostgresGenerator,
    pool: PgPool,
}
impl PostgresDatabase {
    pub fn get_pool(&self) -> crate::Result<&PgPool> {
        Ok(&self.pool)
    }
}
impl SqlGenericExecutor for PostgresDatabase {
    type DB = Postgres;
    type CountType = CountResult;

    fn get_affected_rows(query_result: &<Self::DB as sqlx::Database>::QueryResult) -> u64 {
        query_result.rows_affected()
    }

}
// SqlExecutor + SqlGeneratorContainer + Extractor
impl SqlGeneratorContainer for PostgresDatabase {
    type G = PostgresGenerator;

    fn get_generator(&self) -> &Self::G {
        &self.generator
    }
}

impl SqlExecutor for PostgresDatabase {
    executor_impl!(PgConnection);
}


