# TaiTan-ORM API Documentation

TaiTan-ORM is a high-performance, compile-time optimized ORM for Rust that provides intuitive APIs for database operations. This document outlines the main interfaces and functionality available in the library.

## Table of Contents

1. [Core Concepts](#core-concepts)
2. [Database Connections](#database-connections)
3. [Entity Definition](#entity-definition)
4. [CRUD Operations](#crud-operations)
5. [Query Operations](#query-operations)
6. [Template Engine](#template-engine)
7. [Transactions](#transactions)
8. [Pagination](#pagination)
9. [Low-level Executors](#low-level-executors)

## Core Concepts

TaiTan-ORM is built around several core concepts:

- **Entity**: Represents a database table structure
- **Mutation**: Used for update operations
- **Location**: Used for WHERE conditions
- **Selected**: Used for SELECT projections
- **Unique**: Represents unique constraints
- **Template**: SQL template engine with compile-time processing

## Database Connections

### Supported Databases

- SQLite
- MySQL
- PostgreSQL

### Connection Setup

```rust
// SQLite
use taitan_orm::database::sqlite::prelude::*;
let config = SqliteLocalConfig {
    work_dir: Cow::from("./workspace"),
    db_file: Cow::from("test.db"),
};
let db: SqliteDatabase = SqliteBuilder::build(config).await?;

// MySQL
use taitan_orm::database::mysql::prelude::*;
let opts: MySqlConnectOptions = "mysql://root:password@localhost/db".parse()?;
let db: MySqlDatabase = MySqlDatabase::build(opts).await?;

// PostgreSQL
use taitan_orm::database::postgres::prelude::*;
let opts: PgConnectOptions = "postgres://user:password@localhost/db".parse()?;
let db: PostgresDatabase = PostgresDatabase::build(opts).await?;
```

## Entity Definition

Entities are defined using the `Schema` derive macro:

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
```

This generates several helper structs:
- `UserPrimary` - For primary key operations
- `UserMutation` - For update operations
- `UserUniqueUkName` - For unique constraint operations
- `UserIndexIdxHello` - For index-based queries
- `UserLocation` - For WHERE conditions
- `UserSelected` - For SELECT projections

## CRUD Operations

All CRUD operations are available through the database connection objects.

### Insert

```rust
let entity = User {
    id: 1,
    name: "Allen".to_string(),
    age: Some(23),
    birthday: Some(datetime!(2019-01-01 0:00)),
};
db.insert(&entity).await?;
```

### Update

```rust
let mutation = UserMutation {
    age: Some(Some(24)),
    ..Default::default()
};
let primary = UserPrimary { id: 1 };
let result = db.update(&mutation, &primary).await?;
```

### Select

```rust
let selection = UserSelected::default();
let entity: Option<UserSelected> = db.select(&selection, &primary).await?;
```

### Delete

```rust
let result = db.delete(&primary).await?;
```

## Query Operations

### Select by Unique Constraint

```rust
let uk = UserUniqueUkName {
    name: "Allen".to_string(),
};
let unique_entity: Option<UserSelected> = db.select(&selection, &uk).await?;
```

### Search by Index

```rust
let index = UserIndexIdxHello::AgeBirthday {
    age: Expr::from("=", 24)?,
    birthday: Expr::from("=", datetime!(2019-01-01 0:00))?,
};
let pagination = Pagination::new(10, 0);
let order_by = UserOrderBy::build(vec!["id"]).unwrap();
let index_entities: Vec<UserSelected> = db
    .search::<UserSelected>(&selection, &index, &order_by, &pagination)
    .await?;
```

### Search with Conditions

```rust
let location = UserLocation::Id(Expr {
    val: Some(1),
    cmp: Cmp::Eq,
});
let entities: Vec<UserSelected> = db
    .search(&selection, &location, &order_by, &pagination)
    .await?;
```

### Existence Check

```rust
let exists = db.exists(&primary).await?;
```

### Count Records

```rust
let count = db.count(&location).await?;
```

## Template Engine

TaiTan-ORM includes a powerful template engine based on Askama:

```rust
#[derive(Template, Debug)]
#[template(
    source = "UPDATE `user` SET name = :{name} WHERE `id` = :{id}",
    ext = "txt"
)]
pub struct UserUpdateTemplate {
    id: i32,
    name: String,
}

// Usage
let template = UserUpdateTemplate { id: 1, name: "New Name".to_string() };
db.execute_by_template(&template).await?;
```

Template with conditional logic:

```rust
#[derive(Template, Debug)]
#[template(
    source = "select `id`, `name`, `age` FROM `user` where {% if age.is_some() %} age >= :{age} AND {% endif %} `name` = :{name}",
    ext = "txt"
)]
pub struct UserCustomTemplate {
    name: String,
    age: Option<i32>,
}
```

Template operations:
- `execute_by_template` - Execute template as a statement
- `fetch_one_by_template` - Fetch a single result
- `fetch_option_by_template` - Fetch an optional result
- `fetch_all_by_template` - Fetch multiple results
- `fetch_paged_by_template` - Fetch paged results

## Transactions

Transactions provide ACID compliance with the same API as regular database operations:

```rust
async fn trx_insert_user(
    db: &mut SqliteDatabase,
    user1: &User,
    user2: &User,
) -> taitan_orm::result::Result<()> {
    let mut trx = db.transaction().await?; // create a transaction
    trx.insert(user1).await?;              // same api as database
    trx.insert(user2).await?;              // rollback if there is any error
    trx.commit().await?;                   // commit it
    Ok(())                                 // when trx drop, if not commit, rollback 
}
```

## Pagination

Pagination is supported through the `Pagination` and `PagedList` types:

```rust
let pagination = Pagination::new(10, 0); // page size 10, page number 0
let paged_result = db.search_paged(&selection, &location, &order_by, &pagination).await?;
```

## Low-level Executors

For advanced use cases, TaiTan-ORM provides low-level executor traits:

### SqlExecutor

Immutable executor for read operations:
- `execute` - Execute a statement with arguments
- `execute_plain` - Execute a statement without arguments
- `fetch_exists` - Check if records exist
- `fetch_count` - Count records
- `fetch_option` - Fetch an optional result with selection
- `fetch_all` - Fetch all results with selection
- `fetch_one_full` - Fetch a single result with full selection
- `fetch_option_full` - Fetch an optional result with full selection
- `fetch_all_full` - Fetch all results with full selection

### SqlExecutorMut

Mutable executor for write operations (same methods as SqlExecutor but take `&mut self`).

### SqlGenericExecutor

Generic executor that works with any sqlx Executor:
- Provides generic implementations for all database operations
- Handles result conversion from sqlx types to TaiTan-ORM types
- Abstracts away database-specific details

## Error Handling

All operations return `taitan_orm::result::Result<T>` which is a type alias for `std::result::Result<T, TaitanOrmError>`.

Common error types:
- `TaitanOrmError::DatabaseError` - Database-related errors
- `TaitanOrmError::DecodeError` - Data decoding errors
- `TaitanOrmError::NotImplement` - Not implemented functionality
- `TaitanOrmError::TemplateRenderError` - Template rendering errors

## Performance Features

- **Compile-time Processing**: Maximum compile-time processing for optimal runtime performance
- **Zero-cost Abstractions**: APIs designed to have minimal runtime overhead
- **SQL-First Approach**: Direct SQL access when needed with type safety
- **Async/Await**: Fully asynchronous from the ground up