//! | Rust type                             | MySQL/MariaDB type(s)                                |
//! |---------------------------------------|------------------------------------------------------|
//! | `bool`                                | TINYINT(1), BOOLEAN, BOOL (see below)                |
//! | `i8`                                  | TINYINT                                              |
//! | `i16`                                 | SMALLINT                                             |
//! | `i32`                                 | INT                                                  |
//! | `i64`                                 | BIGINT                                               |
//! | `u8`                                  | TINYINT UNSIGNED                                     |
//! | `u16`                                 | SMALLINT UNSIGNED                                    |
//! | `u32`                                 | INT UNSIGNED                                         |
//! | `u64`                                 | BIGINT UNSIGNED                                      |
//! | `f32`                                 | FLOAT                                                |
//! | `f64`                                 | DOUBLE                                               |
//! | `&str`, [`String`]                    | VARCHAR, CHAR, TEXT                                  |
//! | `&[u8]`, `Vec<u8>`                    | VARBINARY, BINARY, BLOB                              |
//! | `IpAddr`                              | VARCHAR, TEXT                                        |
//! | `Ipv4Addr`                            | INET4 (MariaDB-only), VARCHAR, TEXT                  |
//! | `Ipv6Addr`                            | INET6 (MariaDB-only), VARCHAR, TEXT                  |
//! | [`MySqlTime`]                         | TIME (encode and decode full range)                  |
//! | [`Duration`][std::time::Duration]     | TIME (for decoding positive values only)             |

//! | Rust type                             | MySQL/MariaDB type(s)                                |
//! |---------------------------------------|------------------------------------------------------|
//! | `time::PrimitiveDateTime`             | DATETIME                                             |
//! | `time::OffsetDateTime`                | TIMESTAMP                                            |
//! | `time::Date`                          | DATE                                                 |
//! | `time::Time`                          | TIME (time-of-day only)                              |
//! | `time::Duration`                      | TIME (decodes full range; see note for encoding)     |

//! | Rust type                             | MySQL/MariaDB type(s)                                |
//! |---------------------------------------|------------------------------------------------------|
//! | `bigdecimal::BigDecimal`              | DECIMAL                                              |

//! | Rust type                             | MySQL/MariaDB type(s)                                |
//! |---------------------------------------|------------------------------------------------------|
//! | `uuid::Uuid`                          | BINARY(16) (see note)                                |
//! | `uuid::fmt::Hyphenated`               | CHAR(36), VARCHAR, TEXT, UUID (MariaDB-only)         |
//! | `uuid::fmt::Simple`                   | CHAR(32), VARCHAR, TEXT                              |

use std::marker::PhantomData;
use taitan_orm_trait::{CmpOperator, LocationExpr, LocationTrait, Optional, Selection};
use taitan_orm_trait::{Entity, Location, Mutation, SelectedEntity, Unique, UpdateCommand};
use taitan_orm::SqlGenericExecutor;

use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqliteArguments;
use sqlx::types::Uuid;
use sqlx::{sqlx_macros, Database};

use sqlx::Arguments;
use sqlx::Row;
use taitan_orm::database::sqlite::{SqliteDatabase, SqliteLocalConfig};
use taitan_orm::SqlExecutor;
use time::macros::datetime;

use crate::entities::user::*;
use crate::setup::{get_test_mutex, setup_logger};

async fn test_insert_user(db: &mut SqliteDatabase, user: &User) -> taitan_orm::Result<()> {
    let pool = db.get_pool()?;
    let mut conn = pool.acquire().await?;

    let args = user.gen_insert_arguments_sqlite().unwrap();

    let result =  SqliteDatabase::generic_execute(&mut *conn, "INSERT INTO `user`(`id`, `request_id`, `name`, `age`, `birthday`) VALUES(?, ?, ?, ?, ?)",
                                args).await?;
    assert_eq!(result, 1);

    let mut selection = UserSelection::default();
    selection.request_id = true;
    selection.name = true;
    selection.age = true;
    selection.birthday = true;
    let primary = UserPrimary { id: user.id };
    let primary_args = primary.gen_unique_arguments_sqlite().unwrap();
    let entity_opt: Option<UserSelected> =  SqliteDatabase::generic_fetch_option(
            &mut *conn,
            "SELECT `request_id`, `name`, `age`, `birthday` FROM `user` WHERE `id` = ?",
            &selection,
            primary_args,
        )
        .await?;

    assert!(entity_opt.is_some());
    let selected_entity = entity_opt.unwrap();
    assert_eq!(selected_entity.request_id.unwrap(), user.request_id);
    assert_eq!(selected_entity.name.unwrap(), user.name);
    assert_eq!(selected_entity.age, user.age);
    assert_eq!(selected_entity.birthday, user.birthday);
    Ok(())
}

