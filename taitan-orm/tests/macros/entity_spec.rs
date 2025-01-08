
use sqlx::{sqlx_macros, Sqlite};
use time::PrimitiveDateTime;
use uuid::Uuid;
use taitan_orm::prelude::Schema;
use taitan_orm_trait::{Optional, SelectedEntity};

#[derive(Schema, Clone, Debug)]
#[table_name = "user"]
#[unique_key = "age"]
#[unique_key = "name, birthday"]
#[serde_struct = "primary"]
#[serde_struct = "unique"]
pub struct UserEntity {
    #[primary_key]
    #[auto_increment]
    pub id: Optional<i64>,

    #[field_name = "r_id"]
    pub request_id: Uuid,

    pub age: Optional<i32>,

    pub name: String,

    pub birthday: Optional<PrimitiveDateTime>,
}

fn check_is_selected_entity<DB: sqlx::Database, SE: SelectedEntity<DB>>(se: &SE) {}

#[sqlx_macros::test]
pub async fn entity_macro_spec() -> taitan_orm::result::Result<()> {

    // let selected = UserSelectedEntity::default();
    // check_is_selected_entity::<Sqlite, UserSelectedEntity>(&selected);

    Ok(())
}
