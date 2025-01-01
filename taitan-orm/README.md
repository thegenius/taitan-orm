<h1 align="center"> Great Art Stretches Taste. </h1> 


# Features
-  **Ergonomics** : Ergonomics API design and Error design.
-  **Transactional** : run transaction as normal.
-  **Template** : Write your own sql like mybatis.
-  **Asynchronous** : Based on SQLx, taitan-orm is fully async.
-  **Multi-Database** : MySql + Postgres + Sqlite for now.

# Quick Start
```toml
taitan-orm = { version = "0.1.0" }
```
```rust 
use std::borrow::Cow;
use taitan_orm::{Optional, ReaderApi, Schema, SqlExecutor, WriterApi};
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

    // 4. delete
    let result = db.delete(&primary).await?;
    assert_eq!(result, true);

    let entity: Option<UserSelectedEntity> = db.select(&selection, &primary).await?;
    assert!(entity.is_none());

    println!("crud success!");
    Ok(())
}
```
* you can run the crud example in examples/crud directory.

# ROADMAP
- **0.1 API** :white_check_mark:
- **0.2 Correctness**: code coverage and mocking :pushpin:
- **0.3 Performance**: benchmark and optimize :pushpin:
- **0.4 Documentation**: doc the usage and implementation
- **1.0 Stable**: stabilize the api, macro and error :pushpin:

# Examples
1. examples/crud: basic crud api example
2. examples/transaction: show the transaction api
3. examples/template: show the template api



# LICENSE
Apache License