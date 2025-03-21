use crate::sql_generic_executor::SqlGenericExecutor;
use sqlx::{Arguments, Database, Type};
use taitan_orm_trait::order::OrderBy;
use taitan_orm_trait::page::Pagination;
use taitan_orm_trait::result::Result;
use taitan_orm_trait::traits::{Entity, Location, Mutation, Parameter, Unique};

pub trait ArgsExtractor: SqlGenericExecutor
where
    for<'a> i64: sqlx::Encode<'a, <Self as SqlGenericExecutor>::DB>,i64: Type<<Self as SqlGenericExecutor>::DB>

{
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

    fn extract_location_with_page_arguments<'a>(
        location: &'a dyn Location<Self::DB>,
        page: &'a Pagination,
    ) -> Result<<Self::DB as Database>::Arguments<'a>> {
        tracing::debug!("extract_location_with_page_arguments location: {:?}", location);
        let mut args = <Self::DB as Database>::Arguments::default();
        location.add_to_args(&mut args)?;
        tracing::debug!("extract_location_with_page_arguments len of location: {}", args.len());
        <Pagination as Parameter<Self::DB>>::add_to_args(page, &mut args)?;
        tracing::debug!("extract_location_with_page_arguments len of location + page: {}", args.len());
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
