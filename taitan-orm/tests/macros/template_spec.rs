use sqlx::sqlx_macros;
use std::borrow::Cow;
use time::format_description::FormatItem::Optional;
use taitan_orm::prelude::*;
use taitan_orm_trait::pagination::Pagination;
// use taitan_orm_trait::TemplateRecord;

#[derive(TemplateRecord, Clone, Debug)]
#[sql = "select * from ${name}"]
pub struct TestTemplate1<'a> {
    name: Cow<'a, str>,
}

#[derive(TemplateRecord, Clone, Debug)]
#[sql = "select * from #{name}"]
pub struct TestTemplate2 {
    name: String,
}

#[derive(TemplateRecord, Clone, Debug)]
#[sql = "select * from ${name} #{age}"]
pub struct TestTemplate3<'a> {
    name: Cow<'a, str>,
    age: i32,
}

#[derive(TemplateRecord, Clone, Debug)]
#[sql = "select * from ${name} #{age} 'hello' "]
pub struct TestTemplate4<'a> {
    name: Cow<'a, str>,
    age: i32,
}

#[derive(TemplateRecord, Clone, Debug)]
#[sql = "select * from ${name} #{age} \"hello ${name}\" #{age} LIMIT #{page.offset} #{page.count}"]
#[count_sql = "select count(*) from ${name} #{age} \"hello ${name}\""]
pub struct TestTemplate5<'a> {
    name: Cow<'a, str>,

    age: i32,

    #[limit_field]
    page: Pagination,
}


#[derive(TemplateRecord, Clone, Debug)]
#[sql = "select * from `user` WHERE name = %{name} AND age = #{age} "]
pub struct TestTemplate6 {
    name: taitan_orm::result::Optional<String>,
    age: i32,
}

#[sqlx_macros::test]
pub async fn template_macro_spec() -> taitan_orm::result::Result<()> {
    let template = TestTemplate1 {
        name: Cow::Borrowed("wang"),
    };
    let sql = template.get_sql(None);
    assert_eq!(sql, "select * from wang");

    let template = TestTemplate2 {
        name: String::from("wang"),
    };
    let sql = template.get_sql(None);
    assert_eq!(sql, "select * from ?");

    let template = TestTemplate3 {
        name: Cow::Borrowed("wang"),
        age: 23,
    };
    let sql = template.get_sql(None);
    assert_eq!(sql, "select * from wang ?");

    let template = TestTemplate4 {
        name: Cow::Borrowed("wang"),
        age: 23,
    };
    let sql = template.get_sql(None);
    assert_eq!(sql, "select * from wang ? 'hello'");

    let template = TestTemplate5 {
        name: Cow::Borrowed("wang"),
        age: 23,
        page: Pagination::new(100, 200)
    };
    let sql = template.get_sql(None);
    assert_eq!(sql, "select * from wang ? \"hello ${name}\" ? LIMIT ? ?");

    let template = TestTemplate6 {
        name: taitan_orm::result::Optional::Some("wang".to_string()),
        age: 23,
    };
    let sql = template.get_sql(None);
    assert_eq!(sql, "select * from `user` WHERE name = ?  AND  age = ?");

    let template = TestTemplate6 {
        name: taitan_orm::result::Optional::None,
        age: 23,
    };
    let sql = template.get_sql(None);
    assert_eq!(sql, "select * from `user` WHERE   age = ?");

    let template = TestTemplate6 {
        name: taitan_orm::result::Optional::Null,
        age: 23,
    };


    let sql = template.get_sql(None);
    assert_eq!(sql, "select * from `user` WHERE name IS NULL  AND  age = ?");

    Ok(())
}
