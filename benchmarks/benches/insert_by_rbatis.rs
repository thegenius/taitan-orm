use std::ptr::swap_nonoverlapping;
use std::time::Duration;
use criterion::{async_executor::AsyncExecutor, criterion_group, criterion_main, Criterion, BatchSize};
use rbatis::{crud, RBatis};
use rbdc_sqlite::SqliteDriver;
use time::PrimitiveDateTime;
use tokio::runtime::Runtime;
use sea_orm::PrimaryKeyTrait;
use serde::{Deserialize, Serialize};
use sonyflake::Sonyflake;
use time::macros::datetime;



#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "id")]
    pub id: i64,
    pub name: String,
    pub age: i32,
    pub birthday: PrimitiveDateTime,
}

crud!(User{});
// 初始化数据库连接
async fn setup_db() -> RBatis {
    let rb = RBatis::new();
    rb.init(SqliteDriver {},"sqlite://:memory:")
        .expect("Failed to connect to database");

    // 创建表
    let _ = rb
        .exec(
            r#"
        CREATE TABLE IF NOT EXISTS user (
            id BIGINT PRIMARY KEY,
            name VARCHAR(64) NOT NULL,
            age INTEGER NOT NULL,
            birthday DATETIME NOT NULL
        )"#,
            vec![],
        )
        .await;
    rb
}

// 生成测试用户
fn gen_user(sf: &Sonyflake) -> User {
    User {
        id: sf.next_id().unwrap() as i64,
        name: "test_user".to_string(),
        age: 23,
        birthday: datetime!(2019-01-01 0:00),
    }
}

// 单个插入操作
async fn insert_user(rb: &RBatis, user: User) {
    User::insert(rb, &user).await.expect("Insert failed");
}

// 基准测试组
fn bench_rbatis(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let rb = rt.block_on(setup_db());
    let sf = Sonyflake::new().unwrap();

    let mut group = c.benchmark_group("RBatis");
    group
        .warm_up_time(Duration::from_secs(5))
        .measurement_time(Duration::from_secs(20))
        .sample_size(10_000);
    // 单条插入基准测试
    group.bench_function("single_insert", |b| {
        b.to_async(&rt).iter_batched(
            || gen_user(&sf),
            |user| async {
                insert_user(&rb, user).await
            }
            ,
            BatchSize::SmallInput,
        )
    });

    group.finish();
}

criterion_group!(benches, bench_rbatis);
criterion_main!(benches);