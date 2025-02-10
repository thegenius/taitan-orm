use super::transaction::SqliteTransaction;

use sqlx::SqlitePool;


#[derive(Debug, Clone)]
pub struct SqliteDatabase {
    pub(crate) sqlite_pool: SqlitePool,
}

impl SqliteDatabase {
    pub async fn transaction<'a>(&'a self) -> crate::result::Result<SqliteTransaction<'a>> {
        let trx = self.get_pool()?.begin().await?;
        let transaction = SqliteTransaction::new(trx);
        Ok(transaction)
    }

    pub fn get_pool(&self) -> crate::result::Result<&SqlitePool> {
        Ok(&self.sqlite_pool)
    }
}
