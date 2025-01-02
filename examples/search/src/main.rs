use std::borrow::Cow;
use taitan_orm::{Optional, ReaderApi, Schema, SqlExecutor, WriterApi};
use taitan_orm::database::sqlite::{SqliteDatabase, SqliteLocalConfig};
use taitan_orm::traits::{LocationExpr, Selection};
use taitan_orm::CmpOperator;

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

    let entity = User {
        id: 2,
        name: "Bob".to_string(),
        age: Optional::Some(24),
    };
    let result = db.insert(&entity).await?;
    assert_eq!(result, true);


    // 3. search
    let selection = UserSelectedEntity::full_fields();
    let location = UserLocation {
        name: Optional::Some(LocationExpr::new(CmpOperator::Eq, "Bob".to_string())),
        ..Default::default()
    };
    let entities: Vec<UserSelectedEntity> = db.search(&selection, &location, &None, &None).await?;
    assert_eq!(entities.len(), 1);

    // 4. search
    let selection = UserSelectedEntity::full_fields();
    let location = UserLocation {
        age: Optional::Some(LocationExpr::new(CmpOperator::GreaterOrEq, 23)),
        ..Default::default()
    };
    let entities: Vec<UserSelectedEntity> = db.search(&selection, &location, &None, &None).await?;
    assert_eq!(entities.len(), 2);


    println!("crud success!");
    Ok(())
}
