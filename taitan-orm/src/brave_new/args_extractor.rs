use sqlx::Database;

use crate::brave_new::sql_generic_executor::SqlGenericExecutor;
use taitan_orm_trait::brave_new::result::Result;
use taitan_orm_trait::brave_new::{Entity, Location, Mutation, Pagination, Template, Unique};

pub trait ArgsExtractor: SqlGenericExecutor {
    fn extract_pagination_arguments(
        page: &Pagination,
    ) -> Result<<Self::DB as Database>::Arguments<'_>>;
    fn extract_unique_arguments<M: Mutation<Self::DB>>(
        unique: &dyn Unique<Self::DB, Mutation = M>,
    ) -> Result<<Self::DB as Database>::Arguments<'_>> {
        let mut args = <Self::DB as Database>::Arguments::default();
        unique.add_to_args(&mut args)?;
        Ok(args)
    }
    fn extract_location_arguments(
        location: &dyn Location<Self::DB>,
    ) -> Result<<Self::DB as Database>::Arguments<'_>> {
        let mut args = <Self::DB as Database>::Arguments::default();
        location.add_to_args(&mut args)?;
        Ok(args)
    }
    fn extract_insert_arguments(
        entity: &dyn Entity<Self::DB>,
    ) -> Result<<Self::DB as Database>::Arguments<'_>> {
        entity.gen_args()
    }
    fn extract_upsert_arguments(
        entity: &dyn Entity<Self::DB>,
    ) -> Result<<Self::DB as Database>::Arguments<'_>> {
        entity.gen_args()
    }
    fn extract_update_arguments<'a, M: Mutation<Self::DB>>(
        mutation: &'a M,
        unique: &'a dyn Unique<Self::DB, Mutation = M>,
    ) -> Result<<Self::DB as Database>::Arguments<'a>> {
        let mut args = <Self::DB as Database>::Arguments::default();
        mutation.add_to_args(&mut args)?;
        unique.add_to_args(&mut args)?;
        Ok(args)
    }
    fn extract_change_arguments<'a>(
        mutation: &'a dyn Mutation<Self::DB>,
        location: &'a dyn Location<Self::DB>,
    ) -> Result<<Self::DB as Database>::Arguments<'a>> {
        let mut args = <Self::DB as Database>::Arguments::default();
        mutation.add_to_args(&mut args)?;
        location.add_to_args(&mut args)?;
        Ok(args)
    }
    fn extract_delete_arguments<M: Mutation<Self::DB>>(
        unique: &dyn Unique<Self::DB, Mutation = M>,
    ) -> Result<<Self::DB as Database>::Arguments<'_>> {
        let mut args = <Self::DB as Database>::Arguments::default();
        unique.add_to_args(&mut args)?;
        Ok(args)
    }
    fn extract_purify_arguments(
        location: &dyn Location<Self::DB>,
    ) -> Result<<Self::DB as Database>::Arguments<'_>> {
        let mut args = <Self::DB as Database>::Arguments::default();
        location.add_to_args(&mut args)?;
        Ok(args)
    }
    // fn extract_template_arguments(
    //     template: &dyn Template<Self::DB>,
    // ) -> Result<(String, <Self::DB as Database>::Arguments<'_>)> {
    //     // let mut args = <Self::DB as Database>::Arguments::default();
    //     template.get_sql()
    // }
    // fn extract_template_count_arguments(
    //     template: &dyn Template<Self::DB>,
    // ) -> Result<<Self::DB as Database>::Arguments<'_>> {
    //     template.gen_count_arguments()
    // }
}
