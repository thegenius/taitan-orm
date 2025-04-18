use criterion::{
    BatchSize, Criterion, async_executor::AsyncExecutor, criterion_group, criterion_main,
};
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

#[derive(Clone, Debug, PartialEq, Eq, sea_orm::DeriveEntityModel)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key)]
    id: i64,
    name: String,
    age: i32,
    birthday: PrimitiveDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

async fn setup_db() -> DatabaseConnection {
    let db = Database::connect("sqlite::memory:").await.unwrap();

    let stmt = sea_orm::Statement::from_string(
        sea_orm::DatabaseBackend::Sqlite,
        r#"CREATE TABLE IF NOT EXISTS `user`(`id` BIGINT PRIMARY KEY, `age` INT, `name` VARCHAR(64), `birthday` DATETIME)"#.to_string()
    );
    db.execute(stmt).await.unwrap();
    db
}

fn gen_user(sony_flake: &Sonyflake) -> ActiveModel {
    ActiveModel {
        id: Set(sony_flake.next_id().unwrap() as i64),
        name: Set("test_user".to_string()),
        age: Set(23),
        birthday: Set(datetime!(2019-01-01 0:00)),
    }
}

async fn insert_single_user(db: &DatabaseConnection, model: ActiveModel) {
    Entity::insert(model).exec(db).await.unwrap();
}

fn bench_sea_orm(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let db = rt.block_on(async { setup_db().await });

    let mut group = c.benchmark_group("SeaORM");
    group
        .warm_up_time(Duration::from_secs(5))
        .measurement_time(Duration::from_secs(20))
        .sample_size(10_000);
    let sony_flake = Sonyflake::new().unwrap();
    // 单条插入测试
    group.bench_function("single_insert", |b| {
        b.to_async(&rt).iter_batched(
            || gen_user(&sony_flake),
            |user| async {
                insert_single_user(&db, user).await;
            },
            BatchSize::SmallInput,
        )
    });

    group.finish();
}

criterion_group!(benches, bench_sea_orm);
criterion_main!(benches);
