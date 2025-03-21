use taitan_orm_macro::Schema;
use time::PrimitiveDateTime;

// #[derive(Debug, Clone)]
#[derive(Debug, Clone, Schema)]
#[table(user)]
#[unique(uk_name=(name))]
#[index(idx_arg_birthday=(age, birthday))]
#[primary(id)]
pub struct Spec002 {
    id: i32,
    name: String,
    age: Option<i32>,
    birthday: Option<PrimitiveDateTime>,
}


