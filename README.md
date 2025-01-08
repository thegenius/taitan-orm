<h1 align="center"> Great Art Stretches Taste. </h1>  

![Building](https://github.com/thegenius/taitan-orm/actions/workflows/rust-ci.yml/badge.svg)
[![Version](https://img.shields.io/badge/crates-0.1.6-green)](https://crates.io/crates/taitan-orm)
[![Version](https://img.shields.io/badge/Lines-17k-yellow)](https://crates.io/crates/taitan-orm)
# Features
-  **Ergonomics** : Ergonomics API design and Error design.
-  **Transactional** : Beautiful Transaction Abstraction, As not a Transaction.
-  **Template** : Write Your Own Sql Like Mybatis.
-  **Asynchronous** : Fully Async Based on Sqlx.
-  **Compile-Time** : Maximizing Compile-Time Processing, For API and Performance.

# Quick Start
```toml
taitan-orm = { version = "0.1.7" }
```
```rust 

use std::borrow::Cow;
use taitan_orm::database::sqlite::SqliteDatabase;
use taitan_orm::database::sqlite::SqliteLocalConfig;
use taitan_orm::prelude::*;

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
- **0.1 API** 🔧  
**⚠️API may change, so just have a taste!**  
What is being polished?
- 1. write api: to support postgres insert returning syntax
- 2. search api: support index and more
- 3. template api: support %{if } syntax and more


- **0.2 Correctness**: specification and code coverage and fuzz  📎  
**🙏Help is wanted, maybe a long-running mysql instance and a postgres instance**

- **0.3 Documentation**: doc the usage and implementation  📎  
**🖊️Starting from version 0.3, I will focus my efforts on documentation.**

- **0.4 Performance**: benchmark and optimize  📎   
**🚀The ultimate speed originates from maximizing compile-time processing. But we need to exhibit it.**

- **1.0 Stable**: stabilize the api, macro and error  📎



# LICENSE
Apache License