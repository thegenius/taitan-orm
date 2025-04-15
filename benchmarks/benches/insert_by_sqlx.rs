use sqlx::mysql::MySqlConnectOptions;
use sqlx::postgres::PgConnectOptions;
use sqlx::types::time::PrimitiveDateTime;
use std::borrow::Cow;
use std::fs;
use std::path::Path;
use std::time::Duration;
use time::macros::datetime;
use path_absolutize::Absolutize;

use sonyflake::Sonyflake;
use taitan_orm::database::sqlite::prelude::*;
use taitan_orm::prelude::*;

#[derive(Debug, Schema, Clone)]
#[table(user)]
#[unique(uk_name=(name))]
#[index(idx_hello=(age, birthday))]
#[primary(id)]
pub struct User {
    id: i64,
    name: String,
    age: Option<i32>,
    birthday: Option<PrimitiveDateTime>,
}

use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};
use tokio::runtime::Runtime;
use tracing::info;

async fn init_db(db_file: &str) -> taitan_orm::result::Result<SqliteDatabase> {
    // let config = SqliteLocalConfig {
    //     work_dir: Cow::from("./workspace"),
    //     db_file: Cow::from(db_file),
    // };
    let mut db: SqliteDatabase = SqliteBuilder::build_mem().await?;
    db.execute_plain("DROP TABLE IF EXISTS `user`").await?;
    db.execute_plain(
        "CREATE TABLE IF NOT EXISTS `user`(`id` BIGINT PRIMARY KEY, `age` INT, `name` VARCHAR(64), `birthday` DATETIME)",
    )
        .await?;
    // db.execute_plain("CREATE UNIQUE INDEX `uk_name` ON `user` (`name`);")
    //     .await?;
    // db.execute_plain("CREATE INDEX `idx_age_birthday` ON `user` (`age`, `birthday`);")
    //     .await?;
    Ok(db)
}

async fn sqlx_insert(db: &SqliteDatabase, user: User) {
    sqlx::query("INSERT INTO user (id, age, name, birthday) VALUES (?, ?, ?, ?)")
        .bind(user.id)
        .bind(user.age)
        .bind(user.name)
        .bind(user.birthday)
        .execute(db.get_pool().unwrap())
        .await
        .unwrap();
}
fn gen_user(sony_flake: &Sonyflake) -> User {
    User {
        id: sony_flake.next_id().unwrap() as i64,
        name: "test_user".to_string(),
        age: Some(23),
        birthday: Some(datetime!(2019-01-01 0:00)),
    }
}

fn bench_async_insert(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let db = rt.block_on(async { init_db("test.db").await.unwrap() });
    let mut group = c.benchmark_group("Async Insert");
    group
        .warm_up_time(Duration::from_secs(5))
        .measurement_time(Duration::from_secs(20))
        .sample_size(10_000);
    let sony_flake = Sonyflake::new().unwrap();


    group.bench_function("sqlx_insert", |b| {
        b.to_async(&rt).iter_batched(
            || gen_user(&sony_flake),
            |user| async {  sqlx_insert(&db, user).await; },
            BatchSize::SmallInput,
        )
    });

    group.finish();
}

criterion_group!(benches, bench_async_insert);
criterion_main!(benches);