/**
因为UPDATE语句固定了，所以目前要求mutation必须包含所有字段
*/
async fn test_update_user(
    db: &mut SqliteDatabase,
    user_mutation: &UserMutation,
    user_primary: &UserPrimary,
) -> taitan_orm::Result<()> {
    let pool = db.get_pool()?;
    let mut conn = pool.acquire().await?;
    let update_command = UserPrimaryMutationPair(user_mutation, user_primary);
    let args = update_command.gen_update_arguments_sqlite().unwrap();
    // let args: SqliteArguments = user_primary
    //     .gen_update_arguments_sqlite(user_mutation)
    //     .unwrap();
    let result =  SqliteDatabase::generic_execute(&mut *conn, "UPDATE `user` SET `request_id` = ?, `name` = ?, `age` = ?, `birthday` = ? WHERE `id` = ?",
                                args).await?;
    assert_eq!(result, 1);

    let mut selection = UserSelection::default();
    selection.request_id = true;
    selection.name = true;
    selection.age = true;
    selection.birthday = true;

    let primary_args = user_primary.gen_unique_arguments_sqlite().unwrap();
    let entity_opt: Option<UserSelected> = SqliteDatabase::generic_fetch_option(
            &mut *conn,
            "SELECT `request_id`, `name`, `age`, `birthday` FROM `user` WHERE `id` = ?",
            &selection,
            primary_args,
        )
        .await?;

    assert!(entity_opt.is_some());
    let selected_entity = entity_opt.unwrap();
    assert_eq!(selected_entity.request_id, user_mutation.request_id);
    assert_eq!(selected_entity.name, user_mutation.name);
    assert_eq!(selected_entity.age, user_mutation.age);
    assert_eq!(selected_entity.birthday, user_mutation.birthday);
    Ok(())
}

