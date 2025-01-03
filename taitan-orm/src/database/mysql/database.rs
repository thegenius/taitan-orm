use sqlx::{MySql, MySqlPool};
use crate::sql_generator::MySqlGenerator;
use crate::{executor_impl, CountResult, SqlExecutor, SqlGeneratorContainer, SqlGenericExecutor};

#[derive(Debug, Clone)]
pub struct MySqlDatabase {
    generator: MySqlGenerator,
    pool: MySqlPool,
}
impl MySqlDatabase {
    pub fn get_pool(&self) -> crate::Result<&MySqlPool> {
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
// SqlExecutor + SqlGeneratorContainer + Extractor
impl SqlGeneratorContainer for MySqlDatabase {
    type G = MySqlGenerator;

    fn get_generator(&self) -> &Self::G {
        &self.generator
    }
}

impl SqlExecutor for MySqlDatabase {
    executor_impl!(MySqlConnection);
}


