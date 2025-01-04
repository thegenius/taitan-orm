use std::borrow::Cow;
use sqlx::mysql::MySqlConnectOptions;
use sqlx::postgres::PgConnectOptions;
use taitan_orm::{Optional, ReaderApi, Schema, SqlExecutor, WriterApi};
use taitan_orm::database::mysql::MySqlDatabase;
use taitan_orm::database::postgres::PostgresDatabase;
use taitan_orm::database::sqlite::{SqliteDatabase, SqliteLocalConfig};
use taitan_orm::traits::Selection;

#[derive(Schema, Clone, Debug)]
#[table_name = "user"]
pub struct User {
    #[primary_key]
    id: i32,
    name: String,
    age: Optional<i32>,
}


#[tokio::main]
async fn main() -> taitan_orm::Result<()> {

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();

    // 0. prepare sqlite database

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
    // let opts: PgConnectOptions = "mysql://root:password@localhost/db".parse()?;
    // let mut db: PostgresDatabase = PostgresDatabase::build(opts).await?;


    let config = SqliteLocalConfig {
        work_dir: Cow::from("./workspace"),
        db_file: Cow::from("test.db"),
    };
    let mut db: SqliteDatabase = SqliteDatabase::build(config).await?;



    db.execute_plain(
        "DROP TABLE IF EXISTS `user`"
    ).await?;
    db.execute_plain(
        "CREATE TABLE IF NOT EXISTS `user`(`id` INT PRIMARY KEY, `age` INT, `name` VARCHAR(64))",
    ).await?;

    // 1. insert entity
    let entity = User {
        id: 1,
        name: "Allen".to_string(),
        age: Optional::Some(23),
    };
    let result = db.insert(&entity).await?;
    assert_eq!(result, true);

    // 2. update
    let mutation = UserMutation {
        name: Optional::None,
        age: Optional::Some(24),
    };
    let primary = UserPrimary { id: 1 };
    let result = db.update(&mutation, &primary).await?;
    assert_eq!(result, true);

    // 3. select
    let selection = UserSelectedEntity::full_fields();
    let entity: Option<UserSelectedEntity> = db.select(&selection, &primary).await?;
    assert!(entity.is_some());

    // 4. search
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
