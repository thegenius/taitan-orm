use super::result::Result;
use crate::brave_new::pagination::Pagination;
use crate::brave_new::param::Parameter;
use crate::brave_new::{CountSqlParser, PlaceholderParser};
use crate::NotImplementError;
use sqlx::error::BoxDynError;
use sqlx::mysql::MySqlArguments;
use sqlx::postgres::PgArguments;
use sqlx::sqlite::SqliteArguments;
use sqlx::{Database, MySql, Postgres, Sqlite};
use std::borrow::Cow;
use std::fmt::Debug;
// 有3套SQL需要生成
// （1）普通SQL
// （2）count SQL
// （3）paged SQL
// 如果配合indexed，其实有6套

pub trait Template<DB: Database>: Debug {
    fn get_sql<'a>(&self) -> Result<(String, DB::Arguments<'a>)>;

    fn get_paged_sql<'a>(&self, pagination: &Pagination) -> Result<(String, DB::Arguments<'a>)>;
    fn get_count_sql<'a>(&self) -> Result<(String, DB::Arguments<'a>)>;
}

pub trait MysqlTemplate: Template<MySql> {}
impl<T: Template<MySql>> MysqlTemplate for T {}

pub trait PostgresTemplate: Template<Postgres> {}
impl<T: Template<Postgres>> PostgresTemplate for T {}

pub trait SqliteTemplate: Template<Sqlite> {}
impl<T: Template<Sqlite>> SqliteTemplate for T {}

pub struct DynamicRenderedSql {
    sql: String,
    variables: Vec<String>,
}

pub trait TemplateSqlTrait {
    fn get_template_sql(&self) -> String;
    fn get_rendered_sql(&self) -> Result<String>;
}

pub trait TemplateArgTrait<DB: Database> {
    fn add_to_args<'a, 'b>(
        &'a self,
        name: &'b str,
        args: &'b mut <DB as Database>::Arguments<'a>,
    ) -> Result<()>;
}

pub trait TemplateRenderTrait<DB: Database>: TemplateArgTrait<DB> + TemplateSqlTrait
where
    for<'a> i64: sqlx::Encode<'a, DB>,
    i64: sqlx::Type<DB>,
{
    fn gen_sql(&self) -> Result<(String, <DB as Database>::Arguments<'_>)> {
        let rendered = self.get_rendered_sql()?;
        let (sql, vars) = PlaceholderParser::parse(&rendered);
        let mut args = <DB as Database>::Arguments::default();
        for variable in &vars {
            self.add_to_args(variable, &mut args)?;
        }
        Ok((sql, args))
    }
    fn gen_count_sql(&self) -> Result<(String, <DB as Database>::Arguments<'_>)> {
        let rendered = self.get_rendered_sql()?;
        let replaced = CountSqlParser::replace(&rendered)?;
        let (sql, vars) = PlaceholderParser::parse(&replaced);
        let mut args = <DB as Database>::Arguments::default();
        for variable in &vars {
            self.add_to_args(variable, &mut args)?;
        }
        Ok((sql, args))
    }

    fn gen_paged_sql<'a>(
        &'a self,
        pagination: &'a Pagination,
    ) -> Result<(String, <DB as Database>::Arguments<'a>)> {
        let rendered = self.get_rendered_sql()?;
        let paged_sql = format!("{} {}", rendered, Pagination::gen_limit_sql());
        let (sql, vars) = PlaceholderParser::parse(&paged_sql);
        let mut args = <DB as Database>::Arguments::default();
        for variable in &vars {
            self.add_to_args(variable, &mut args)?;
        }
        <Pagination as Parameter<DB>>::add_to_args(pagination, &mut args)?;
        Ok((sql, args))
    }

    fn gen_indexed_sql(&self) -> Result<(String, <DB as Database>::Arguments<'_>)> {
        let rendered = self.get_rendered_sql()?;
        let (sql, vars) = PlaceholderParser::parse_indexed(&rendered);
        let mut args = <DB as Database>::Arguments::default();
        for variable in &vars {
            self.add_to_args(variable, &mut args)?;
        }
        Ok((sql, args))
    }
    fn gen_indexed_count_sql(&self) -> Result<(String, <DB as Database>::Arguments<'_>)> {
        let rendered = self.get_rendered_sql()?;
        let replaced = CountSqlParser::replace(&rendered)?;
        let (sql, vars) = PlaceholderParser::parse_indexed(&replaced);
        let mut args = <DB as Database>::Arguments::default();
        for variable in &vars {
            self.add_to_args(variable, &mut args)?;
        }
        Ok((sql, args))
    }

    fn gen_indexed_paged_sql<'a>(
        &'a self,
        pagination: &'a Pagination,
    ) -> Result<(String, <DB as Database>::Arguments<'a>)> {
        let rendered = self.get_rendered_sql()?;
        let (sql, vars) = PlaceholderParser::parse(&rendered);
        let paged_sql = format!("{} {}", sql, Pagination::gen_limit_sql_indexed(vars.len()));
        let mut args = <DB as Database>::Arguments::default();
        for variable in &vars {
            self.add_to_args(variable, &mut args)?;
        }
        <Pagination as Parameter<DB>>::add_to_args(pagination, &mut args)?;
        Ok((paged_sql, args))
    }
}

impl<T, DB> TemplateRenderTrait<DB> for T
where
    DB: sqlx::Database,
    T: TemplateArgTrait<DB> + TemplateSqlTrait,
    for<'a> i64: sqlx::Encode<'a, DB>,
    i64: sqlx::Type<DB>,
{
}
