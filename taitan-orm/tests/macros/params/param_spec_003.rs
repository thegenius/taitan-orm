use sqlx::Arguments;
use time::PrimitiveDateTime;

#[derive(
    Debug, Clone, taitan_orm :: macros :: Parameter, serde :: Serialize, serde :: Deserialize,
)]
pub enum Spec003 {
    Age {
        age: taitan_orm::op::Expr<i32>,
    },
    AgeBirthday {
        age: taitan_orm::op::Expr<i32>,
        birthday: taitan_orm::op::Expr<PrimitiveDateTime>,
    },
}

