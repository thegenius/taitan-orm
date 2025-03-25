use taitan_orm_trait::traits::{PostgresEntity, PostgresSelected, PostgresUnique, PostgresLocation, PostgresMutation};
use crate::sql_generator_impl;
use taitan_orm_trait::order::OrderBy;
use taitan_orm_trait::page::Pagination;

pub struct PostgresSqlGenerator;

impl PostgresSqlGenerator {
    sql_generator_impl!(
        Postgres,
        PostgresEntity,
        PostgresSelected,
        PostgresUnique,
        PostgresLocation,
        PostgresMutation
    );
}