use taitan_orm_macro::{EntityNew, Parameter};
#[derive(Debug, Parameter, EntityNew)]
struct ParamSpec001 {
    a: String,
    b: i64,
}