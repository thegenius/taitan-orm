
use crate::{change_fn, delete_fn, insert_fn, purify_fn, update_fn, upsert_fn, Result};
use crate::{SqlExecutor, SqlGenerator};

use crate::sql_generator_container::SqlGeneratorContainer;
use sqlx::MySql;

use taitan_orm_trait::{
    Entity, Location, Mutation, Unique,
};

pub trait MySqlWriteCommander: SqlExecutor<DB = MySql> + SqlGeneratorContainer {

    insert_fn!(MySqlArguments, Entity::gen_insert_arguments_mysql);

    upsert_fn!(MySqlArguments, Entity::gen_upsert_arguments_mysql);

    update_fn!(MySqlArguments, Unique::gen_update_arguments_mysql);

    change_fn!(MySqlArguments, Mutation::gen_change_arguments_mysql);

    delete_fn!(MySqlArguments, Unique::gen_unique_arguments_mysql);

    purify_fn!(MySqlArguments, Location::gen_location_arguments_mysql);
}