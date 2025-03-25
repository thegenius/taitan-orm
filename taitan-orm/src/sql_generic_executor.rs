use sqlx::query::Query;
use sqlx::{Database, Executor, IntoArguments, Type};
use std::marker::PhantomData;

use taitan_orm_trait::error::TaitanOrmError;
use taitan_orm_trait::result::Result;
use taitan_orm_trait::traits::Selected;
/**
本模块提供2个维度的封装
1. 结果的转化，上层抽象不再感知Row和QueryResult的存在
QueryResult -> u64
Vec<Row>    -> Vec<SE>
Row         -> SE
Option<Row> -> Option<SE>
Option<Row> -> bool
2. sqlx::query::Query结构体的封装，上层抽象不再感知这个结构体的存在

本模块提供以下泛型接口函数


generic_execute           (ex, stmt, args) -> Result<u64>
generic_execute_plain     (ex, stmt, _   ) -> Result<u64>

generic_exists            (ex, stmt, args) -> Result<bool>
generic_exists_plain      (ex, stmt, _   ) -> Result<bool>
generic_count            (ex, stmt, args) -> Result<bool>
generic_count_plain      (ex, stmt, _   ) -> Result<bool>

generic_fetch_all         (ex, stmt, selection, args) -> Result<Vec<SE>>
generic_fetch_all_plain   (ex, stmt, selection, _   ) -> Result<Vec<SE>>
generic_fetch_one         (ex, stmt, selection, args) -> Result<SE>
generic_fetch_one_plain   (ex, stmt, selection, _   ) -> Result<SE>
generic_fetch_option      (ex, stmt, selection, args) -> Result<Option<SE>>
generic_fetch_option_plain(ex, stmt, selection, _   ) -> Result<Option<SE>>

generic_fetch_all_         (ex, stmt, se, args) -> Result<Vec<SE>>
generic_fetch_all_plain_   (ex, stmt, se, _   ) -> Result<Vec<SE>>
generic_fetch_one_         (ex, stmt, se, args) -> Result<SE>
generic_fetch_one_plain_   (ex, stmt, se, _   ) -> Result<SE>
generic_fetch_option_      (ex, stmt, se, args) -> Result<Option<SE>>
generic_fetch_option_plain_(ex, stmt, se, _   ) -> Result<Option<SE>>


generic_fetch_all_full         (ex, stmt, args) -> Result<Vec<SE>>
generic_fetch_all_full_plain   (ex, stmt, _   ) -> Result<Vec<SE>>
generic_fetch_one_full         (ex, stmt, args) -> Result<SE>
generic_fetch_one_full_plain   (ex, stmt, _   ) -> Result<SE>
generic_fetch_option_full      (ex, stmt, args) -> Result<Option<SE>>
generic_fetch_option_full_plain(ex, stmt, _   ) -> Result<Option<SE>>
**/

