# TaiTan-ORM Examples

This document provides an overview of the examples included with TaiTan-ORM and highlights areas that need further clarification.

## Overview of Examples

### 1. CRUD Example (`crud/`)

This example demonstrates basic CRUD operations with TaiTan-ORM:

- **Entity Definition**: Defines a [User](file:///Users/wangwei/project/rust/taitan-orm/examples/crud/src/main.rs#L12-L17) struct with `#[derive(Schema)]` and various attributes like `#[table(user)]`, `#[primary(id)]`, `#[unique(uk_name=(name))]`, and `#[index(idx_hello=(age, birthday))]`
- **Database Setup**: Shows how to set up a SQLite database using [SqliteBuilder::build](file:///Users/wangwei/project/rust/taitan-orm/taitan-orm/src/database/sqlite/mod.rs#L36-L41)
- **Operations Covered**:
  - Inserting entities with [db.insert()](file:///Users/wangwei/project/rust/taitan-orm/taitan-orm/src/database/api.rs#L21-L21)
  - Updating with [db.update()](file:///Users/wangwei/project/rust/taitan-orm/taitan-orm/src/database/api.rs#L32-L32) using [UserMutation](file:///Users/wangwei/project/rust/taitan-orm/taitan-orm-derive/src/schema/mod.rs#L245-L259)
  - Selecting with [db.select()](file:///Users/wangwei/project/rust/taitan-orm/taitan-orm/src/database/api.rs#L26-L26) using [UserSelected](file:///Users/wangwei/project/rust/taitan-orm/taitan-orm-derive/src/schema/mod.rs#L261-L275)
  - Searching with [db.search()](file:///Users/wangwei/project/rust/taitan-orm/taitan-orm/src/database/api.rs#L44-L44) using [UserLocation](file:///Users/wangwei/project/rust/taitan-orm/taitan-orm-derive/src/schema/mod.rs#L277-L291)
  - Deleting with [db.delete()](file:///Users/wangwei/project/rust/taitan-orm/taitan-orm/src/database/api.rs#L38-L38)

### 2. Template Example (`template/`)

This example shows how to use the template engine features:

- **Template Definition**: Defines structs with `#[derive(Template)]` and `#[template(source = "...", ext = "txt")]`
- **Conditional Templates**: Shows how to use conditional logic in templates with `{% if age.is_some() %}`
- **Operations Covered**:
  - Executing templates with [db.execute_by_template()](file:///Users/wangwei/project/rust/taitan-orm/taitan-orm/src/database/api.rs#L80-L80)
  - Fetching results with [db.fetch_option_by_template()](file:///Users/wangwei/project/rust/taitan-orm/taitan-orm/src/database/api.rs#L86-L86), [db.fetch_paged_by_template()](file:///Users/wangwei/project/rust/taitan-orm/taitan-orm/src/database/api.rs#L98-L98), and [db.fetch_all_by_template()](file:///Users/wangwei/project/rust/taitan-orm/taitan-orm/src/database/api.rs#L92-L92)

### 3. Transaction Example (`transaction/`)

This example demonstrates transaction handling:

- **Transaction Creation**: Shows how to create transactions with [db.transaction()](file:///Users/wangwei/project/rust/taitan-orm/taitan-orm/src/database/api.rs#L104-L104)
- **Transaction Operations**: Demonstrates [trx.insert()](file:///Users/wangwei/project/rust/taitan-orm/taitan-orm/src/database/api.rs#L21-L21), [trx.commit()](file:///Users/wangwei/project/rust/taitan-orm/taitan-orm/src/database/api.rs#L110-L110), and rollback behavior
- **Error Handling**: Shows how transaction errors are handled when constraints are violated

### 4. Search Example (`search/`)

This example focuses on search operations with complex conditions:

- **Simple Searches**: Basic search with single conditions
- **Complex Searches**: Using [And::new()](file:///Users/wangwei/project/rust/taitan-orm/taitan-orm/src/database/location.rs#L23-L32) and [Or::new()](file:///Users/wangwei/project/rust/taitan-orm/taitan-orm/src/database/location.rs#L54-L63) to combine multiple conditions
- **Pagination**: Using [Pagination](file:///Users/wangwei/project/rust/taitan-orm/taitan-orm/src/database/pagination.rs#L3-L12) and [PagedList](file:///Users/wangwei/project/rust/taitan-orm/taitan-orm/src/database/pagination.rs#L15-L18) for paginated results

### 5. Axum CRUD Example (`axum_crud/`)

This example shows integration with the Axum web framework:

- **State Management**: Using a custom [AppState](file:///Users/wangwei/project/rust/taitan-orm/examples/axum_crud/src/state.rs#L7-L9) struct that wraps the database executor
- **Deref Implementation**: Implementing [Deref](file:///Users/wangwei/project/rust/taitan-orm/examples/axum_crud/src/state.rs#L24-L30) and [DerefMut](file:///Users/wangwei/project/rust/taitan-orm/examples/axum_crud/src/state.rs#L32-L36) to expose database methods directly
- **Web Handlers**: Creating Axum handlers for various HTTP operations (GET, POST, PATCH, DELETE)
- **Transactions in Web Context**: Using transactions in web handlers with proper error handling

## Areas of Unclear Understanding

While reviewing these examples, I have some questions about the library's usage patterns:

### 1. Derived Types Generation

I'm not fully clear on how the derive macros generate the various helper structs:

- How exactly does `#[derive(Schema)]` generate [UserPrimary](file:///Users/wangwei/project/rust/taitan-orm/taitan-orm-derive/src/schema/mod.rs#L225-L229), [UserMutation](file:///Users/wangwei/project/rust/taitan-orm/taitan-orm-derive/src/schema/mod.rs#L245-L259), [UserSelected](file:///Users/wangwei/project/rust/taitan-orm/taitan-orm-derive/src/schema/mod.rs#L261-L275), [UserLocation](file:///Users/wangwei/project/rust/taitan-orm/taitan-orm-derive/src/schema/mod.rs#L277-L291), etc.?
- What determines which helper structs are generated based on the attributes?

### 2. Database Abstraction

I'm unclear about the database abstraction layers:

- How do [SqliteDatabase](file:///Users/wangwei/project/rust/taitan-orm/taitan-orm/src/database/sqlite/database.rs#L13-L16), [MySqlDatabase](file:///Users/wangwei/project/rust/taitan-orm/taitan-orm/src/database/mysql/database.rs#L10-L13), and [PostgresDatabase](file:///Users/wangwei/project/rust/taitan-orm/taitan-orm/src/database/postgres/database.rs#L10-L13) relate to each other?
- What traits do they implement to provide a common interface?

### 3. Template Engine Internals

I have questions about the template engine:

- How does the template engine process the conditional logic (`{% if ... %}`)?
- What is the relationship between the template engine and the underlying SQLx library?

### 4. Error Handling Patterns

I'm not fully clear on the error handling patterns:

- How are database errors propagated and handled in different contexts?
- What's the difference between various error types in the library?

### 5. Performance Considerations

I'm unclear about performance aspects:

- How does the compile-time optimization work in practice?
- What are the performance implications of using the various features?

## Summary

The examples demonstrate that TaiTan-ORM provides a comprehensive set of features for database operations with a focus on compile-time optimization. The derive macros seem to generate significant boilerplate code that simplifies database operations, and the library supports multiple database backends with a consistent API.

However, I need further clarification on how the derive macros work internally and how the various components of the library fit together to provide the performance benefits mentioned in the documentation.