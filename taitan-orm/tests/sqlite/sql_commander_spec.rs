// use crate::entities::user::{UserPrimary, UserSelection};
use crate::entities::user::*;
use sqlx::sqlx_macros;
use taitan_orm::database::sqlite::{ SqliteDatabase, SqliteLocalConfig};
use taitan_orm::prelude:: { WriterApi, ReaderApi};
use time::macros::datetime;
use uuid::Uuid;

// use entities::user::*;
use taitan_orm_trait::{CmpOperator, LocationExpr, Optional, Selection};
use crate::setup::{get_test_mutex, setup_logger};

#[sqlx_macros::test]
pub async fn sql_commander_spec() -> taitan_orm::result::Result<()> {
    let test_mutex = get_test_mutex();
    let test_lock = test_mutex.lock();
    setup_logger();

    let config = SqliteLocalConfig {
        work_dir: "./workspace".into(),
        db_file: "test.db".into(),
    };
    let mut db: SqliteDatabase = SqliteDatabase::build(config).await?;
    prepare_user_table(&mut db).await?;

    let entity1 = User {
        id: 1,
        request_id: Uuid::new_v4(),
        name: "Allen".to_string(),
        age: Optional::Some(23),
        birthday: Optional::Some(datetime!(2019-01-01 0:00)),
    };
    test_insert_user(&mut db, &entity1).await?;

    let mutation1 = UserMutation {
        request_id: Optional::Some(Uuid::new_v4()),
        name: Optional::Some("Allen Woods".to_string()),
        age: Optional::Some(25),
        birthday: Optional::Some(datetime!(2019-01-02 0:00)),
    };
    let primary1 = UserPrimary { id: 1 };
    test_update_user(&mut db.clone(), &mutation1, &primary1).await?;

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
        age:Optional:: None,
        birthday: Optional::Some(LocationExpr::new(
            CmpOperator::Eq,
            datetime!(2020-01-03 12:59),
        )),
    };
    test_select_location(&mut db.clone(), &user_location, 1).await?;

    Ok(())
}

async fn test_insert_user(db: &mut SqliteDatabase, user: &User) -> taitan_orm::result::Result<()> {
    let success = db.insert(user).await?;
    assert!(success);

    let selection = UserSelected::full_fields();
    let primary = UserPrimary { id: user.id };
    let entity_opt: Option<UserSelected> = db.select(&selection, &primary).await?;
    assert!(entity_opt.is_some());

    let selected_entity = entity_opt.unwrap();
    assert_eq!(selected_entity.request_id.unwrap(), user.request_id);
    assert_eq!(selected_entity.name.unwrap(), user.name);
    assert_eq!(selected_entity.age.unwrap(), user.age.unwrap());
    assert_eq!(selected_entity.birthday.unwrap(), user.birthday.unwrap());
    Ok(())
}

async fn test_update_user(
    db: &mut SqliteDatabase,
    user_mutation: &UserMutation,
    user_primary: &UserPrimary,
) -> taitan_orm::result::Result<()> {
    let success = db.update(user_mutation, user_primary).await?;
    assert!(success);

    let selection = UserSelected::full_fields();
    let primary = UserPrimary {
        id: user_primary.id,
    };
    let entity_opt: Option<UserSelected> = db.select(&selection, &primary).await?;
    assert!(entity_opt.is_some());

    let selected_entity = entity_opt.unwrap();
    assert_eq!(selected_entity.request_id, user_mutation.request_id);
    assert_eq!(selected_entity.name, user_mutation.name);
    assert_eq!(selected_entity.age, user_mutation.age);
    assert_eq!(selected_entity.birthday, user_mutation.birthday);
    Ok(())
}

async fn test_upsert_user(db: &mut SqliteDatabase, user: &User) -> taitan_orm::result::Result<()> {
    let success = db.upsert(user).await?;
    assert!(success);

    let selection = UserSelected::full_fields();
    let user_primary: UserPrimary = UserPrimary { id: user.id };
    let entity_opt: Option<UserSelected> = db.select(&selection, &user_primary).await?;

    assert!(entity_opt.is_some());
    let selected_entity = entity_opt.unwrap();
    assert_eq!(selected_entity.request_id.unwrap(), user.request_id);
    assert_eq!(selected_entity.name.unwrap(), user.name);
    assert_eq!(selected_entity.age.unwrap(), user.age.unwrap());
    assert_eq!(selected_entity.birthday.unwrap(), user.birthday.unwrap());
    Ok(())
}

async fn test_delete_user(
    db: &mut SqliteDatabase,
    user_primary: &UserPrimary,
) -> taitan_orm::result::Result<()> {
    let success = db.delete(user_primary).await?;
    assert!(success);

    let entity_opt: Option<UserSelected> = db
        .select(&UserSelected::full_fields(), user_primary)
        .await?;
    assert!(entity_opt.is_none());
    Ok(())
}

async fn test_select_all(db: &mut SqliteDatabase, expect_cnt: usize) -> taitan_orm::result::Result<()> {
    let selection = UserSelected::full_fields();
    let order_by = UserOrderBy::build(["id", "name", "age", "birthday"])?;
    let entity_vec: Vec<UserSelected> = db.devour(&selection, &Some(&order_by), &None).await?;

    assert_eq!(entity_vec.len(), expect_cnt);
    Ok(())
}

async fn test_select_location(
    db: &mut SqliteDatabase,
    user_location: &UserLocation,
    expect_cnt: usize,
) -> taitan_orm::result::Result<()> {
    let order_by = UserOrderBy::build(["id", "name", "age", "birthday"])?;
    let entities: Vec<UserSelected> = db
        .search(&UserSelected::full_fields(), user_location, &Some(&order_by), &None)
        .await?;
    assert_eq!(entities.len(), expect_cnt);
    Ok(())
}
