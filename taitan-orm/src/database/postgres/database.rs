use sqlx::{MySqlPool, PgPool, Postgres};
use sqlx::mysql::MySqlConnectOptions;
use sqlx::postgres::PgConnectOptions;
use crate::sql_generator::{MySqlGenerator, PostgresGenerator};
use crate::{executor_impl, CountResult, SqlExecutor, SqlGeneratorContainer, SqlGenericExecutor};

#[derive(Debug, Clone)]
pub struct PostgresDatabase {
    generator: PostgresGenerator,
    pool: PgPool,
}
impl PostgresDatabase {
    pub async fn build(config: PgConnectOptions)-> crate::Result<PostgresDatabase> {
        let pool = PgPool::connect_with(config).await?;
        let generator = PostgresGenerator::new();
        let database = PostgresDatabase {
            generator,
            pool,
        };
        Ok(database)
    }
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


