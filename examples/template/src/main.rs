use std::borrow::Cow;
use taitan_orm::database::sqlite::{SqliteDatabase, SqliteLocalConfig};

use taitan_orm::prelude::*;

#[derive(Schema, Clone, Debug)]
#[table_name = "user"]
pub struct User {
    #[primary_key]
    id: i32,
    name: String,
    age: Optional<i32>,
}

#[derive(TemplateRecord, Debug)]
#[sql = "UPDATE `user` SET name = #{name} WHERE `id` = #{id}"]
pub struct UserUpdateTemplate {
    id: i32,
    name: String,
}

#[derive(TemplateRecord, Debug)]
#[sql = "select `id`, `name`, `age` FROM `user` where `id` >= #{id}"]
#[count_sql = "select count(*) FROM `user` where `id` >= #{id}"]
pub struct UserSelectTemplate {
    id: i32,

    #[limit_field]
    page: Pagination,
}


#[derive(TemplateRecord, Debug)]
#[sql = "select `id`, `name`, `age` FROM `user` where age >= %{age} AND `name` = #{name}"]
pub struct UserCustomTemplate {
    name: String,
    age: Optional<i32>,
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
    let mut db: SqliteDatabase = SqliteDatabase::build(config).await?;
    db.execute_plain("DROP TABLE IF EXISTS `user`").await?;
    db.execute_plain(
        "CREATE TABLE IF NOT EXISTS `user`(`id` INT PRIMARY KEY, `age` INT, `name` VARCHAR(64))",
    )
    .await?;

    // 1. insert entity
    let entity = User {
        id: 1,
        name: "Allen".to_string(),
        age: Optional::Some(23),
    };
    let result = db.insert(&entity).await?;
    assert_eq!(result, true);

    // 2. update by template
    let template = UserUpdateTemplate {
        id: 1,
        name: "Bob".to_string(),
    };
    let affected_rows = db.execute_by_template(&template).await?;
    assert_eq!(affected_rows, 1);
    let entity: Option<UserSelectedEntity> = db
        .select(&UserSelectedEntity::full_fields(), &UserPrimary { id: 1 })
        .await?;
    assert!(entity.is_some());
    assert_eq!(entity.unwrap().name.unwrap(), "Bob".to_string());

    // 3. select by template
    let template = UserSelectTemplate {
        id: 1,
        page: Default::default(),
    };
    let user: Option<UserSelectedEntity> = db.fetch_option_by_template(&template).await?;
    assert!(user.is_some());

    let template = UserSelectTemplate {
        id: 1,
        page: Pagination::new(10, 0),
    };
    let paged_users: PagedList<sqlx::Sqlite, UserSelectedEntity> = db.fetch_paged_by_template(&template).await?;
    assert_eq!(paged_users.page.page_size, 10);
    assert_eq!(paged_users.page.total, 1);
    let user = paged_users.data.first().unwrap();
    assert_eq!(user.name.clone().unwrap(), "Bob".to_string());

    // This template will execute sql: select `id`, `name`, `age` FROM `user` where `name` = #{name}
    let template = UserCustomTemplate {
        name: "Bob".to_string(),
        age: Optional::None,
    };
    let users: Vec<UserSelectedEntity> = db.fetch_all_by_template(&template).await?;
    assert_eq!(users.len(), 1);

    // This template will execute sql: select `id`, `name`, `age` FROM `user` where `age` > #{age} `name` = #{name}
    let template = UserCustomTemplate {
        name: "Bob".to_string(),
        age: Optional::Some(25),
    };
    let users: Vec<UserSelectedEntity> = db.fetch_all_by_template(&template).await?;
    assert_eq!(users.len(), 0);

    println!("template success!");
    Ok(())
}
