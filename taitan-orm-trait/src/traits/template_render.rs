use crate::error::{TaitanOrmError, TemplateRenderError};
use crate::traits::Parameter;
use crate::result::Result;
use crate::parsers::{CountSqlParser,  PlaceholderParser};
use crate::page::Pagination;
use sqlx::Database;

pub struct DynamicRenderedSql {
    sql: String,
    variables: Vec<String>,
}

pub trait TemplateSqlTrait: askama::Template {
    fn get_rendered_sql(&self) -> Result<String> {
        Ok(self.render().map_err(|err| {
            TaitanOrmError::TemplateRenderError(TemplateRenderError(err.to_string()))
        })?)
    }
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
