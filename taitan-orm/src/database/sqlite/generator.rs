

use crate::sql_generator_impl;
use taitan_orm_trait::order::OrderBy;
use taitan_orm_trait::page::Pagination;
use taitan_orm_trait::traits::{
    Entity, Location, Mutation, Selected, SqliteEntity, SqliteLocation, SqliteMutation,
    SqliteSelected, SqliteUnique,
};


pub struct SqliteSqlGenerator;

impl SqliteSqlGenerator {
    sql_generator_impl!(
        Sqlite,
        SqliteEntity,
        SqliteSelected,
        SqliteUnique,
        SqliteLocation,
        SqliteMutation
    );
}
