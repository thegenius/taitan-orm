// use sqlx::sqlx_macros;
// use taitan_orm::database::sqlite::{
//       SqliteDatabase, SqliteLocalConfig,
// };
// use taitan_orm::prelude:: ReaderMutApi;
// use taitan_orm::prelude::WriterMutApi;
// use taitan_orm::prelude::ReaderApi;
// use taitan_orm::prelude::WriterApi;
// use time::macros::datetime;
// use uuid::Uuid;
//
// use crate::entities::user::*;
// use taitan_orm_trait::{Optional, Selection};
// use crate::entities::user::prepare_user_table;
// use crate::setup::{get_test_mutex, setup_logger};
//
// #[sqlx_macros::test]
// pub async fn transaction_spec() -> taitan_orm::result::Result<()> {
//     let test_mutex = get_test_mutex();
//     let test_lock = test_mutex.lock();
//     setup_logger();
//
//     let config = SqliteLocalConfig {
//         work_dir: "./workspace".into(),
//         db_file: "test.db".into(),
//     };
//     let mut db: SqliteDatabase = SqliteDatabase::build(config).await?;
//     prepare_user_table(&mut db).await?;
//     let user1 = User {
//         id: 1,
//         request_id: Uuid::new_v4(),
//         name: "Allen".to_string(),
//         age: Optional::Some(23),
//         birthday: Optional::Some(datetime!(2019-01-01 0:00)),
//     };
//     let user2 = User {
//         id: 2,
//         request_id: Uuid::new_v4(),
//         name: "Bob".to_string(),
//         age: Optional::Some(24),
//         birthday: Optional::Some(datetime!(2019-01-01 0:00)),
//     };
//     test_insert_user(&mut db, &user1, &user2).await?;
//
//     let user3 = User {
//         id: 3,
//         request_id: Uuid::new_v4(),
//         name: "Allen".to_string(),
//         age: Optional::Some(24),
//         birthday: Optional::Some(datetime!(2019-01-01 0:00)),
//     };
//     let user4 = User {
//         id: 3,
//         request_id: Uuid::new_v4(),
//         name: "Bob".to_string(),
//         age: Optional::Some(24),
//         birthday: Optional::Some(datetime!(2019-01-01 0:00)),
//     };
//     test_rollback(&mut db, &user3, &user4).await?;
//
//     Ok(())
// }
//
// async fn insert_user_transactional(
//     db: &mut SqliteDatabase,
//     user1: &User,
//     user2: &User,
// ) -> taitan_orm::result::Result<()> {
//     let mut trx = db.transaction().await?;
//     if let Err(err) = trx.insert(user1).await {
//         trx.rollback().await?;
//         return Err(err);
//     }
//     if let Err(err) = trx.insert(user2).await {
//         trx.rollback().await?;
//         return Err(err);
//     }
//     trx.commit().await?;
//     Ok(())
// }
//
// async fn test_insert_user(
//     db: &mut SqliteDatabase,
//     user1: &User,
//     user2: &User,
// ) -> taitan_orm::result::Result<()> {
//     let result = insert_user_transactional(db, user1, user2).await;
//     assert!(result.is_ok());
//
//     let selection = UserSelected::full_fields();
//     let primary = UserPrimary { id: user1.id };
//     let entity_opt: Option<UserSelected> = db.select(&selection, &primary).await?;
//     assert!(entity_opt.is_some());
//
//     let primary = UserPrimary { id: user2.id };
//     let entity_opt: Option<UserSelected> = db.select(&selection, &primary).await?;
//     assert!(entity_opt.is_some());
//
//     Ok(())
// }
//
// async fn test_rollback(
//     db: &mut SqliteDatabase,
//     user1: &User,
//     user2: &User,
// ) -> taitan_orm::result::Result<()> {
//     let result = insert_user_transactional(db, user1, user2).await;
//     assert!(result.is_err());
//
//     let selection = UserSelected::full_fields();
//     let primary = UserPrimary { id: user1.id };
//     let entity_opt: Option<UserSelected> = db.select(&selection, &primary).await?;
//     assert!(entity_opt.is_none());
//
//     let primary = UserPrimary { id: user2.id };
//     let entity_opt: Option<UserSelected> = db.select(&selection, &primary).await?;
//     assert!(entity_opt.is_none());
//
//     Ok(())
// }
