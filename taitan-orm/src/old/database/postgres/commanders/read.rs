
use crate::{count_all_fn, count_fn, devour_fn, devour_paged_fn, exists_fn, search_fn, search_paged_fn, select_fn};
use crate::{CountResult, Result};
use crate::{SqlExecutor, SqlGenerator, SqlGeneratorContainer};
use taitan_orm_trait::{
    Location, Mutation, OrderBy, SelectedEntity, Selection, Unique,
};



pub trait PostgresReadCommander: SqlExecutor<DB = sqlx::Postgres> + SqlGeneratorContainer {

    exists_fn!(PgArguments, Unique::gen_unique_arguments_postgres);

    count_fn!(PgArguments, Location::gen_location_arguments_postgres);

    count_all_fn!(PgArguments);

    select_fn!(Unique::gen_unique_arguments_postgres);

    search_fn!(Location::gen_location_arguments_postgres);

    search_paged_fn!(Location::gen_location_arguments_postgres);

    devour_fn!(crate::page::Pagination::gen_page_arguments_postgres);

    devour_paged_fn!(crate::page::Pagination::gen_page_arguments_postgres);
}
