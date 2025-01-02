
use sqlx::sqlx_macros;
use time::PrimitiveDateTime;
use uuid::Uuid;
use taitan_orm::Schema;
use taitan_orm_trait::Optional;

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

#[sqlx_macros::test]
pub async fn entity_macro_spec() -> taitan_orm::Result<()> {
    Ok(())
}
