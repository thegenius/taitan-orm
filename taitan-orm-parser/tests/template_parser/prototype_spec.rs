use nom::branch::alt;
use nom::bytes::complete::{tag, tag_no_case, take_until};
use nom::character::complete::{multispace0, multispace1};
use nom::sequence::preceded;
use nom::IResult;
use rinja::Template;
use sqlx::mysql::MySqlArguments;
use sqlx::postgres::PgArguments;
use sqlx::{Arguments, Database, Postgres};
use taitan_orm_trait::brave_new::error::TemplateRenderError;
use taitan_orm_trait::brave_new::result::Result;
use taitan_orm_trait::brave_new::{Pagination, PlaceholderParser};

#[derive(Template)]
#[template(source = "{{ get_template_sql() }}", ext = "txt")]
pub struct Query<'a> {
    a: &'a str,
    b: Option<i64>,
    c: Option<Option<&'a str>>,
}



#[test]
pub fn test() {
    let query = Query {
        a: "a",
        b: Some(1),
        c: None,
    };
    let (sql, args) = query.get_rendered().unwrap();
    assert_eq!(sql, "SELECT * FROM users WHERE a=? AND b=? AND c=?");
    assert_eq!(args.len(), 2);
}
