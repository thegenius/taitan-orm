use sqlx::Database;
use taitan_orm_trait::{Entity, Location, Mutation, TemplateRecord, Unique};
use taitan_orm_trait::pagination::Pagination;
use crate::database::postgres::database::PostgresDatabase;
use crate::database::postgres::transaction::PostgresTransaction;
use crate::extractor::Extractor;

impl Extractor for PostgresDatabase {

    #[inline(always)]
    fn extract_pagination_arguments(page: &Pagination) -> crate::result::Result<<Self::DB as Database>::Arguments<'_>> {
        Ok(page.gen_page_arguments_postgres()?)
    }

    #[inline(always)]
    fn extract_unique_arguments<M: Mutation>(unique: &dyn Unique<Mutation=M>) -> crate::result::Result<<Self::DB as Database>::Arguments<'_>> {
        Ok(unique.gen_unique_arguments_postgres()?)
    }

    #[inline(always)]
    fn extract_location_arguments(location: &dyn Location) -> crate::result::Result<<Self::DB as Database>::Arguments<'_>> {
        Ok(location.gen_location_arguments_postgres()?)
    }

    #[inline(always)]
    fn extract_insert_arguments(entity: &dyn Entity) -> crate::result::Result<<Self::DB as Database>::Arguments<'_>> {
        Ok(entity.gen_insert_arguments_postgres()?)
    }

    #[inline(always)]
    fn extract_upsert_arguments(entity: &dyn Entity) -> crate::result::Result<<Self::DB as Database>::Arguments<'_>> {
        Ok(entity.gen_upsert_arguments_postgres()?)
    }

    #[inline(always)]
    fn extract_update_arguments<'a, M: Mutation>(mutation: &'a M, unique: &'a dyn Unique<Mutation=M>) -> crate::result::Result<<Self::DB as Database>::Arguments<'a>> {
        Ok(unique.gen_update_arguments_postgres(mutation)?)
    }

    #[inline(always)]
    fn extract_change_arguments<'a, M: Mutation>(mutation: &'a M, location: &'a M::Location) -> crate::result::Result<<Self::DB as Database>::Arguments<'a>> {
        Ok(mutation.gen_change_arguments_postgres(location)?)
    }

    #[inline(always)]
    fn extract_delete_arguments<M: Mutation>(unique: &dyn Unique<Mutation=M>) -> crate::result::Result<<Self::DB as Database>::Arguments<'_>> {
        Ok(unique.gen_unique_arguments_postgres()?)
    }

    #[inline(always)]
    fn extract_purify_arguments(location: &dyn Location) -> crate::result::Result<<Self::DB as Database>::Arguments<'_>> {
        Ok(location.gen_location_arguments_postgres()?)
    }

    #[inline(always)]
    fn extract_template_arguments(template: &dyn TemplateRecord) -> crate::result::Result<<Self::DB as Database>::Arguments<'_>> {
        Ok(template.gen_template_arguments_postgres()?)
    }

    #[inline(always)]
    fn extract_template_count_arguments(template: &dyn TemplateRecord) -> crate::result::Result<<Self::DB as Database>::Arguments<'_>> {
        Ok(template.gen_template_count_arguments_postgres()?)
    }
}

impl <'t> Extractor for PostgresTransaction<'t> {

    #[inline(always)]
    fn extract_pagination_arguments(page: &Pagination) -> crate::result::Result<<Self::DB as Database>::Arguments<'_>> {
        Ok(page.gen_page_arguments_postgres()?)
    }

    #[inline(always)]
    fn extract_unique_arguments<M: Mutation>(unique: &dyn Unique<Mutation=M>) -> crate::result::Result<<Self::DB as Database>::Arguments<'_>> {
        Ok(unique.gen_unique_arguments_postgres()?)
    }

    #[inline(always)]
    fn extract_location_arguments(location: &dyn Location) -> crate::result::Result<<Self::DB as Database>::Arguments<'_>> {
        Ok(location.gen_location_arguments_postgres()?)
    }

    #[inline(always)]
    fn extract_insert_arguments(entity: &dyn Entity) -> crate::result::Result<<Self::DB as Database>::Arguments<'_>> {
        Ok(entity.gen_insert_arguments_postgres()?)
    }

    #[inline(always)]
    fn extract_upsert_arguments(entity: &dyn Entity) -> crate::result::Result<<Self::DB as Database>::Arguments<'_>> {
        Ok(entity.gen_upsert_arguments_postgres()?)
    }

    #[inline(always)]
    fn extract_update_arguments<'a, M: Mutation>(mutation: &'a M, unique: &'a dyn Unique<Mutation=M>) -> crate::result::Result<<Self::DB as Database>::Arguments<'a>> {
        Ok(unique.gen_update_arguments_postgres(mutation)?)
    }

    #[inline(always)]
    fn extract_change_arguments<'a, M: Mutation>(mutation: &'a M, location: &'a M::Location) -> crate::result::Result<<Self::DB as Database>::Arguments<'a>> {
        Ok(mutation.gen_change_arguments_postgres(location)?)
    }

    #[inline(always)]
    fn extract_delete_arguments<M: Mutation>(unique: &dyn Unique<Mutation=M>) -> crate::result::Result<<Self::DB as Database>::Arguments<'_>> {
        Ok(unique.gen_unique_arguments_postgres()?)
    }

    #[inline(always)]
    fn extract_purify_arguments(location: &dyn Location) -> crate::result::Result<<Self::DB as Database>::Arguments<'_>> {
        Ok(location.gen_location_arguments_postgres()?)
    }

    #[inline(always)]
    fn extract_template_arguments(template: &dyn TemplateRecord) -> crate::result::Result<<Self::DB as Database>::Arguments<'_>> {
        Ok(template.gen_template_arguments_postgres()?)
    }

    #[inline(always)]
    fn extract_template_count_arguments(template: &dyn TemplateRecord) -> crate::result::Result<<Self::DB as Database>::Arguments<'_>> {
        Ok(template.gen_template_count_arguments_postgres()?)
    }
}