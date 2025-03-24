use crate::sql_generic_executor::SqlGenericExecutor;
use sqlx::sqlite::SqliteArguments;
use sqlx::{Arguments, Database, Sqlite, Type};
use taitan_orm_trait::order::OrderBy;
use taitan_orm_trait::page::Pagination;
use taitan_orm_trait::result::Result;
use taitan_orm_trait::traits::{Entity, Location, Mutation, Parameter, SqliteEntity, SqliteLocation, SqliteMutation, SqliteUnique, Unique};

pub struct SqliteArgsExtractor;
impl SqliteArgsExtractor {
    pub fn extract_pagination_arguments(page: &Pagination) -> Result<SqliteArguments<'_>> {
        Ok(<Pagination as Parameter<Sqlite>>::gen_args(page)?)
    }
    pub fn extract_unique_arguments<M: SqliteMutation>(
        unique: &dyn SqliteUnique<Mutation = M>,
    ) -> Result<<Sqlite as Database>::Arguments<'_>> {
        let mut args = <Sqlite as Database>::Arguments::default();
        unique.add_to_args(&mut args)?;
        Ok(args)
    }
    pub fn extract_location_arguments(
        location: &dyn Location<Sqlite>,
    ) -> Result<<Sqlite as Database>::Arguments<'_>> {
        let mut args = <Sqlite as Database>::Arguments::default();
        location.add_to_args(&mut args)?;
        Ok(args)
    }

    pub fn extract_location_with_page_arguments<'a>(
        location: &'a dyn SqliteLocation,
        page: &'a Pagination,
    ) -> Result<<Sqlite as Database>::Arguments<'a>> {
        tracing::debug!(
            "extract_location_with_page_arguments location: {:?}",
            location
        );
        let mut args = <Sqlite as Database>::Arguments::default();
        location.add_to_args(&mut args)?;
        tracing::debug!(
            "extract_location_with_page_arguments len of location: {}",
            args.len()
        );
        <Pagination as Parameter<Sqlite>>::add_to_args(page, &mut args)?;
        tracing::debug!(
            "extract_location_with_page_arguments len of location + page: {}",
            args.len()
        );
        Ok(args)
    }

    pub fn extract_insert_arguments(entity: &dyn SqliteEntity) -> Result<SqliteArguments<'_>> {
        entity.gen_args()
    }
    pub fn extract_upsert_arguments(
        entity: &dyn Entity<Sqlite>,
    ) -> Result<<Sqlite as Database>::Arguments<'_>> {
        entity.gen_args()
    }
    pub fn extract_update_arguments<'a, M: Mutation<Sqlite>>(
        mutation: &'a M,
        unique: &'a dyn Unique<Sqlite, Mutation = M>,
    ) -> Result<<Sqlite as Database>::Arguments<'a>> {
        let mut args = <Sqlite as Database>::Arguments::default();
        mutation.add_to_args(&mut args)?;
        unique.add_to_args(&mut args)?;
        Ok(args)
    }
    pub fn extract_change_arguments<'a>(
        mutation: &'a dyn Mutation<Sqlite>,
        location: &'a dyn Location<Sqlite>,
    ) -> Result<<Sqlite as Database>::Arguments<'a>> {
        let mut args = <Sqlite as Database>::Arguments::default();
        mutation.add_to_args(&mut args)?;
        location.add_to_args(&mut args)?;
        Ok(args)
    }
    pub fn extract_delete_arguments<M: Mutation<Sqlite>>(
        unique: &dyn Unique<Sqlite, Mutation = M>,
    ) -> Result<<Sqlite as Database>::Arguments<'_>> {
        let mut args = <Sqlite as Database>::Arguments::default();
        unique.add_to_args(&mut args)?;
        Ok(args)
    }
    pub fn extract_purify_arguments(
        location: &dyn Location<Sqlite>,
    ) -> Result<<Sqlite as Database>::Arguments<'_>> {
        let mut args = <Sqlite as Database>::Arguments::default();
        location.add_to_args(&mut args)?;
        Ok(args)
    }
}
