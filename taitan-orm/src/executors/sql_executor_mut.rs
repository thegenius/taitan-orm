use sqlx::Database;
use taitan_orm_trait::result::Result;
use taitan_orm_trait::traits::Selected;

/**
现在sqlx::Executor的实现还是太过简陋，导致无法把不同数据库和事务的连接抽象成一个独立的实体
屏蔽掉ex，让更上层的API层不再感知connection/transaction，
这层抽象让上层实现API接口的实现可以把普通操作和事务操作合并为一份

本模块提供以下接口方法
execute           (stmt, args) -> Result<u64>
execute_plain     (stmt      ) -> Result<u64>

fetch_exists            (stmt, args) -> Result<bool>
fetch_exists_plain      (stmt,     ) -> Result<bool>

fetch_option      (stmt, selection, args) -> Result<Option<SE>>
fetch_option_plain(stmt, selection,     ) -> Result<Option<SE>>
-- fetch_one         (stmt, selection, args) -> Result<SE>
-- fetch_one_plain   (stmt, selection,     ) -> Result<SE>
fetch_all         (stmt, selection, args) -> Result<Vec<SE>>
fetch_all_plain   (stmt, selection,     ) -> Result<Vec<SE>>

fetch_all_full         (stmt, args) -> Result<Vec<SE>>
fetch_all_full_plain   (stmt,     ) -> Result<Vec<SE>>
fetch_one_full         (stmt, args) -> Result<SE>
fetch_one_full_plain   (stmt,     ) -> Result<SE>
fetch_option_full      (stmt, args) -> Result<Option<SE>>
fetch_option_full_plain(stmt,     ) -> Result<Option<SE>>
*/
pub trait SqlExecutorMut<DB>
where
    DB: Database,
    i64: sqlx::Type<DB>,
    for<'a> i64: sqlx::Encode<'a, DB>,
{
    // type Connection: Connection;
    //
    // async fn get_connection(&mut self) -> Result<PoolConnection<DB>> {
    //     Err(NotImplement("get_connection not implemented".to_string()))
    // }

    // execute           (stmt, args) -> Result<u64>
    async fn execute<'a>(
        &'a mut self,
        stmt: &'a str,
        args: DB::Arguments<'a>,
    ) -> Result<u64>;

    // execute_plain     (stmt, _   ) -> Result<u64>
    async fn execute_plain<'a>(&'a mut self, stmt: &'a str) -> Result<u64>;

    // fetch_exists            (stmt, args) -> Result<bool>
    async fn fetch_exists<'a>(
        &'a mut self,
        stmt: &'a str,
        args: DB::Arguments<'a>,
    ) -> Result<bool>;

    async fn fetch_exists_plain<'a, A>(&'a mut self, stmt: &'a str) -> Result<bool>;

    async fn fetch_count<'s, 'a>(
        &'a mut self,
        stmt: &'s str,
        args: DB::Arguments<'a>,
    ) -> Result<u64>
    where
        'a: 's;

    async fn fetch_count_plain<'a>(&'a mut self, stmt: &'a str) -> Result<u64>;

    // fetch_option      (stmt, selection, args) -> Result<Option<SE>>
    async fn fetch_option<'a, SE>(
        &'a mut self,
        stmt: &'a str,
        selection: &'a SE,
        args: DB::Arguments<'a>,
    ) -> Result<Option<SE>>
    where
        SE: Selected<DB> + Send + Unpin;

    async fn fetch_option_<'a, SE>(
        &'a mut self,
        stmt: &'a str,
        selection: &'a SE,
        args: DB::Arguments<'a>,
    ) -> Result<Option<SE>>
    where
        SE: Selected<DB> + Send + Unpin;

    // fetch_option_plain(stmt, selection) -> Result<Option<SE>>
    async fn fetch_option_plain<'a, SE>(
        &'a mut self,
        stmt: &'a str,
        selection: &'a SE,
    ) -> Result<Option<SE>>
    where
        SE: Selected<DB> + Send + Unpin;

    async fn fetch_option_plain_<'a, SE>(
        &'a mut self,
        stmt: &'a str,
        selection: &'a SE,
    ) -> Result<Option<SE>>
    where
        SE: Selected<DB> + Send + Unpin;

    // fetch_all         (stmt, selection, args) -> Result<Vec<SE>>
    async fn fetch_all<'a, SE>(
        &'a mut self,
        stmt: &'a str,
        selection: &'a SE,
        args: DB::Arguments<'a>,
    ) -> Result<Vec<SE>>
    where
        SE: Selected<DB> + Send + Unpin;

    async fn fetch_all_<'a, SE>(
        &'a mut self,
        stmt: &'a str,
        selection: &'a SE,
        args: DB::Arguments<'a>,
    ) -> Result<Vec<SE>>
    where
        SE: Selected<DB> + Send + Unpin;

    // fetch_all_plain   (stmt, selection) -> Result<Vec<SE>>
    async fn fetch_all_plain<'a, SE>(
        &'a mut self,
        stmt: &'a str,
        selection: &'a SE,
    ) -> Result<Vec<SE>>
    where
        SE: Selected<DB> + Send + Unpin;

    async fn fetch_all_plain_<'a, SE>(
        &'a mut self,
        stmt: &'a str,
        selection: &'a SE,
    ) -> Result<Vec<SE>>
    where
        SE: Selected<DB> + Send + Unpin;

    // fetch_one_full         (stmt, args) -> Result<SE>
    async fn fetch_one_full<'a, SE>(
        &'a mut self,
        stmt: &'a str,
        args: DB::Arguments<'a>,
    ) -> Result<SE>
    where
        SE: Selected<DB> + Send + Unpin;

    // fetch_one_full_plain   (stmt, _   ) -> Result<SE>
    async fn fetch_one_full_plain<'a, SE>(&'a mut self, stmt: &'a str) -> Result<SE>
    where
        SE: Selected<DB> + Send + Unpin;

    // fetch_option_full      (stmt, args) -> Result<Option<SE>>
    async fn fetch_option_full<'a, SE>(
        &'a mut self,
        stmt: &'a str,
        args: DB::Arguments<'a>,
    ) -> Result<Option<SE>>
    where
        SE: Selected<DB> + Send + Unpin;

    // fetch_option_full_plain(stmt) -> Result<Option<SE>>
    async fn fetch_option_full_plain<'a, SE>(&'a mut self, stmt: &'a str) -> Result<Option<SE>>
    where
        SE: Selected<DB> + Send + Unpin;

    // fetch_all_full         (stmt, args) -> Result<Vec<SE>>
    async fn fetch_all_full<'a, SE>(
        &'a mut self,
        stmt: &'a str,
        args: DB::Arguments<'a>,
    ) -> Result<Vec<SE>>
    where
        SE: Selected<DB> + Send + Unpin;

    // fetch_all_full_plain   (stmt) -> Result<Vec<SE>>
    async fn fetch_all_full_plain<'a, SE>(&'a mut self, stmt: &'a str) -> Result<Vec<SE>>
    where
        SE: Selected<DB> + Send + Unpin;

    // fetch_one         (stmt, selection, args) -> Result<SE>
    // fetch_one_plain   (stmt, selection, _   ) -> Result<SE>
}
