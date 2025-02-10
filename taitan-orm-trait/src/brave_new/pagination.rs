use sqlx::Arguments;
use sqlx::error::BoxDynError;
use sqlx::mysql::MySqlArguments;
use sqlx::postgres::PgArguments;
use sqlx::sqlite::SqliteArguments;
use crate::NotImplementError;

#[derive(Clone, Debug, Default)]
pub struct Pagination {
    pub page_size: u64,
    pub page_num: u64,
    pub offset: i64, // sqlx pg and mysql does not support u64, use i64 instead
    pub count: i64,  // sqlx pg and mysql does not support u64, use i64 instead
}

impl Pagination {

    pub fn new(page_size: u64, page_num: u64) -> Self {
        Self {
            page_size,
            page_num,
            offset: (page_size * page_num) as i64,
            count: page_size as i64,
        }
    }

    pub fn gen_limit_sql(&self) -> String {
        format!("LIMIT {}, {}", self.offset, self.count)
    }

    // #[inline(always)]
    // pub fn get_offset(&self) -> u64 {
    //     self.page_num * self.page_size
    // }
    //
    // #[inline(always)]
    // pub fn get_count(&self) -> u64 {
    //     self.page_size
    // }

    pub fn gen_page_arguments_sqlite(&self) -> Result<SqliteArguments<'_>, BoxDynError> {
        let offset: i64 = self.offset as i64;
        let count: i64 = self.count as i64;
        let mut arguments = SqliteArguments::default();
        arguments.add(offset)?;
        arguments.add(count)?;
        Ok(arguments)
    }
    pub fn gen_page_arguments_mysql(&self) -> Result<MySqlArguments, BoxDynError> {
        let offset = self.offset;
        let count = self.count;
        let mut arguments = MySqlArguments::default();
        arguments.add(offset)?;
        arguments.add(count)?;
        Ok(arguments)
    }
    pub fn gen_page_arguments_postgres(&self) -> Result<PgArguments, BoxDynError> {
        let offset: i64 = self.offset as i64;
        let count: i64 = self.count as i64;
        let mut arguments = PgArguments::default();
        arguments.add(offset)?;
        arguments.add(count)?;
        Ok(arguments)
    }
}