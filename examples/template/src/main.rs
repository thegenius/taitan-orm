use std::borrow::Cow;
use taitan_orm::database::sqlite::{SqliteBuilder, SqliteDatabase, SqliteLocalConfig};

use taitan_orm::prelude::*;

#[derive(Schema, Clone, Debug)]
#[table(user)]
#[primary(id)]
pub struct User {
    id: i32,
    name: String,
    age: Option<i32>,
}

#[derive(Template, askama::Template, Debug)]
#[template(
    source = "UPDATE `user` SET name = :{name} WHERE `id` = :{id}",
    ext = "txt"
)]
pub struct UserUpdateTemplate {
    id: i32,
    name: String,
}

#[derive(Template, askama::Template, Debug)]
#[template(
    source = "select `id`, `name`, `age` FROM `user` where `id` >= :{id}",
    ext = "txt"
)]
pub struct UserSelectTemplate {
    id: i32,
}

#[derive(Template, askama::Template, Debug)]
#[template(
    source = "select `id`, `name`, `age` FROM `user` where {% if age.is_some() %} age >= :{age} AND {% endif %} `name` = :{name}",
    ext = "txt"
)]
pub struct UserCustomTemplate {
    name: String,
    age: Option<i32>,
}

#[tokio::main]
async fn main() -> taitan_orm::result::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();

    // 0. prepare sqlite database
    let config = SqliteLocalConfig {
        work_dir: Cow::from("./workspace"),
        db_file: Cow::from("test.db"),
    };
    let mut db: SqliteDatabase = SqliteBuilder::build(config).await?;
    db.execute_plain("DROP TABLE IF EXISTS `user`").await?;
    db.execute_plain(
        "CREATE TABLE IF NOT EXISTS `user`(`id` INT PRIMARY KEY, `age` INT, `name` VARCHAR(64))",
    )
    .await?;

    // 1. insert entity
    let entity = User {
        id: 1,
        name: "Allen".to_string(),
        age: Option::Some(23),
    };
    let result = db.insert(&entity).await?;

    // 2. update by template
    let template = UserUpdateTemplate {
        id: 1,
        name: "Bob".to_string(),
    };
    let affected_rows = db.execute_by_template(&template).await?;
    assert_eq!(affected_rows, 1);
    let entity: Option<UserSelected> = db
        .select(&UserSelected::default(), &UserPrimary { id: 1 })
        .await?;
    assert!(entity.is_some());
    assert_eq!(entity.unwrap().name, Some(Some("Bob".to_string())));

    // 3. select by template


    let template = UserSelectTemplate { id: 1 };
    let user: Option<UserSelected> = db.fetch_option_by_template(&template).await?;
    assert!(user.is_some());

    let page: Pagination = Pagination::new(10, 0);
    let paged_users: PagedList<sqlx::Sqlite, UserSelected> =
        db.fetch_paged_by_template(&template, &page).await?;
    assert_eq!(paged_users.page.page_size, 10);
    assert_eq!(paged_users.page.total, 1);
    let user = paged_users.data.first().unwrap();
    assert_eq!(user.name.clone().unwrap(), Some("Bob".to_string()));

    // This template will execute sql: select `id`, `name`, `age` FROM `user` where `name` = ?
    let template = UserCustomTemplate {
        name: "Bob".to_string(),
        age: None,
    };
    let users: Vec<UserSelected> = db.fetch_all_by_template(&template).await?;
    assert_eq!(users.len(), 1);

    // This template will execute sql: select `id`, `name`, `age` FROM `user` where `age` > ? AND `name` = ?
    let template = UserCustomTemplate {
        name: "Bob".to_string(),
        age: Some(25),
    };
    let users: Vec<UserSelected> = db.fetch_all_by_template(&template).await?;
    assert_eq!(users.len(), 0);

    println!("template success!");
    Ok(())
}
