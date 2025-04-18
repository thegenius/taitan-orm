
use time::PrimitiveDateTime;
use std::time::Duration;
use time::macros::datetime;

use sonyflake::Sonyflake;
use taitan_orm::database::sqlite::prelude::*;
use taitan_orm::prelude::*;
use rand::Rng;
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
use rand::rngs::ThreadRng;
use tokio::runtime::Runtime;
use tracing::info;

const RECORD_CNT: i32 = 10_000;
async fn init_db(db_file: &str, sony_flake: &Sonyflake) -> taitan_orm::result::Result<(SqliteDatabase, Vec<i64>)> {
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

    let mut id_list: Vec<i64> = Vec::new();
    for i in 0..RECORD_CNT {
        let new_id = sony_flake.next_id().unwrap() as i64;
        id_list.push(new_id);
        let new_user = gen_user(new_id);
        taitan_insert(&db, new_user).await;
    }

    Ok((db, id_list))
}

// async fn sqlx_insert(db: &SqliteDatabase, user: User) {
//     sqlx::query("INSERT INTO user (id, age, name, birthday) VALUES (?, ?, ?, ?)")
//         .bind(user.id)
//         .bind(user.age)
//         .bind(user.name)
//         .bind(user.birthday)
//         .execute(db.get_pool().unwrap())
//         .await
//         .unwrap();
// }

async fn taitan_insert(db: &SqliteDatabase, user: User) {
    db.insert(&user).await.unwrap();
}
async fn taitan_select(db: &SqliteDatabase, index_wrap: IndexWrap) {
    let selection = UserSelected::default();
    db.select(&selection, &UserPrimary {id: index_wrap.index}).await.unwrap();
}

fn random_select(id_list: &[i64], rng: &mut ThreadRng) -> i64 {
    let rand_num: i32 = rng.random_range(0..RECORD_CNT);
    id_list[rand_num as usize]
}


fn gen_user(new_id: i64) -> User {
    User {
        id: new_id,
        name: "test_user".to_string(),
        age: Some(23),
        birthday: Some(datetime!(2019-01-01 0:00)),
    }
}

#[derive(Clone)]
pub struct IndexWrap {
    pub index: i64
}

fn bench_async_insert(c: &mut Criterion) {
    // 创建独立运行时
    let rt = Runtime::new().unwrap();
    let sony_flake = Sonyflake::new().unwrap();
    let (db, id_list) = rt.block_on(async { init_db("test.db", &sony_flake).await.unwrap() });
    let mut rng = rand::rng();
    let mut group = c.benchmark_group("Async Select");
    group
        .warm_up_time(Duration::from_secs(5))
        .measurement_time(Duration::from_secs(20))
        .sample_size(10_000);

    group.bench_function("taitan_select", |b| {
        b.to_async(&rt).iter_batched(
            ||  {
                let index = rng.random_range(0..RECORD_CNT) as usize;
                IndexWrap{ index: id_list[index] }
            },
             |index_wrap|  async  {
                 taitan_select(&db, index_wrap).await;
             },
            BatchSize::SmallInput,
        )
    });

    // group.bench_function("sqlx_insert", |b| {
    //     b.to_async(&rt).iter_batched(
    //         || gen_user(&sony_flake),
    //         |user| sqlx_insert(&db, user),
    //         BatchSize::SmallInput,
    //     )
    // });





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
