use crate::entities::user3::{
    UserEntity, UserMutation, UserPrimary, UserSelectedEntity, UserSelection,
};
use taitan_orm::traits::Selection;
use taitan_orm::{Executor, SqlGenericExecutor, WriterApi};
use taitan_orm_trait::SelectedEntity;

async fn test_insert_user<EX>(db: &mut EX, user: &UserEntity) -> taitan_orm::Result<()>
where
    EX: Executor,
    UserSelectedEntity: SelectedEntity<<EX as SqlGenericExecutor>::DB>,
{
    let success = db.insert(user).await?;
    assert!(success);

    let selection = UserSelectedEntity::full_fields();
    let primary = UserPrimary {
        id: user.id.unwrap(),
    };
    let entity_opt: Option<UserSelectedEntity> = db.select(&selection, &primary).await?;
    assert!(entity_opt.is_some());

    let selected_entity = entity_opt.unwrap();
    assert_eq!(selected_entity.request_id.unwrap(), user.request_id);
    assert_eq!(selected_entity.name.unwrap(), user.name);
    assert_eq!(selected_entity.age.unwrap(), user.age.unwrap());
    assert_eq!(selected_entity.birthday.unwrap(), user.birthday.unwrap());
    Ok(())
}

async fn test_update_user<EX>(
    db: &mut EX,
    user_mutation: &UserMutation,
    user_primary: &UserPrimary,
) -> taitan_orm::Result<()>
where
    EX: Executor,
    UserSelectedEntity: SelectedEntity<<EX as SqlGenericExecutor>::DB>,
{
    let success = db.update(user_mutation, user_primary).await?;
    assert!(success);

    let selection = UserSelectedEntity::full_fields();
    let primary = UserPrimary {
        id: user_primary.id,
    };
    let entity_opt: Option<UserSelectedEntity> = db.select(&selection, &primary).await?;
    assert!(entity_opt.is_some());

    let selected_entity = entity_opt.unwrap();
    assert_eq!(selected_entity.request_id, user_mutation.request_id);
    assert_eq!(selected_entity.name, user_mutation.name);
    assert_eq!(selected_entity.age, user_mutation.age);
    assert_eq!(selected_entity.birthday, user_mutation.birthday);
    Ok(())
}

async fn test_upsert_user<EX>(db: &mut EX, user: &UserEntity) -> taitan_orm::Result<()>
where
    EX: Executor,
    UserSelectedEntity: SelectedEntity<<EX as SqlGenericExecutor>::DB>,
{
    let success = db.upsert(user).await?;
    assert!(success);

    let selection = UserSelectedEntity::full_fields();
    let user_primary: UserPrimary = UserPrimary { id: user.id.unwrap() };
    let entity_opt: Option<UserSelectedEntity> = db.select(&selection, &user_primary).await?;

    assert!(entity_opt.is_some());
    let selected_entity = entity_opt.unwrap();
    assert_eq!(selected_entity.request_id.unwrap(), user.request_id);
    assert_eq!(selected_entity.name.unwrap(), user.name);
    assert_eq!(selected_entity.age.unwrap(), user.age.unwrap());
    assert_eq!(selected_entity.birthday.unwrap(), user.birthday.unwrap());
    Ok(())
}
