use std::borrow::Cow;
use taitan_orm::prelude::*;
use taitan_orm::database::sqlite::{SqliteDatabase, SqliteLocalConfig};

#[derive(Schema, Clone, Debug)]
#[table_name = "user"]
pub struct User {
    #[primary_key]
    id: i32,
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
    db.execute_plain(
        "DROP TABLE IF EXISTS `user`"
    ).await?;
    db.execute_plain(
        "CREATE TABLE IF NOT EXISTS `user`(`id` INT PRIMARY KEY, `age` INT, `name` VARCHAR(64))",
    ).await?;

    // insert entity
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

    //  simple search with one condition
    let selection = UserSelectedEntity::full_fields();
    let location = UserLocationExpr::id(">=", 1)?;
    let entities: Vec<UserSelectedEntity> = db.search(&selection, &location, &None, &None).await?;
    assert_eq!(entities.len(), 2);

    //  simple search with one condition but without construct error
    let selection = UserSelectedEntity::full_fields();
    let location = UserLocationExpr::Id(LocationExpr::new(CmpOperator::GreaterOrEq, 1));
    let entities: Vec<UserSelectedEntity> = db.search(&selection, &location, &None, &None).await?;
    assert_eq!(entities.len(), 2);


    // search with multi-condition
    let selection = UserSelectedEntity::full_fields();
    let location = UserLocation {
        id: Optional::Some(LocationExpr::new(CmpOperator::Eq, 2)),
        name: Optional::Some(LocationExpr::new(CmpOperator::Eq, "Bob".to_string())),
        ..Default::default()
    };
    let entities: Vec<UserSelectedEntity> = db.search(&selection, &location, &None, &None).await?;
    assert_eq!(entities.len(), 1);


    // search with multi-condition connect with OR
    let selection = UserSelectedEntity::full_fields();
    let location = UserLocation {
        mode: LocationMode::Or,
        id: Optional::Some(LocationExpr::from(">", 3)?),
        age: Optional::Some(LocationExpr::from("<=", 23)?),
        ..Default::default()
    };
    let entities: Vec<UserSelectedEntity> = db.search(&selection, &location, &None, &None).await?;
    assert_eq!(entities.len(), 1);

    // for more complicate search, suggest to use template, which can be more maintainable


    println!("search success!");
    Ok(())
}
