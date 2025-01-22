<h1 align="center"> Great Art Stretches Taste. </h1>  

![Building](https://github.com/thegenius/taitan-orm/actions/workflows/rust-ci.yml/badge.svg)
[![Version](https://img.shields.io/badge/crates-0.1.9-green)](https://crates.io/crates/taitan-orm)
[![Version](https://img.shields.io/badge/Lines-19k-yellow)](https://crates.io/crates/taitan-orm)
# Features
-  **Ergonomics** : Ergonomics API design and Error design.
-  **Transactional** : Beautiful Transaction Abstraction, As not a Transaction.
-  **Template** : Write Your Own Sql Like Mybatis.
-  **Asynchronous** : Fully Async Based on Sqlx.
-  **Compile-Time** : Maximizing Compile-Time Processing, For API and Performance.

🎆**The next two weeks are for the Chinese Spring Festival, and I will be taking two weeks off.**
**Following the Spring Festival, I plan to release a significant update. This new version will encompass a wide range of enhancements and API updates.**


# Quick Start
```toml
taitan-orm = { version = "0.1.9" }
```
```rust 

use std::borrow::Cow;
use sqlx::types::time::PrimitiveDateTime;
use time::macros::datetime;
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

 // 0. setup database
 // refer to docs/setup.md for database setup
 let db = ...; 

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
 let entity: Option<UserSelectedEntity> 
         = db.select(&selection, &primary).await?;
 assert!(entity.is_some());

 // 4. select by unique
 let uk = UserNameUnique { name: "Allen".to_string() };
 let unique_entity : Option<UserSelectedEntity> 
         = db.select(&selection, &uk).await?;
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
```
* you can run the crud example in examples/crud directory.

# Usage
## Compile Time Generation  
When derive(Schema), TaiTan-ORM will generate helper struct for you.

```rust
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

// struct for primary key 
pub struct UserPrimary {
 id: i32
}

// struct for update 
pub struct UserMutation {
 name: Optional<String>,
 age: Optional<i32>,
 birthday: Optional<PrimitiveDateTime>
}

// struct for unique key
pub struct UserNameUnique { 
 name: String 
}

// struct for index_hello, designed for index prefix matching
// age is allowed,
// age, birthday is allowed
// birthday is not allowed
pub enum UserIndexIdxHello {
    Age { 
        age: LocationExpr<i32> 
    },
    AgeBirthday{ 
        age: LocationExpr<i32>, 
        birthday: LocationExpr<PrimitiveDateTime> 
    }
}

// struct to generate where condition 
pub struct UserLocation {
 mode: LocationMode,
 id: Optional<LocationExpr<i32>>,
 name: Optional<LocationExpr<String>>,
 age: Optional<LocationExpr<i32>>,
}

// struct to select field and recv result from database 
pub struct UserSelectedEntity {
 id: Optional<i32>,
 name: Optional<String>,
 age: Optional<i32>,
}
```


## Template   
TaiTan-ORM: The Most Powerful ORM Template Engine You'll Ever Meet

### 1. Comprehensive Template Features with Jinja Syntax Support
Titan-ORM offers a full-featured template engine that supports the popular Jinja syntax, providing powerful templating capabilities.
### 2. Compile-Time Processing for Zero Runtime Overhead
In the vast majority of cases, compile-time processing is utilized, resulting in essentially zero overhead during runtime. This ensures optimal performance and efficiency.
### 3. A Revolutionary Optional Syntax
Introducing a new Optional syntax that helps you avoid cumbersome and unsightly template code, making your templates cleaner and more maintainable.
### ⚠️ <span style="color: red;">warning: before 0.2, syntax may change</span>

| syntax                                   | description                                                            |
|------------------------------------------|------------------------------------------------------------------------|
| <span style="color: red;">#{}</span>     | compile time render syntax, may change                                 |
| <span style="color: white;">${}</span>   | dynamic render syntax                                                  |
| <span style="color: red;">%{}</span>     | now used for optional, but it may confused with {% %}, so maybe change |
| <span style="color: green;">{% %}</span> | Jinja template syntax, **this will not change**                        |
| <span style="color: white;">@{}</span>   | candidate for compile time render                                      |

```rust
/// This is the #{} syntax, this will be parsed at compile-time,
/// In run time, engine will get the sql: 
/// UPDATE `user` SET name = ? WHERE `id` = ?
/// There will be zero-overhead at run-time
#[derive(TemplateRecord, Debug)]
#[sql = "UPDATE `user` SET name = #{name} WHERE `id` = #{id}"]
pub struct UserUpdateTemplate {
    id: i32,
    name: String,
}

/// This is the ${} syntax, and will render the sql at run-time,
/// After render, engine get the sql: 
/// UPDATE `user` SET user_name = 2 WHERE `id` = 100
/// This is usually used for dynamic sql generation
#[derive(TemplateRecord, Debug)]
#[sql = "UPDATE `user` SET ${name} = 2 WHERE `id` = ${id}"]
pub struct UserUpdateTemplate {
 id: i32,      // suppose to be 100
 name: String, // suppose to be "user_name"
}

/// This is the %{} syntax, the most beautiful one, will be parsed at run time,
/// But Special for Optional variable
/// 1. When template is { name: "Allen", age: None },
///    After render, engine will get the sql: 
///    select `id`, `name`, `age` FROM `user` where `name` = ?
/// 2. When template is { name: "Allen", age: Some(33) },
///    After render, engine will get the sql: 
///    select `id`, `name`, `age` FROM `user` where age >= ? AND `name` = ?
///
/// when parser encounters "%{age}", 
/// it will treat the entire expression with connective 
/// "age >= %{age} AND" as an integrated unit
/// when Optional variable is None, the entire expression will be ignored
#[derive(TemplateRecord, Debug)]
#[sql = "select `id`, `name`, `age` FROM `user` where age >= %{age} AND `name` = #{name}"]
pub struct UserCustomTemplate {
 name: String,
 age: Optional<i32>,
}

```
* you can run the template example in examples/template directory.

## Transaction
Seamless transaction handling
```rust
async fn trx_insert_user(
    db: &mut SqliteDatabase,
    user1: &User,
    user2: &User,
) -> taitan_orm::result::Result<()> {
    let mut trx = db.transaction().await?; // create a transaction, trx
    trx.insert(user1).await?;              // same api as database
    trx.insert(user2).await?;              // rollback if there is any error
    trx.commit().await?;                   // commit it
    Ok(())                                 // when trx drop, if not commit, rollback 
}
```


# Concepts
![](https://github.com/thegenius/taitan-orm/blob/main/docs/concept.png)

# Documents
1. [Write API](https://github.com/thegenius/taitan-orm/blob/main/docs/write_en.md)

# Examples
At present, the documentation for this newly-born project is limited. You can refer to the examples project for more details.

| example     | description                 |
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
Please refer to 🔗[Setup](https://github.com/thegenius/taitan-orm/blob/main/docs/setup.md) for details


# ROADMAP
- **0.1 API** 🔧  
**⚠️API may change, so just have a taste!**  
What is being polished?
- 1. write api: to support postgres insert returning syntax and batch insert/batch upsert
- 2. search api: support index and more
- 3. error

- **0.2 Correctness**: specification and code coverage and fuzz  📎  
🙏Help is wanted, maybe a long-running mysql instance and a postgres instance  
now there is a rough coverage report: 🪧[report](https://github.com/thegenius/taitan-orm/blob/main/docs/coverage.md)

- **0.3 Documentation**: doc the usage and implementation  📎  
🖊️Starting from version 0.3, I will focus my efforts on documentation.

- **0.4 Performance**: benchmark and optimize  📎   
🚀The ultimate speed originates from maximizing compile-time processing. But we need to exhibit it.

- **1.0 Stable**: stabilize the api, macro and error  📎



# LICENSE
Apache License