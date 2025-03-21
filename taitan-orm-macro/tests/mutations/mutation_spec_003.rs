use taitan_orm_macro::{Mutation, Parameter};

#[derive(Clone, Debug, Parameter, Mutation)]
pub struct SchemaSpec001Mutation {
    pub b: std::option::Option<i64>,
    pub c: std::option::Option<bool>,
    pub d: std::option::Option<i64>,
}


