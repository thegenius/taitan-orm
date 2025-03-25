use sqlx::postgres::PgConnectOptions;
use sqlx::PgPool;
use super::PostgresDatabase;
pub struct PostgresBuilder {}

impl PostgresBuilder {
    pub async fn build(config: PgConnectOptions)-> crate::result::Result<PostgresDatabase> {
        let pool = PgPool::connect_with(config).await?;

        let database = PostgresDatabase {
            pool,
        };
        Ok(database)
    }
}
