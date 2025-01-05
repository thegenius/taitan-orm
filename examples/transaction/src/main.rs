use sqlx::types::time::PrimitiveDateTime;
use std::borrow::Cow;
use taitan_orm::database::sqlite::{SqliteDatabase, SqliteLocalConfig};
use taitan_orm::prelude::*;
use time::macros::datetime;

#[derive(Schema, Clone, Debug)]
#[table_name = "user"]
pub struct User {
    #[primary_key]
    id: i32,
    name: String,
    age: Optional<i32>,
    birthday: Optional<PrimitiveDateTime>,
}

async fn trx_insert_user(
    db: &mut SqliteDatabase,
    user1: &User,
    user2: &User,
) -> taitan_orm::result::Result<()> {
    let mut trx = db.transaction().await?;
    trx.insert(user1).await?;
    trx.insert(user2).await?;
    trx.commit().await?;
    Ok(())
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
        "CREATE TABLE IF NOT EXISTS `user`(`id` INT PRIMARY KEY, `age` INT, `name` VARCHAR(64), `birthday` DATETIME)",
    ).await?;

    // 1. transaction should be successfully commit
    let user1 = User {
        id: 1,
        name: "Allen".to_string(),
        age: Optional::Some(23),
        birthday: Optional::Some(datetime!(2019-01-01 0:00)),
    };
    let user2 = User {
        id: 2,
        name: "Bob".to_string(),
        age: Optional::Some(24),
        birthday: Optional::Some(datetime!(2019-01-01 0:00)),
    };
    let result = trx_insert_user(&mut db, &user1, &user2).await;
    assert!(result.is_ok());
    let entity: Option<UserSelectedEntity> = db
        .select(&UserSelectedEntity::full_fields(), &UserPrimary { id: 1 })
        .await?;
    assert!(entity.is_some());

    // 2. transaction should be rollback
    let user3 = User {
        id: 3,
        name: "Allen".to_string(),
        age: Optional::Some(24),
        birthday: Optional::Some(datetime!(2019-01-01 0:00)),
    };
    let user4 = User {
        id: 3,
        name: "Bob".to_string(),
        age: Optional::Some(24),
        birthday: Optional::Some(datetime!(2019-01-01 0:00)),
    };
    let result = trx_insert_user(&mut db, &user3, &user4).await;
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "error returned from database: (code: 1555) UNIQUE constraint failed: user.id".to_string()
    );
    let entity: Option<UserSelectedEntity> = db
        .select(&UserSelectedEntity::full_fields(), &UserPrimary { id: 3 })
        .await?;
    assert!(entity.is_none());

    println!("transaction success!");
    Ok(())
}
