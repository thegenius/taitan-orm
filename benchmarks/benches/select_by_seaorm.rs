use std::ptr::swap_nonoverlapping;
use std::time::Duration;
use criterion::{async_executor::AsyncExecutor, criterion_group, criterion_main, Criterion, BatchSize};
use rand::prelude::ThreadRng;
use rand::Rng;
use sea_orm::{Database, EntityTrait, DatabaseConnection, ActiveModelTrait, ActiveValue, ConnectionTrait, Set, ActiveModelBehavior, EnumIter, DeriveRelation};
use time::PrimitiveDateTime;
use tokio::runtime::Runtime;
use sea_orm::PrimaryKeyTrait;
use sonyflake::Sonyflake;
use time::macros::datetime;
use sea_orm::DerivePrimaryKey;
use taitan_orm::database::sqlite::api::ReaderApi;
use taitan_orm::database::sqlite::SqliteDatabase;

#[derive(Clone, Debug, PartialEq, Eq, sea_orm::DeriveEntityModel)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key)]
    id: i64,
    name: String,
    age: i32,
    birthday: PrimitiveDateTime
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

async fn setup_db(sony_flake: &Sonyflake) -> (DatabaseConnection, Vec<i64>) {
    let db = Database::connect("sqlite::memory:").await.unwrap();

    let stmt = sea_orm::Statement::from_string(
        sea_orm::DatabaseBackend::Sqlite,
        r#"CREATE TABLE IF NOT EXISTS `user`(`id` BIGINT PRIMARY KEY, `age` INT, `name` VARCHAR(64), `birthday` DATETIME)"#.to_string()
    );
    db.execute(stmt).await.unwrap();

    let mut id_list: Vec<i64> = Vec::new();
    for i in 0..RECORD_CNT {
        let new_id = sony_flake.next_id().unwrap() as i64;
        id_list.push(new_id);
        let new_user = gen_user(new_id);
        insert_single_user(&db, new_user).await;
    }

    (db, id_list)
}

fn gen_user(new_id: i64) -> ActiveModel {
    ActiveModel {
        id: Set(new_id.into()),
        name: Set("test_user".to_string()),
        age: Set(23),
        birthday: Set(datetime!(2019-01-01 0:00))
    }
}

async fn insert_single_user(db: &DatabaseConnection, model: ActiveModel) {
    Entity::insert(model)
        .exec(db)
        .await
        .unwrap();
}
const RECORD_CNT: i32 = 10_000;
#[derive(Clone)]
pub struct IndexWrap {
    pub index: i64
}

async fn seaorm_select(db: &DatabaseConnection, index_wrap: IndexWrap) {
    let model: Option<Model> = Entity::find_by_id(index_wrap.index).one(db).await.unwrap();
}

fn random_select(id_list: &[i64], rng: &mut ThreadRng) -> i64 {
    let rand_num: i32 = rng.random_range(0..RECORD_CNT);
    id_list[rand_num as usize]
}


fn bench_sea_orm(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let sony_flake = Sonyflake::new().unwrap();
    let (db, id_list) = rt.block_on(async { setup_db(&sony_flake).await });

    let mut group = c.benchmark_group("SeaORM");
    group
        .warm_up_time(Duration::from_secs(5))
        .measurement_time(Duration::from_secs(20))
        .sample_size(10_000);

    let mut rng = rand::rng();
    // 单条插入测试
    group.bench_function("single_insert", |b| {
        b.to_async(&rt).iter_batched(
            ||  {
                let index = rng.random_range(0..RECORD_CNT) as usize;
                IndexWrap{ index: id_list[index] }
            },
            |index_wrap|  async  {
                seaorm_select(&db, index_wrap).await;
            },
            BatchSize::SmallInput
        )
    });
    group.finish();
}

criterion_group!(benches, bench_sea_orm);
criterion_main!(benches);