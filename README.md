<h1 align="center"> Great Art Stretches Taste. </h1>  

![Building](https://github.com/thegenius/taitan-orm/actions/workflows/rust-ci.yml/badge.svg)
[![Version](https://img.shields.io/badge/crates-0.1.10-green)](https://crates.io/crates/taitan-orm)
[![Version](https://img.shields.io/badge/Lines-32k-yellow)](https://crates.io/crates/taitan-orm)
# Features
-  **Ergonomics** : Ergonomics API design and Error design.
-  **Transactional** : Beautiful Transaction Abstraction, As not a Transaction.
-  **Template** : Write Your Own Sql with Ninja Template Syntax.
-  **Asynchronous** : Fully Async Based on Sqlx.
-  **Compile-Time** : Maximizing Compile-Time Processing, For API and Performance.

# 🎉 What's New in 0.1.10
#### [1] Database support is more accurate: Now so if you only need mysql, there will be no sqlite/postgres code generation
traits has been refactored to database aware 
```rust
pub trait Entity<DB: Database>: Parameter<DB> + Debug {
    fn gen_insert_sql<'a>(&self) -> Cow<'a, str>;
    fn gen_upsert_sql<'a>(&self) -> Cow<'a, str>;
    fn gen_create_sql<'a>(&self) -> Cow<'a, str>;
}
```

#### [2] Template engine now has the full power of Askama Engine
Now you can write any Ninja syntax in template.
```rust
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

#### [3] API has been polished
There are only 7 write apis with intuitive design
```rust
insert(entity) -> () // fail if conflict
upsert(entity) -> () // update if conflict
create(entity) -> () // fail if conflict, return generated field, still experimental!!!

update(mutation, unique  ) -> bool // return true if update take effect
change(mutation, location) -> u64  // return affected rows

delete(unique  ) -> bool // return true if delete take effect
purify(location) -> u64  // return deleted rows
```

There are only 4 read apis with intuitive design
```rust
select       (selection, unique               ) -> Optional<SE>
search       (selection, location, order, page) -> Vec<SE>
search_all   (selection, location, order      ) -> Vec<SE>
search_paged (selection, location, order, page) -> PagedList<SE>
```

Location now can be combined with Logic Operator
```rust
let location = And::new(
    UserLocation::Id(Expr {
        cmp: Cmp::GreaterOrEq,
        val: Some(1),
    }),
    UserLocation::Age(Expr {
        cmp: Cmp::GreaterOrEq,
        val: Some(24),
    }),
);
let location = Or::new(
    UserLocation::Id(Expr {
        cmp: Cmp::GreaterOrEq,
        val: Some(1),
    }),
    UserLocation::Age(Expr {
        cmp: Cmp::GreaterOrEq,
        val: Some(24),
   }),
);

```

**Other APIs are just syntactic sugar and maybe some performance optimize.**


# Quick Start
```toml
taitan-orm = { version = "0.1.10" }
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