pub trait SqlGenericExecutor
where
    i64: Type<Self::DB>,
    for<'a> i64: sqlx::Encode<'a, Self::DB>
{
    type DB: Database;
    type CountType: Selected<Self::DB>;

    fn get_affected_rows(query_result: &<Self::DB as Database>::QueryResult) -> u64;

    // 1. generic_exists           (ex, stmt, args) -> Result<bool>
    async fn generic_exists<'a, EX, A>(ex: EX, stmt: &'a str, args: A) -> Result<bool>
    where
        EX: Executor<'a, Database = Self::DB>,
        A: IntoArguments<'a, Self::DB> + 'a,
    {
        let query: Query<'a, Self::DB, A> = sqlx::query_with(stmt, args);
        let result_opt: Option<<Self::DB as Database>::Row> = query.fetch_optional(ex).await?;
        if let Some(_) = result_opt {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    // 2. generic_exists           (ex, stmt, args) -> Result<bool>
    async fn generic_exists_plain<'a, EX, A>(
        ex: EX,
        stmt: &'a str,
        _args: PhantomData<A>,
    ) -> Result<bool>
    where
        EX: Executor<'a, Database = Self::DB>,
        A: IntoArguments<'a, Self::DB> + 'a + Default,
    {
        let query: Query<'a, Self::DB, A> = sqlx::query_with(stmt, Default::default());
        let result_opt: Option<<Self::DB as Database>::Row> = query.fetch_optional(ex).await?;
        if let Some(_) = result_opt {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    // generic_count            (ex, stmt, args) -> Result<bool>
    async fn generic_count<'s, 'a, EX, A>(ex: EX, stmt: &'s str, args: A) -> Result<Self::CountType>
    where
        's: 'a,
        EX: Executor<'a, Database = Self::DB>,
        A: IntoArguments<'a, Self::DB> + 'a,
    {
        let query: Query<'a, Self::DB, A> = sqlx::query_with(stmt, args);
        let result_opt: Option<<Self::DB as Database>::Row> = query.fetch_optional(ex).await?;
        if let Some(row) = result_opt {
            Ok(Self::CountType::from_row(&Self::CountType::default(), row)?)
        } else {
            Ok(Default::default())
        }
    }

    // generic_count_plain      (ex, stmt, _   ) -> Result<bool>
    async fn generic_count_plain<'a, EX, A>(
        ex: EX,
        stmt: &'a str,
        _args: PhantomData<A>,
    ) -> Result<Self::CountType>
    where
        EX: Executor<'a, Database = Self::DB>,
        A: IntoArguments<'a, Self::DB> + 'a + Default,
    {
        let query: Query<'a, Self::DB, A> = sqlx::query_with(stmt, Default::default());
        let result_opt: Option<<Self::DB as Database>::Row> = query.fetch_optional(ex).await?;
        if let Some(row) = result_opt {
            Ok(Self::CountType::from_row_full(row)?)
        } else {
            Ok(Default::default())
        }
    }

    // 3. generic_execute           (ex, stmt, args) -> Result<u64>
    async fn generic_execute<'a, 'e, EX, A>(ex: EX, query: &'a str, args: A) -> Result<u64>
    where
        EX: Executor<'e, Database = Self::DB>,
        A: IntoArguments<'a, Self::DB> + 'a,
    {
        let query: Query<'a, Self::DB, A> = sqlx::query_with(query, args);
        let result: <Self::DB as Database>::QueryResult = query.execute(ex).await?;
        Ok(Self::get_affected_rows(&result))
    }

    // 4. generic_execute_plain     (ex, stmt, _   ) -> Result<u64>
    async fn generic_execute_plain<'a, EX, A>(
        ex: EX,
        query: &'a str,
        _args: PhantomData<A>,
    ) -> Result<u64>
    where
        EX: Executor<'a, Database = Self::DB>,
        A: IntoArguments<'a, Self::DB> + 'a + Default,
    {
        let query: Query<'a, Self::DB, A> = sqlx::query_with(query, Default::default());
        let result: <Self::DB as Database>::QueryResult = query.execute(ex).await?;
        Ok(Self::get_affected_rows(&result))
    }

    // 5. generic_fetch_all         (ex, stmt, selection, args) -> Result<Vec<SE>>
    async fn generic_fetch_all<'a, EX, SE, A>(
        ex: EX,
        stmt: &'a str,
        selection: &SE,
        args: A,
    ) -> Result<Vec<SE>>
    where
        EX: Executor<'a, Database = Self::DB>,
        SE: Selected<Self::DB> + Send + Unpin,
        A: IntoArguments<'a, Self::DB> + 'a + Default,
    {
        let query: Query<'a, Self::DB, A> = sqlx::query_with(stmt, args);
        let result_opt: Vec<<Self::DB as Database>::Row> = query.fetch_all(ex).await?;
        let mut result: Vec<SE> = Vec::new();
        for row in result_opt {
            let selected_result = SE::from_row(selection, row);
            if let Ok(selected_entity) = selected_result {
                result.push(selected_entity);
            } else {
                return Err(TaitanOrmError::DecodeError("".to_string()));
            }
        }
        Ok(result)
    }

    async fn generic_fetch_all_<'a, EX, SE, A>(
        ex: EX,
        stmt: &'a str,
        selection: &SE,
        args: A,
    ) -> Result<Vec<SE>>
    where
        EX: Executor<'a, Database = Self::DB>,
        SE: Selected<Self::DB> + Send + Unpin,
        A: IntoArguments<'a, Self::DB> + 'a + Default,
    {
        let query: Query<'a, Self::DB, A> = sqlx::query_with(stmt, args);
        let result_opt: Vec<<Self::DB as Database>::Row> = query.fetch_all(ex).await?;
        let mut result: Vec<SE> = Vec::new();
        for row in result_opt {
            let selected_result = SE::from_row(selection, row);
            if let Ok(selected_entity) = selected_result {
                result.push(selected_entity);
            } else {
                return Err(TaitanOrmError::DecodeError("".to_string()));
            }
        }
        Ok(result)
    }

    // 6. generic_fetch_all_plain   (ex, stmt, selection, _   ) -> Result<Vec<SE>>
    async fn generic_fetch_all_plain<'a, EX, SE, A>(
        ex: EX,
        stmt: &'a str,
        selection: &SE,
        _: PhantomData<A>,
    ) -> Result<Vec<SE>>
    where
        EX: Executor<'a, Database = Self::DB>,
        SE: Selected<Self::DB> + Send + Unpin,
        A: IntoArguments<'a, Self::DB> + 'a + Default,
    {
        let query: Query<'a, Self::DB, A> = sqlx::query_with(stmt, Default::default());
        let result_opt: Vec<<Self::DB as Database>::Row> = query.fetch_all(ex).await?;
        let mut result: Vec<SE> = Vec::new();
        for row in result_opt {
            let selected_result = SE::from_row(selection, row);
            if let Ok(selected_entity) = selected_result {
                result.push(selected_entity);
            } else {
                return Err(TaitanOrmError::DecodeError("".to_string()));
            }
        }
        Ok(result)
    }

    async fn generic_fetch_all_plain_<'a, EX, SE, A>(
        ex: EX,
        stmt: &'a str,
        selection: &SE,
        _: PhantomData<A>,
    ) -> Result<Vec<SE>>
    where
        EX: Executor<'a, Database = Self::DB>,
        SE: Selected<Self::DB> + Send + Unpin,
        A: IntoArguments<'a, Self::DB> + 'a + Default,
    {
        let query: Query<'a, Self::DB, A> = sqlx::query_with(stmt, Default::default());
        let result_opt: Vec<<Self::DB as Database>::Row> = query.fetch_all(ex).await?;
        let mut result: Vec<SE> = Vec::new();
        for row in result_opt {
            let selected_result = SE::from_row(selection, row);
            if let Ok(selected_entity) = selected_result {
                result.push(selected_entity);
            } else {
                return Err(TaitanOrmError::DecodeError("".to_string()));
            }
        }
        Ok(result)
    }

    // 7. generic_fetch_one         (ex, stmt, selection, args) -> Result<SE>
    async fn generic_fetch_one<'a, EX, SE, A>(
        ex: EX,
        stmt: &'a str,
        selection: &SE,
        args: A,
    ) -> Result<SE>
    where
        EX: Executor<'a, Database = Self::DB>,
        SE: Selected<Self::DB> + Send + Unpin,
        A: IntoArguments<'a, Self::DB> + 'a,
    {
        let query: Query<'a, Self::DB, A> = sqlx::query_with(stmt, args);
        let result: <Self::DB as Database>::Row = query.fetch_one(ex).await?;
        Ok(SE::from_row(selection, result)?)
    }

    async fn generic_fetch_one_<'a, EX, SE, A>(
        ex: EX,
        stmt: &'a str,
        selection: &SE,
        args: A,
    ) -> Result<SE>
    where
        EX: Executor<'a, Database = Self::DB>,
        SE: Selected<Self::DB> + Send + Unpin,
        A: IntoArguments<'a, Self::DB> + 'a,
    {
        let query: Query<'a, Self::DB, A> = sqlx::query_with(stmt, args);
        let result: <Self::DB as Database>::Row = query.fetch_one(ex).await?;
        Ok(SE::from_row(selection, result)?)
    }

    // 8. generic_fetch_one_plain   (ex, stmt, selection, _   ) -> Result<SE>
    async fn generic_fetch_one_plain<'a, EX, SE, A>(
        ex: EX,
        stmt: &'a str,
        selection: &SE,
        _: PhantomData<A>,
    ) -> Result<SE>
    where
        EX: Executor<'a, Database = Self::DB>,
        SE: Selected<Self::DB> + Send + Unpin,
        A: IntoArguments<'a, Self::DB> + 'a + Default,
    {
        let query: Query<'a, Self::DB, A> = sqlx::query_with(stmt, Default::default());
        let result: <Self::DB as Database>::Row = query.fetch_one(ex).await?;
        Ok(SE::from_row(selection, result)?)
    }

    async fn generic_fetch_one_plain_<'a, EX, SE, A>(
        ex: EX,
        stmt: &'a str,
        selection: &SE,
        _: PhantomData<A>,
    ) -> Result<SE>
    where
        EX: Executor<'a, Database = Self::DB>,
        SE: Selected<Self::DB> + Send + Unpin,
        A: IntoArguments<'a, Self::DB> + 'a + Default,
    {
        let query: Query<'a, Self::DB, A> = sqlx::query_with(stmt, Default::default());
        let result: <Self::DB as Database>::Row = query.fetch_one(ex).await?;
        Ok(SE::from_row(selection, result)?)
    }

    // 9. generic_fetch_option      (ex, stmt, selection, args) -> Result<Option<SE>>
    async fn generic_fetch_option<'a, EX, SE, A>(
        ex: EX,
        stmt: &'a str,
        selection: &SE,
        args: A,
    ) -> Result<Option<SE>>
    where
        EX: Executor<'a, Database = Self::DB>,
        SE: Selected<Self::DB> + Send + Unpin,
        A: IntoArguments<'a, Self::DB> + 'a + Default,
    {
        let query: Query<'a, Self::DB, A> = sqlx::query_with(stmt, args);
        let result_opt: Option<<Self::DB as Database>::Row> = query.fetch_optional(ex).await?;
        if let Some(result) = result_opt {
            Ok(Some(SE::from_row(selection, result)?))
        } else {
            Ok(None)
        }
    }

    async fn generic_fetch_option_<'a, EX, SE, A>(
        ex: EX,
        stmt: &'a str,
        selection: &SE,
        args: A,
    ) -> Result<Option<SE>>
    where
        EX: Executor<'a, Database = Self::DB>,
        SE: Selected<Self::DB> + Send + Unpin,
        A: IntoArguments<'a, Self::DB> + 'a + Default,
    {
        let query: Query<'a, Self::DB, A> = sqlx::query_with(stmt, args);
        let result_opt: Option<<Self::DB as Database>::Row> = query.fetch_optional(ex).await?;
        if let Some(result) = result_opt {
            Ok(Some(SE::from_row(selection, result)?))
        } else {
            Ok(None)
        }
    }

    // 10. generic_fetch_option_plain(ex, stmt, selection, _   ) -> Result<Option<SE>>
    async fn generic_fetch_option_plain<'a, EX, SE, A>(
        ex: EX,
        stmt: &'a str,
        selection: &SE,
        _: PhantomData<A>,
    ) -> Result<Option<SE>>
    where
        EX: Executor<'a, Database = Self::DB>,
        SE: Selected<Self::DB> + Send + Unpin,
        A: IntoArguments<'a, Self::DB> + 'a + Default,
    {
        let query: Query<'a, Self::DB, A> = sqlx::query_with(stmt, Default::default());
        let result_opt: Option<<Self::DB as Database>::Row> = query.fetch_optional(ex).await?;
        if let Some(result) = result_opt {
            Ok(Some(SE::from_row(selection, result)?))
        } else {
            Ok(None)
        }
    }

    async fn generic_fetch_option_plain_<'a, EX, SE, A>(
        ex: EX,
        stmt: &'a str,
        selection: &SE,
        _: PhantomData<A>,
    ) -> Result<Option<SE>>
    where
        EX: Executor<'a, Database = Self::DB>,
        SE: Selected<Self::DB> + Send + Unpin,
        A: IntoArguments<'a, Self::DB> + 'a + Default,
    {
        let query: Query<'a, Self::DB, A> = sqlx::query_with(stmt, Default::default());
        let result_opt: Option<<Self::DB as Database>::Row> = query.fetch_optional(ex).await?;
        if let Some(result) = result_opt {
            Ok(Some(SE::from_row(selection, result)?))
        } else {
            Ok(None)
        }
    }

    // 11.  generic_fetch_all_full         (ex, stmt, args) -> Result<Vec<SE>>
    async fn generic_fetch_all_full<'a, EX, SE, A>(
        ex: EX,
        stmt: &'a str,
        args: A,
    ) -> Result<Vec<SE>>
    where
        EX: Executor<'a, Database = Self::DB>,
        SE: Selected<Self::DB> + Send + Unpin,
        A: IntoArguments<'a, Self::DB> + 'a,
    {
        let query: Query<'a, Self::DB, A> = sqlx::query_with(stmt, args);
        let result_vec: Vec<<Self::DB as Database>::Row> = query.fetch_all(ex).await?;
        let mut result: Vec<SE> = Vec::new();
        for row in result_vec {
            result.push(SE::from_row(&SE::default(), row)?);
        }
        Ok(result)
    }

    // 12. generic_fetch_all_full_plain   (ex, stmt, _   ) -> Result<Vec<SE>>
    async fn generic_fetch_all_full_plain<'a, EX, SE, A>(
        ex: EX,
        stmt: &'a str,
        _: PhantomData<A>,
    ) -> Result<Vec<SE>>
    where
        EX: Executor<'a, Database = Self::DB>,
        SE: Selected<Self::DB> + Send + Unpin,
        A: IntoArguments<'a, Self::DB> + 'a + Default,
    {
        let query: Query<'a, Self::DB, A> = sqlx::query_with(stmt, Default::default());
        let result_vec: Vec<<Self::DB as Database>::Row> = query.fetch_all(ex).await?;
        let mut result: Vec<SE> = Vec::new();
        for row in result_vec {
            result.push(SE::from_row_full(row)?);
        }
        Ok(result)
    }

    // 13. generic_fetch_one_full         (ex, stmt, args) -> Result<SE>
    async fn generic_fetch_one_full<'a, EX, SE, A>(ex: EX, stmt: &'a str, args: A) -> Result<SE>
    where
        EX: Executor<'a, Database = Self::DB>,
        SE: Selected<Self::DB> + Send + Unpin,
        A: IntoArguments<'a, Self::DB> + 'a,
    {
        let query: Query<'a, Self::DB, A> = sqlx::query_with(stmt, args);
        let result_opt: Option<<Self::DB as Database>::Row> = query.fetch_optional(ex).await?;
        if let Some(result) = result_opt {
            Ok(SE::from_row_full(result)?)
        } else {
            Err(sqlx::error::Error::RowNotFound.into())
        }
    }

    // 14. generic_fetch_one_full_plain   (ex, stmt, _   ) -> Result<SE>
    async fn generic_fetch_one_full_plain<'a, EX, SE, A>(
        ex: EX,
        stmt: &'a str,
        _: PhantomData<A>,
    ) -> Result<SE>
    where
        EX: Executor<'a, Database = Self::DB>,
        SE: Selected<Self::DB> + Send + Unpin,
        A: IntoArguments<'a, Self::DB> + 'a + Default,
    {
        let query: Query<'a, Self::DB, A> = sqlx::query_with(stmt, Default::default());
        let result_opt: Option<<Self::DB as Database>::Row> = query.fetch_optional(ex).await?;
        if let Some(result) = result_opt {
            Ok(SE::from_row_full(result)?)
        } else {
            Err(sqlx::error::Error::RowNotFound.into())
        }
    }

    // 15. generic_fetch_option_full      (ex, stmt, args) -> Result<Option<SE>>
    async fn generic_fetch_option_full<'a, EX, SE, A>(
        ex: EX,
        stmt: &'a str,
        args: A,
    ) -> Result<Option<SE>>
    where
        EX: Executor<'a, Database = Self::DB>,
        SE: Selected<Self::DB> + Send + Unpin,
        A: IntoArguments<'a, Self::DB> + 'a,
    {
        let query: Query<'a, Self::DB, A> = sqlx::query_with(stmt, args);
        let result_opt: Option<<Self::DB as Database>::Row> = query.fetch_optional(ex).await?;
        if let Some(result) = result_opt {
            Ok(Some(SE::from_row_full(result)?))
        } else {
            Ok(None)
        }
    }

    // 16. generic_fetch_option_full_plain(ex, stmt, _   ) -> Result<Option<SE>>
    async fn generic_fetch_option_full_plain<'a, EX, SE, A>(
        ex: EX,
        stmt: &'a str,
        _: PhantomData<A>,
    ) -> Result<Option<SE>>
    where
        EX: Executor<'a, Database = Self::DB>,
        SE: Selected<Self::DB> + Send + Unpin,
        A: IntoArguments<'a, Self::DB> + 'a + Default,
    {
        let query: Query<'a, Self::DB, A> = sqlx::query_with(stmt, Default::default());
        let result_opt: Option<<Self::DB as Database>::Row> = query.fetch_optional(ex).await?;
        if let Some(result) = result_opt {
            Ok(Some(SE::from_row(&SE::default(), result)?))
        } else {
            Ok(None)
        }
    }
}
