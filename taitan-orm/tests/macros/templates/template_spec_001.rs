use askama::Template;
use sqlx::{Arguments, Database, Postgres};
use taitan_orm::result::Result;
use taitan_orm::page::{Pagination};
use taitan_orm::traits::{TemplateRenderTrait, TemplateSqlTrait, TemplateArgTrait};

#[derive(Template, Debug)]
#[template(source = "SELECT * FROM users WHERE a=:{a} AND b=:{b} AND c=:{c}", ext = "txt")]
pub struct Query<'a> {
    a: &'a str,
    b: Option<i64>,
    c: Option<Option<&'a str>>,
}

impl<'a> TemplateSqlTrait for Query<'a> {}

impl<'a> TemplateArgTrait<Postgres> for Query<'a> {
    fn add_to_args<'c, 'd>(&'c self, name: &'d str, args: &'d mut <Postgres as Database>::Arguments<'d>) -> Result<()> {
        match name {
            "a" => args.add(&self.a)?,
            "b" => args.add(&self.b)?,
            "c" => if let Some(c) = &self.c {
                args.add(c)?
            },
            _ => unreachable!(),
        }
        Ok(())
    }
}

impl<'t> taitan_orm_trait::traits::Template<Postgres> for Query<'t> {
    fn get_sql(&self) -> Result<(String, <Postgres as Database>::Arguments<'_>)> {
        TemplateRenderTrait::gen_indexed_sql(self)
    }

    fn get_paged_sql(&self, pagination: &Pagination) -> Result<(String, <Postgres as Database>::Arguments<'_>)> {
        TemplateRenderTrait::gen_indexed_paged_sql(self, pagination)
    }

    fn get_count_sql(&self) -> Result<(String, <Postgres as Database>::Arguments<'_>)> {
        TemplateRenderTrait::gen_indexed_count_sql(self)
    }
}


#[test]
pub fn test() {
    let query = Query {
        a: "a",
        b: Some(1),
        c: None,
    };
    let (sql, args) = taitan_orm::traits::Template::get_sql(&query).unwrap();
    assert_eq!(sql, "SELECT * FROM users WHERE a=$1 AND b=$2 AND c=$3");
    assert_eq!(args.len(), 2);
}
