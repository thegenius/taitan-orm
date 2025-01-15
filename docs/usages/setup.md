
# MySql Setup
```rust

 // manually build ConnectOptions
 let conn = MySqlConnectOptions::new()
     .host("localhost")
     .username("root")
     .password("password")
     .database("db")
     .connect().await?;
 
 // parse options from a string
 let opts: MySqlConnectOptions = "mysql://root:password@localhost/db".parse()?;
 let mut db: MySqlDatabase = MySqlDatabase::build(opts).await?;
```

# Postgres Setup
```rust
 // For Postgres
 Manually-constructed options
 let conn = PgConnectOptions::new()
     .host("secret-host")
     .port(2525)
     .username("secret-user")
     .password("secret-password")
     .ssl_mode(PgSslMode::Require)
     .connect()
     .await?;
 // parse options from a string
 let mut opts: PgConnectOptions = "postgres:// localhost/ mydb".parse()?;
 let mut db: PostgresDatabase = PostgresDatabase::build(opts).await?;
```

# Sqlite Setup
```rust
 // 0. prepare sqlite database
 let config = SqliteLocalConfig {
  work_dir: Cow::from("./workspace"),
  db_file: Cow::from("test.db"),
 };
 let mut db: SqliteDatabase = SqliteDatabase::build(config).await?;


```

# Create Table
```rust
 db.execute_plain("DROP TABLE IF EXISTS `user`").await?;

 db.execute_plain(
  "CREATE TABLE IF NOT EXISTS `user`(`id` INT PRIMARY KEY, `age` INT, `name` VARCHAR(64), `birthday` DATETIME)",
 ).await?;

 db.execute_plain(
  "CREATE UNIQUE INDEX `uk_name` ON `user` (`name`);",
 ).await?;

 db.execute_plain(
  "CREATE INDEX `idx_age_birthday` ON `user` (`age`, `birthday`);",
 ).await?;
```