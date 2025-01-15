use sqlx::mysql::MySqlConnectOptions;
use sqlx::postgres::PgConnectOptions;
use std::borrow::Cow;
use sqlx::types::time::PrimitiveDateTime;
use time::macros::datetime;
use taitan_orm::database::mysql::MySqlDatabase;
use taitan_orm::database::postgres::PostgresDatabase;

use taitan_orm::database::sqlite::SqliteDatabase;
use taitan_orm::database::sqlite::SqliteLocalConfig;

use taitan_orm::prelude::*;

#[derive(Schema, Clone, Debug)]
#[table_name = "user"]
#[unique_key = "name"]
#[index(name = "idx_hello", fields("age", "birthday"))]
pub struct User {
    #[primary_key]
    id: i32,
    name: String,
    age: Optional<i32>,
    birthday: Optional<PrimitiveDateTime>
}

#[tokio::main]
async fn main() -> taitan_orm::result::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();

    // // For MySql
    // // manually build ConnectOptions
    // let conn = MySqlConnectOptions::new()
    //     .host("localhost")
    //     .username("root")
    //     .password("password")
    //     .database("db")
    //     .connect().await?;
    //
    // // parse options from a string
    // let opts: MySqlConnectOptions = "mysql://root:password@localhost/db".parse()?;
    // let mut db: MySqlDatabase = MySqlDatabase::build(opts).await?;

    // // For Postgres
    // Manually-constructed options
    // let conn = PgConnectOptions::new()
    //     .host("secret-host")
    //     .port(2525)
    //     .username("secret-user")
    //     .password("secret-password")
    //     .ssl_mode(PgSslMode::Require)
    //     .connect()
    //     .await?;
    // // parse options from a string
    // let mut opts: PgConnectOptions = "postgres:// localhost/ mydb".parse()?;
    // let mut db: PostgresDatabase = PostgresDatabase::build(opts).await?;

    // 0. prepare sqlite database
    let config = SqliteLocalConfig {
        work_dir: Cow::from("./workspace"),
        db_file: Cow::from("test.db"),
    };
    let mut db: SqliteDatabase = SqliteDatabase::build(config).await?;

    db.execute_plain("DROP TABLE IF EXISTS `user`").await?;
    db.execute_plain(
        "CREATE TABLE IF NOT EXISTS `user`(`id` INT PRIMARY KEY, `age` INT, `name` VARCHAR(64), `birthday` DATETIME)",
    )
    .await?;
    db.execute_plain(
        "CREATE UNIQUE INDEX `uk_name` ON `user` (`name`);",
    )
        .await?;
    db.execute_plain(
        "CREATE INDEX `idx_age_birthday` ON `user` (`age`, `birthday`);",
    )
        .await?;

    // 1. insert entity
    let entity = User {
        id: 1,
        name: "Allen".to_string(),
        age: Optional::Some(23),
        birthday: Optional::Some(datetime!(2019-01-01 0:00))
    };
    let result = db.insert(&entity).await?;
    assert_eq!(result, true);

    // 2. update
    let mutation = UserMutation {
        name: Optional::None,
        age: Optional::Some(24),
        birthday: Optional::None
    };
    let primary = UserPrimary { id: 1 };
    let result = db.update(&mutation, &primary).await?;
    assert_eq!(result, true);

    // 3. select
    let selection = UserSelectedEntity::full_fields();
    let entity: Option<UserSelectedEntity> = db.select(&selection, &primary).await?;
    assert!(entity.is_some());

    // 4. select by unique
    let uk = UserNameUnique { name: "Allen".to_string() };
    let unique_entity : Option<UserSelectedEntity> = db.select(&selection, &uk).await?;
    assert!(unique_entity.is_some());

    // 5. search by index
    let index = UserIndexIdxHello::AgeBirthday {
        age: LocationExpr::from("=", 24)?,
        birthday: LocationExpr::from("=", datetime!(2019-01-01 0:00))?
    };
    let index_entities: Vec<UserSelectedEntity> = db.search(&selection, &index, &None, &None).await?;
    assert_eq!(index_entities.len(), 1);

    // 6. search
    let selection = UserSelectedEntity::full_fields();
    let location = UserLocationExpr::id(">=", 1)?;
    let entities: Vec<UserSelectedEntity> = db.search(&selection, &location, &None, &None).await?;
    assert_eq!(entities.len(), 1);

    // 5. delete
    let result = db.delete(&primary).await?;
    assert_eq!(result, true);

    let entity: Option<UserSelectedEntity> = db.select(&selection, &primary).await?;
    assert!(entity.is_none());

    println!("crud success!");
    Ok(())
}
