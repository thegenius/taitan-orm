use criterion::{
    BatchSize, Criterion, async_executor::AsyncExecutor, criterion_group, criterion_main,
};
use rand::Rng;
use rand::prelude::ThreadRng;
use sea_orm::DerivePrimaryKey;
use sea_orm::PrimaryKeyTrait;
use sea_orm::{
    ActiveModelBehavior, ActiveModelTrait, ActiveValue, ConnectionTrait, Database,
    DatabaseConnection, DeriveRelation, EntityTrait, EnumIter, Set,
};
use sonyflake::Sonyflake;
use std::time::Duration;
use rbatis::{crud, impl_select, RBatis};
use rbdc_sqlite::SqliteDriver;
use serde::{Deserialize, Serialize};
use time::PrimitiveDateTime;
use time::macros::datetime;
use tokio::runtime::Runtime;
use taitan_orm::database::sqlite::{SqliteBuilder, SqliteDatabase};
use taitan_orm::prelude::SqlExecutor;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "id")]
    pub id: i64,
    pub name: String,
    pub age: i32,
    pub birthday: PrimitiveDateTime,
}

crud!(User {});
impl_select!(User{ select_by_id(id: i64) -> Option => "`where id = #{id} limit 1`"});
async fn setup_db(sony_flake: &Sonyflake) -> (RBatis, Vec<i64>) {
    let rb = RBatis::new();
    rb.init(SqliteDriver {}, "sqlite://:memory:")
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

    let mut id_list: Vec<i64> = Vec::new();
    for i in 0..RECORD_CNT {
        let new_id = sony_flake.next_id().unwrap() as i64;
        id_list.push(new_id);
        let new_user = gen_user(new_id);
        insert_single_user(&rb, new_user).await;
    }

    (rb, id_list)
}

fn gen_user(new_id: i64) -> User {
    User {
        id: new_id,
        name: "test_user".to_string(),
        age: 23,
        birthday: datetime!(2019-01-01 0:00),
    }
}

async fn insert_single_user(rb: &RBatis, user: User) {
    User::insert(rb, &user).await.expect("Insert failed");
}

const RECORD_CNT: i32 = 10_000;
#[derive(Clone)]
pub struct IndexWrap {
    pub index: i64,
}

async fn rbatis_select(rb: &RBatis, index_wrap: IndexWrap) {
   User::select_by_id(rb, index_wrap.index).await.expect("Select failed");
}

fn random_select(id_list: &[i64], rng: &mut ThreadRng) -> i64 {
    let rand_num: i32 = rng.random_range(0..RECORD_CNT);
    id_list[rand_num as usize]
}

fn bench_sea_orm(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let sony_flake = Sonyflake::new().unwrap();
    let (mut rb, id_list) = rt.block_on(async { setup_db(&sony_flake).await });

    let mut group = c.benchmark_group("SeaORM");
    group
        .warm_up_time(Duration::from_secs(5))
        .measurement_time(Duration::from_secs(20))
        .sample_size(10_000);

    let mut rng = rand::rng();
    // 单条插入测试
    group.bench_function("single_insert", |b| {
        b.to_async(&rt).iter_batched(
            || {
                let index = rng.random_range(0..RECORD_CNT) as usize;
                IndexWrap {
                    index: id_list[index],
                }
            },
            |index_wrap| async {
                rbatis_select(&rb, index_wrap).await;
            },
            BatchSize::SmallInput,
        )
    });
    group.finish();
}

criterion_group!(benches, bench_sea_orm);
criterion_main!(benches);
