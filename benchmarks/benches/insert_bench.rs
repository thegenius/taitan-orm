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
    let config = SqliteLocalConfig {
        work_dir: Cow::from("./workspace"),
        db_file: Cow::from(db_file),
    };
    let mut db: SqliteDatabase = SqliteBuilder::build(config).await?;
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

async fn taitan_insert(db: &SqliteDatabase, user: User) {
    db.insert(&user).await.unwrap();
}

fn bench_async_insert(c: &mut Criterion) {
    // 创建独立运行时
    let rt = Runtime::new().unwrap();
    let db = rt.block_on(async { init_db("test.db").await.unwrap() });
    let mut group = c.benchmark_group("Async Insert");
    group
        .warm_up_time(Duration::from_secs(5))
        .measurement_time(Duration::from_secs(20))
        .sample_size(10_000);
    let sony_flake = Sonyflake::new().unwrap();
    // 单条插入基准测试
    group.bench_function("taitan_insert", |b| {
        b.to_async(&rt).iter_batched(
            || User {
                id: sony_flake.next_id().unwrap() as i64,
                name: "test_user".to_string(),
                age: Some(23),
                birthday: Some(datetime!(2019-01-01 0:00)),
            },
            // |user| sqlx_insert(&db, user),
            |user| taitan_insert(&db, user),
            BatchSize::SmallInput,
        )
    });

    group.bench_function("sqlx_insert", |b| {
        b.to_async(&rt).iter_batched(
            || User {
                id: sony_flake.next_id().unwrap() as i64,
                name: "test_user".to_string(),
                age: Some(23),
                birthday: Some(datetime!(2019-01-01 0:00)),
            },
            |user| sqlx_insert(&db, user),
            // |user| taitan_insert(&db, user),
            BatchSize::SmallInput,
        )
    });



    // // 批量插入基准测试（使用事务）
    // group.bench_function("batch_insert(100)", |b| {
    //     b.to_async(&rt).iter_batched(
    //         || {
    //             (0..100).map(|i| (
    //                 Uuid::new_v4().to_string(),
    //                 format!("user_{}", i),
    //                 format!("user{}@test.com", i)
    //             )).collect::<Vec<_>>()
    //         },
    //         |records| async {
    //             let mut tx = pool.begin().await.unwrap();
    //             for (id, name, email) in records {
    //                 sqlx::query(
    //                     "INSERT INTO users (id, name, email) VALUES (?, ?, ?)"
    //                 )
    //                     .bind(&id)
    //                     .bind(&name)
    //                     .bind(&email)
    //                     .execute(&mut tx)
    //                     .await
    //                     .unwrap();
    //             }
    //             tx.commit().await.unwrap();
    //         },
    //         BatchSize::LargeInput
    //     )
    // });
    //
    // // 高并发测试（100个并行插入）
    // group.bench_function("concurrent_insert(100)", |b| {
    //     b.to_async(&rt).iter(|| async {
    //         let tasks: Vec<_> = (0..100).map(|i| {
    //             let pool = pool.clone();
    //             tokio::spawn(async move {
    //                 async_insert(
    //                     &pool,
    //                     Uuid::new_v4().to_string(),
    //                     format!("user_{}", i),
    //                     format!("user{}@test.com", i)
    //                 ).await
    //             })
    //         }).collect();
    //
    //         for task in tasks {
    //             task.await.unwrap();
    //         }
    //     });
    // });

    group.finish();
}

criterion_group!(benches, bench_async_insert);
criterion_main!(benches);
