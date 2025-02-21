

use crate::{change_fn, delete_fn, insert_fn, purify_fn, update_fn, upsert_fn, Result};
use crate::{SqlExecutor, SqlGenerator};


use crate::sql_generator_container::SqlGeneratorContainer;
use sqlx::Postgres;

use taitan_orm_trait::{
    Entity, Location, Mutation, Unique,
};


pub trait PostgresWriteCommander: SqlExecutor<DB = Postgres> + SqlGeneratorContainer {

    insert_fn!(PgArguments, Entity::gen_insert_arguments_postgres);

    upsert_fn!(PgArguments, Entity::gen_upsert_arguments_postgres);

    update_fn!(PgArguments, Unique::gen_update_arguments_postgres);

    change_fn!(PgArguments, Mutation::gen_change_arguments_postgres);

    delete_fn!(PgArguments, Unique::gen_unique_arguments_postgres);

    purify_fn!(PgArguments, Location::gen_location_arguments_postgres);
}