use sqlx::mysql::MySqlArguments;
use sqlx::postgres::PgArguments;
use sqlx::{Arguments, Database, Postgres};
use taitan_orm_macro::Template;
use taitan_orm_macro::TemplateArg;
use taitan_orm_trait::brave_new::error::TemplateRenderError;
use taitan_orm_trait::brave_new::result::Result;
use taitan_orm_trait::brave_new::Pagination;
use taitan_orm_trait::brave_new::{TemplateArgTrait, TemplateRenderTrait, TemplateSqlTrait};
#[derive(askama::Template,  Template, Debug)]
#[template(
    source = "SELECT * FROM users WHERE a=:{a} AND b=:{b} {% if c.is_some() %} AND c=:{c} {% endif %}",
    ext = "txt"
)]
pub struct Query {
    a: String,
    b: Option<i64>,
    c: Option<Option<String>>,
}



#[test]
pub fn test() {
    let query = Query {
        a: "a".to_string(),
        b: Some(1),
        c: None,
    };
    let (sql, args) = taitan_orm_trait::brave_new::Template::<sqlx::Postgres>::get_sql(&query).unwrap();
    assert_eq!(sql, "SELECT * FROM users WHERE a=$1 AND b=$2 ");
    assert_eq!(args.len(), 2);
}
