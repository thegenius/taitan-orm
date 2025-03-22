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

    // insert entity
    let entity = User {
        id: 1,
        name: "Allen".to_string(),
        age: Some(23),
    };
    db.insert(&entity).await?;

    let entity = User {
        id: 2,
        name: "Bob".to_string(),
        age: Some(24),
    };
    db.insert(&entity).await?;

    let pagination = Pagination::new(10, 0);
    let order_by = UserOrderBy::build(vec!["id"]).unwrap();

    //  simple search with one condition
    let selection = UserSelected::default();
    let location = UserLocation::Id(Expr {
        cmp: Cmp::GreaterOrEq,
        val: Some(1),
    });
    let entities: Vec<UserSelected> = db
        .search(&selection, &location, &order_by, &pagination)
        .await?;
    assert_eq!(entities.len(), 2);

    // search with multi-conditions
    // conditions connect with AND
    let location = And::new(
        UserLocation::Id(Expr {
            cmp: Cmp::GreaterOrEq,
            val: Some(1),
        }),
        UserLocation::Id(Expr {
            cmp: Cmp::GreaterOrEq,
            val: Some(1),
        }),
    );

    let entities: Vec<UserSelected> = db.search(&selection, &location, &order_by, &pagination).await?;
    assert_eq!(entities.len(), 1);

    // search with multi-conditions connect with OR
    let location = Or::new(
        UserLocation::Id(Expr {
            cmp: Cmp::GreaterOrEq,
            val: Some(1),
        }),
        UserLocation::Id(Expr {
            cmp: Cmp::GreaterOrEq,
            val: Some(1),
        }),
    );
    let entities: Vec<UserSelected> = db.search(&selection, &location,  &order_by, &pagination).await?;
    assert_eq!(entities.len(), 1);

    // for more complicate search, suggest to use template, which can be more maintainable

    println!("search success!");
    Ok(())
}
