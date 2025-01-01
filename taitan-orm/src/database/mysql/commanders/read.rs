

use crate::{count_all_fn, count_fn, devour_fn, devour_paged_fn, exists_fn, search_fn, search_paged_fn, select_fn};
use crate::{SqlExecutor, SqlGenerator, SqlGeneratorContainer};


use taitan_orm_trait::{
    Entity, Location, Mutation, OrderBy, SelectedEntity, Selection, TemplateRecord, Unique,
};
use crate::{CountResult, Result};


pub trait MySqlReadCommander: SqlExecutor<DB = sqlx::MySql> + SqlGeneratorContainer {

    exists_fn!(MySqlArguments, Unique::gen_unique_arguments_mysql);

    count_fn!(MySqlArguments, Location::gen_location_arguments_mysql);

    count_all_fn!(MySqlArguments);

    select_fn!(Unique::gen_unique_arguments_mysql);

    search_fn!(Location::gen_location_arguments_mysql);

    search_paged_fn!(Location::gen_location_arguments_mysql);

    devour_fn!(crate::page::Pagination::gen_page_arguments_mysql);

    devour_paged_fn!(crate::page::Pagination::gen_page_arguments_mysql);
}
