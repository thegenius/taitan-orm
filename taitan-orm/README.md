<h1 align="center"> Great Art Stretches Taste. </h1>  

![Building](https://github.com/thegenius/taitan-orm/actions/workflows/rust-ci.yml/badge.svg)
[![Version](https://img.shields.io/badge/Lines-18k-yellow)](https://crates.io/crates/taitan-orm)
# Features
-  **Ergonomics** : Ergonomics API design and Error design.
-  **Transactional** : Beautiful Transaction Abstraction, As not a Transaction.
-  **Template** : Write Your Own Sql Like Mybatis.
-  **Asynchronous** : Fully Async Based on Sqlx.

# Quick Start

```rust 
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
```
* you can run the crud example in examples/crud directory.

# Examples
At present, the documentation for this newly-born project is limited. You can refer to the examples project for more details.

| example     | descption                   |
|-------------|-----------------------------|
| crud        | basic crud example          |
| template    | template with paged example |
| transaction | basic transaction example   |
| search      | multi search features       |
| axum_crud   | integrate with axum         |

# Supported Database
MySql  
Postgres  
Sqlite  

# ROADMAP
- **0.1 API** 
- **0.2 Correctness**: code coverage and mocking 
- **0.3 Performance**: benchmark and optimize. 
- **0.4 Documentation**: doc the usage and implementation
- **1.0 Stable**: stabilize the api, macro and error 



# LICENSE
Apache License