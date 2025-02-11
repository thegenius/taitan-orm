
use sqlx::{MySqlPool};
use sqlx::mysql::MySqlConnectOptions;
use crate::brave_new::database::mysql::database::MySqlDatabase;


pub struct MySqlBuilder {}

impl MySqlBuilder {
    pub async fn build(config: MySqlConnectOptions)-> crate::result::Result<MySqlDatabase> {
        let pool = MySqlPool::connect_with(config).await?;

        let database = MySqlDatabase {
            pool,
        };
        Ok(database)
    }
}
