<h1 align="center"> Great Art Stretches Taste. </h1>  

![Building](https://github.com/thegenius/taitan-orm/actions/workflows/rust-ci.yml/badge.svg)
[![Version](https://img.shields.io/badge/crates-0.1.12-green)](https://crates.io/crates/taitan-orm)
[![Version](https://img.shields.io/badge/Lines-32k-yellow)](https://crates.io/crates/taitan-orm)

# Features
<div style="margin-left: 20px; font-family: 'SFMono-Regular', Consolas, 'Liberation Mono', Menlo, Courier, monospace;">
  <div><strong>SQL-First</strong>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;: SQL with template syntax and extremely performance.</div>
  <div><strong>Ergonomics</strong>&nbsp;&nbsp;&nbsp;&nbsp;: Ergonomics API design and error handling.</div>
  <div><strong>Compile-Time</strong>&nbsp;&nbsp;: Maximizing compile-time processing.</div>
  <div><strong>Transactional</strong>&nbsp: Beautiful transaction, as if not a transaction.</div>
  <div><strong>Asynchronous</strong>&nbsp;&nbsp;: Fully async from day one.</div>
</div>


# Why Another ORM
🔗[Why Another ORM](https://github.com/thegenius/taitan-orm/blob/main/docs/why_another_orm_en.md)

# Deep Wiki
🔗[wiki](https://deepwiki.com/thegenius/taitan-orm)

# Quick Taste
```
git clone https://github.com/thegenius/taitan-orm.git
cd taitan-orm/examples/crud
cargo run
```

# Examples
At present, the documentation for this newly-born project is limited. You can refer to the examples project for more details.

| example     | description                 |
|-------------|-----------------------------|
| crud        | basic crud example          |
| template    | template with paged example |
| transaction | basic transaction example   |
| search      | multi search features       |
| axum_crud   | integrate with axum         |

# Quick Start
```shell
cargo add taitan-orm
```

```rust
use std::borrow::Cow;
use sqlx::types::time::PrimitiveDateTime;
use time::macros::datetime;

use taitan_orm::database::sqlite::prelude::*;
use taitan_orm::prelude::*;

#[derive(Debug, Schema, Clone)]
#[table(user)]
#[unique(uk_name=(name))]
#[index(idx_hello=(age, birthday))]
#[primary(id)]
pub struct User {
 id: i32,
 name: String,
 age: Option<i32>,
 birthday: Option<PrimitiveDateTime>,
}

#[tokio::main]
async fn main() -> taitan_orm::result::Result<()> {
    tracing_subscriber::fmt()
            .with_max_level(tracing::Level::TRACE)
            .init();
 
    // setup database
    // refer to docs/setup.md for database setup
    let db = ...;
    db.execute_plain("DROP TABLE IF EXISTS `user`").await?;
    db.execute_plain(
     "CREATE TABLE IF NOT EXISTS `user`(`id` INT PRIMARY KEY, `age` INT, `name` VARCHAR(64), `birthday` DATETIME)",
    )
            .await?;
    db.execute_plain("CREATE UNIQUE INDEX `uk_name` ON `user` (`name`);")
            .await?;
    db.execute_plain("CREATE INDEX `idx_age_birthday` ON `user` (`age`, `birthday`);")
            .await?;
   
    // 1. insert entity
    let entity = User {
     id: 1,
     name: "Allen".to_string(),
     age: Option::Some(23),
     birthday: Option::Some(datetime!(2019-01-01 0:00)),
    };
    let result = db.insert(&entity).await?;
   
    // 2. update
    let mutation = UserMutation {
     name: None,
     age: Some(Some(24)),
     birthday: None,
    };
    let primary = UserPrimary { id: 1 };
    let result = db.update(&mutation, &primary).await?;
    assert_eq!(result, true);
   
    // 3. select
    let selection = UserSelected::default();
    let entity: Option<UserSelected> = db.select(&selection, &primary).await?;
    assert!(entity.is_some());
   
    // 4. select by unique
    let uk = UserUniqueUkName {
     name: "Allen".to_string(),
    };
    let unique_entity: Option<UserSelected> = db.select(&selection, &uk).await?;
    assert!(unique_entity.is_some());
   
    // 5. search by index
    let index = UserIndexIdxHello::AgeBirthday {
     age: Expr::from("=", 24)?,
     birthday: Expr::from("=", datetime!(2019-01-01 0:00))?,
    };
    let pagination = Pagination::new(10, 0);
    let order_by = UserOrderBy::build(vec!["id"]).unwrap();
    let index_entities: Vec<UserSelected> = db
            .search::<UserSelected>(&selection, &index, &order_by, &pagination)
            .await?;
    assert_eq!(index_entities.len(), 1);
   
    // 6. search
    let selection = UserSelected::default();
    let location = UserLocation::Id(Expr {
     val: Some(1),
     cmp: Cmp::Eq,
    });
    let entities: Vec<UserSelected> = db
            .search(&selection, &location, &order_by, &pagination)
            .await?;
    assert_eq!(entities.len(), 1);
   
    // 7. delete
    let result = db.delete(&primary).await?;
    assert_eq!(result, true);
   
    let entity: Option<UserSelected> = db.select(&selection, &primary).await?;
    assert!(entity.is_none());
   
    println!("crud success!");
    Ok(())
}
```
* you can run the crud example in examples/crud directory.


# Documents
1. 🔗[Setup](https://github.com/thegenius/taitan-orm/blob/main/docs/usages/setup.md)
2. 🔗[Schema Definition](https://github.com/thegenius/taitan-orm/blob/main/docs/usages/schema_definition.md)
3. 🔗[Write API](https://github.com/thegenius/taitan-orm/blob/main/docs/write_en.md)

# Usage
## Compile Time Generation  
When derive(Schema), TaiTan-ORM will generate helper struct for you.

```rust
#[derive(Debug, Schema, Clone)]
#[table(user)]
#[unique(uk_name=(name))]
#[index(idx_hello=(age, birthday))]
#[primary(id)]
pub struct User {
    id: i32,
    name: String,
    age: Option<i32>,
    birthday: Option<PrimitiveDateTime>,
}

// struct for primary key 
pub struct UserPrimary {
    id: i32
}

// struct for update
// None: skip this field
// Some(None): null
// Some(Some(23)): actual set value
// before 0.1.9, there is a special enum Optional to support null expression
// 0.1.10 remove Optional, use Option<Option<T>> instead.
pub struct UserMutation {
    name: Option<Option<String>>,
    age: Option<Option<i32>>,
    birthday: Option<Option<PrimitiveDateTime>>
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
pub enum UserLocation {
    Id(LocationExpr<i32>),
    Name(LocationExpr<String>),
    Age(LocationExpr<i32>),
    Birthday(LocationExpr<PrimitiveDateTime>)
}

// struct to select field and recv result from database 
pub struct UserSelected {
    id: Option<Option<i32>>,
    name: Option<Option<String>>,
    age: Option<Option<i32>>,
    birthday: Option<Option<PrimitiveDateTime>>
}
```


## Template   
TaiTan-ORM: The Most Powerful ORM Template Engine You'll Ever Meet

| syntax                                   | description                |
|------------------------------------------|----------------------------|
| <span style="color: red;">:{}</span>     | placeholder binding syntax |
| <span style="color: white;">{{ }}</span> | Askama variable syntax     |
| <span style="color: green;">{% %}</span> | Askame template syntax     |

```rust
#[derive(Template, askama::Template, Debug)]
#[template(
 source = "UPDATE `user` SET name = :{name} WHERE `id` = :{id}",
 ext = "txt"
)]
pub struct UserUpdateTemplate {
 id: i32,
 name: String,
}

#[derive(Template, askama::Template, Debug)]
#[template(
 source = "select `id`, `name`, `age` FROM `user` where `id` >= :{id}",
 ext = "txt"
)]
pub struct UserSelectTemplate {
 id: i32,
}

#[derive(Template, askama::Template, Debug)]
#[template(
 source = "select `id`, `name`, `age` FROM `user` where {% if age.is_some() %} age >= :{age} AND {% endif %} `name` = :{name}",
 ext = "txt"
)]
pub struct UserCustomTemplate {
 name: String,
 age: Option<i32>,
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




 
# Supported Database
 MySql  
 Postgres   
 Sqlite  
Please refer to 🔗[Setup](https://github.com/thegenius/taitan-orm/blob/main/docs/usages/setup.md) for details

# Performance
Bench Environment:
```
CPU: Apple M4
MEM: 16G
SSD: 256G
OS : Mac Sequoia 15.1.1 
DB : Sqlite Memory Mode
```

| ORM        | Insert        | select        |
|------------|---------------|---------------|
| sqlx       | 18.676 us/ops | -             |
| TaiTan-ORM | 17.831 us/ops | 15.942 us/ops |
| Rbatis     | 19.360 us/ops | 18.248 us/ops |
| Sea-ORM    | 28.116 us/ops | 25.818 us/ops |

Thanks to the maximum usage of compile time processing, TaiTan-ORM even faster than sqlx binding style!!

**You can run benchmarks on your own machine in directory of `./benchmarks` with `cargo bench`**

# Release Notes
## 0.1.12
include the askama template, simplify the template definition
## 0.1.11
compile time tracing dependency
## 0.1.10
[1] Database support is more accurate: Now so if you only need mysql, there will be no sqlite/postgres code generation  
[2] Template engine has the full power of Askama Engine  
[3] API has been polished  
[4] Location now can be combined with Logic Operator  



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