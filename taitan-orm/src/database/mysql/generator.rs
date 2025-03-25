use taitan_orm_trait::traits::{MySqlEntity, MySqlSelected, MySqlUnique, MySqlLocation, MySqlMutation};
use crate::sql_generator_impl;
use taitan_orm_trait::order::OrderBy;
use taitan_orm_trait::page::Pagination;

pub struct MySqlSqlGenerator;

impl MySqlSqlGenerator {
    sql_generator_impl!(
        MySql,
        MySqlEntity,
        MySqlSelected,
        MySqlUnique,
        MySqlLocation,
        MySqlMutation
    );
}