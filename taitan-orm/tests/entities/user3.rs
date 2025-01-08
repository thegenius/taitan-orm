use taitan_orm_trait::Optional;
use time::PrimitiveDateTime;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct UserEntity {
    pub id: Optional<i64>,

    pub request_id: Uuid,

    pub age: Optional<i32>,

    pub name: String,

    pub birthday: Optional<PrimitiveDateTime>,
}
