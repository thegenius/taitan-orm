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
use time::PrimitiveDateTime;
use time::macros::datetime;
use tokio::runtime::Runtime;
use taitan_orm::database::sqlite::{SqliteBuilder, SqliteDatabase};
use taitan_orm::prelude::SqlExecutor;

#[derive(sqlx::FromRow)]
pub struct User {
    id: i64,
    name: String,
    age: Option<i32>,
    birthday: Option<PrimitiveDateTime>,
}

async fn setup_db(sony_flake: &Sonyflake) -> (SqliteDatabase, Vec<i64>) {
    // let db = Database::connect("sqlite::memory:").await.unwrap();
    let mut db: SqliteDatabase = SqliteBuilder::build_mem().await.unwrap();
    db.execute_plain("DROP TABLE IF EXISTS `user`").await.unwrap();
    db.execute_plain(
        "CREATE TABLE IF NOT EXISTS `user`(`id` BIGINT PRIMARY KEY, `age` INT, `name` VARCHAR(64), `birthday` DATETIME)",
    )
        .await.unwrap();
    // let stmt = sea_orm::Statement::from_string(
    //     sea_orm::DatabaseBackend::Sqlite,
    //     r#"CREATE TABLE IF NOT EXISTS `user`(`id` BIGINT PRIMARY KEY, `age` INT, `name` VARCHAR(64), `birthday` DATETIME)"#.to_string()
    // );
    // db.execute(stmt).await.unwrap();

    let mut id_list: Vec<i64> = Vec::new();
    for i in 0..RECORD_CNT {
        let new_id = sony_flake.next_id().unwrap() as i64;
        id_list.push(new_id);
        let new_user = gen_user(new_id);
        insert_single_user(&db, new_user).await;
    }

    (db, id_list)
}

fn gen_user(new_id: i64) -> User {
    User {
        id: new_id,
        name: "test_user".to_string(),
        age: Some(23),
        birthday: Some(datetime!(2019-01-01 0:00)),
    }
}

async fn insert_single_user(db: &SqliteDatabase, user: User) {
    sqlx::query("INSERT INTO user (id, age, name, birthday) VALUES (?, ?, ?, ?)")
        .bind(user.id)
        .bind(user.age)
        .bind(user.name)
        .bind(user.birthday)
        .execute(db.get_pool().unwrap())
        .await
        .unwrap();
}

const RECORD_CNT: i32 = 10_000;
#[derive(Clone)]
pub struct IndexWrap {
    pub index: i64,
}

async fn sqlx_select(db: &SqliteDatabase, index_wrap: IndexWrap) {
    sqlx::query_as::<_, User>("SELECT * FROM user WHERE id = ?")
    .bind(index_wrap.index).fetch(db.get_pool().unwrap());
}

fn random_select(id_list: &[i64], rng: &mut ThreadRng) -> i64 {
    let rand_num: i32 = rng.random_range(0..RECORD_CNT);
    id_list[rand_num as usize]
}

fn bench_sea_orm(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let sony_flake = Sonyflake::new().unwrap();
    let (mut db, id_list) = rt.block_on(async { setup_db(&sony_flake).await });

    let mut group = c.benchmark_group("Sqlx Select");
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
                sqlx_select(&db, index_wrap).await;
            },
            BatchSize::SmallInput,
        )
    });
    group.finish();
}

criterion_group!(benches, bench_sea_orm);
criterion_main!(benches);