async fn test_upsert_user(db: &mut SqliteDatabase, user: &User) -> taitan_orm::Result<()> {
    let pool = db.get_pool()?;
    let mut conn = pool.acquire().await?;
    let args: SqliteArguments = user.gen_upsert_arguments_sqlite().unwrap();
    let result =  SqliteDatabase::generic_execute(&mut *conn, "INSERT INTO `user`(`id`, `request_id`, `name`, `age`, `birthday`) VALUES (?, ?, ?, ?, ?)
ON CONFLICT (`id`) DO UPDATE SET
`request_id` = ?, `name` = ?, `age` = ?, `birthday` = ?", args).await?;
    assert_eq!(result, 1);

    let mut selection = UserSelection::default();
    selection.request_id = true;
    selection.name = true;
    selection.age = true;
    selection.birthday = true;

    let user_primary: UserPrimary = UserPrimary { id: user.id };
    let primary_args = user_primary.gen_unique_arguments_sqlite().unwrap();
    let entity_opt: Option<UserSelected> =  SqliteDatabase::generic_fetch_option(
            &mut *conn,
            "SELECT `request_id`, `name`, `age`, `birthday` FROM `user` WHERE `id` = ?",
            &selection,
            primary_args,
        )
        .await?;

    assert!(entity_opt.is_some());
    let selected_entity = entity_opt.unwrap();
    assert_eq!(selected_entity.request_id.unwrap(), user.request_id);
    assert_eq!(selected_entity.name.unwrap(), user.name);
    assert_eq!(selected_entity.age, user.age);
    assert_eq!(selected_entity.birthday, user.birthday);
    Ok(())
}

async fn test_delete_user(
    db: &mut SqliteDatabase,
    user_primary: &UserPrimary,
) -> taitan_orm::Result<()> {
    let pool = db.get_pool()?;
    let mut conn = pool.acquire().await?;
    let args: SqliteArguments = user_primary.gen_unique_arguments_sqlite().unwrap();
    let result =  SqliteDatabase::generic_execute(&mut *conn, "DELETE FROM `user` WHERE `id` = ?", args)
        .await?;
    assert_eq!(result, 1);

    let mut selection = UserSelection::default();
    selection.request_id = true;
    selection.name = true;
    selection.age = true;
    selection.birthday = true;

    let user_primary: UserPrimary = UserPrimary {
        id: user_primary.id,
    };
    let primary_args = user_primary.gen_unique_arguments_sqlite().unwrap();
    let entity_opt: Option<UserSelected> =  SqliteDatabase::generic_fetch_option(
            &mut *conn,
            "SELECT `request_id`, `name`, `age`, `birthday` FROM `user` WHERE `id` = ?",
            &selection,
            primary_args,
        )
        .await?;

    assert!(entity_opt.is_none());
    Ok(())
}

async fn test_select_all(db: &mut SqliteDatabase, expect_cnt: usize) -> taitan_orm::Result<()> {
    let pool = db.get_pool()?;
    let mut conn = pool.acquire().await?;

    let mut selection = UserSelection::default();
    selection.request_id = true;
    selection.name = true;
    selection.age = true;
    selection.birthday = true;

    let phantom = PhantomData::<SqliteArguments>::default();
    let entity_vec: Vec<UserSelected> =  SqliteDatabase::generic_fetch_all_plain(&mut *conn,
            "SELECT `request_id`, `name`, `age`, `birthday` FROM `user`",
            &selection,
            phantom,
        )
        .await?;

    assert_eq!(entity_vec.len(), expect_cnt);
    Ok(())
}

async fn test_select_location(
    db: &mut SqliteDatabase,
    user_location: &UserLocation,
    expect_cnt: usize,
) -> taitan_orm::Result<()> {
    let pool = db.get_pool()?;
    let mut conn = pool.acquire().await?;
    let loc_args: SqliteArguments = user_location.gen_location_arguments_sqlite().unwrap();

    let mut selection = UserSelection::default();
    selection.request_id = true;
    selection.name = true;
    selection.age = true;
    selection.birthday = true;

    let entities: Vec<UserSelected> =  SqliteDatabase::generic_fetch_all(
            &mut *conn,
            "SELECT `request_id`, `name`, `age`, `birthday` FROM `user` WHERE `birthday` = ?",
            &selection,
            loc_args,
        )
        .await?;

    assert_eq!(entities.len(), expect_cnt);
    Ok(())
}

/**
测试用例设计如下：
insert  id = 1
update  id = 1
upsert  id = 2

select all
select id = 2
delete id = 2
select all where id = 1
select all where id = 2
*/
#[sqlx_macros::test]
pub async fn sql_executor_spec() -> taitan_orm::Result<()> {
    let test_mutex = get_test_mutex();
    let test_lock = test_mutex.lock();
    setup_logger();

    let config = SqliteLocalConfig {
        work_dir: "./workspace".into(),
        db_file: "test.db".into(),
    };

    let mut db: SqliteDatabase = SqliteDatabase::build(config).await.unwrap();
    let pool = db.get_pool()?;
    let conn = pool.acquire().await.unwrap();
    prepare_user_table(&mut db).await?;

    let entity1 = User {
        id: 1,
        request_id: Uuid::new_v4(),
        name: "Allen".to_string(),
        age: Optional::Some(23),
        birthday: Optional::Some(datetime!(2019-01-01 0:00)),
    };

    test_insert_user(&mut db.clone(), &entity1).await?;

    let mutaion1 = UserMutation {
        request_id: Optional::Some(Uuid::new_v4()),
        name: Optional::Some("Allen Woods".to_string()),
        age: Optional::Some(25),
        birthday: Optional::Some(datetime!(2019-01-02 0:00)),
    };
    let primary1 = UserPrimary { id: 1 };
    test_update_user(&mut db.clone(), &mutaion1, &primary1).await?;

    let entity1 = User {
        id: 1,
        request_id: Uuid::new_v4(),
        name: "Bob".to_string(),
        age: Optional::Some(24),
        birthday: Optional::Some(datetime!(2020-01-03 12:59)),
    };
    test_upsert_user(&mut db.clone(), &entity1).await?;

    let entity1 = User {
        id: 2,
        request_id: Uuid::new_v4(),
        name: "Bob Woods".to_string(),
        age: Optional::Some(24),
        birthday: Optional::Some(datetime!(2020-01-01 0:00)),
    };
    test_insert_user(&mut db.clone(), &entity1).await?;
    test_select_all(&mut db.clone(), 2).await?;

    let user_primary2 = UserPrimary { id: 2 };
    test_delete_user(&mut db.clone(), &user_primary2).await?;
    test_select_all(&mut db.clone(), 1).await?;

    let user_location: UserLocation = UserLocation {
        request_id: Optional::None,
        name: Optional::None,
        age: Optional::None,
        birthday: Optional::Some(LocationExpr::new(
            CmpOperator::Eq,
            datetime!(2020-01-03 12:59),
        )),
    };
    test_select_location(&mut db.clone(), &user_location, 1).await?;

    Ok(())
}
