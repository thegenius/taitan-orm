use taitan_orm_macro::{MutationNew, Parameter};

#[derive(Clone, Debug, Parameter, MutationNew)]
pub struct SchemaSpec001Mutation {
    pub b: std::option::Option<i64>,
    pub c: std::option::Option<bool>,
    pub d: std::option::Option<i64>,
}


