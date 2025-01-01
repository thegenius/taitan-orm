use sqlx::Database;
use taitan_orm_trait::{Entity, Location, Mutation, TemplateRecord, Unique};
use taitan_orm_trait::pagination::Pagination;
use crate::database::sqlite::SqliteDatabase;
use crate::extractor::Extractor;

impl Extractor for SqliteDatabase {

    #[inline(always)]
    fn extract_pagination_arguments(page: &Pagination) -> crate::Result<<Self::DB as Database>::Arguments<'_>> {
        Ok(page.gen_page_arguments_sqlite()?)
    }

    #[inline(always)]
    fn extract_unique_arguments<M: Mutation>(unique: &dyn Unique<Mutation=M>) -> crate::Result<<Self::DB as Database>::Arguments<'_>> {
        Ok(unique.gen_unique_arguments_sqlite()?)
    }

    #[inline(always)]
    fn extract_location_arguments(location: &dyn Location) -> crate::Result<<Self::DB as Database>::Arguments<'_>> {
        Ok(location.gen_location_arguments_sqlite()?)
    }

    #[inline(always)]
    fn extract_insert_arguments(entity: &dyn Entity) -> crate::Result<<Self::DB as Database>::Arguments<'_>> {
        Ok(entity.gen_insert_arguments_sqlite()?)
    }

    #[inline(always)]
    fn extract_upsert_arguments(entity: &dyn Entity) -> crate::Result<<Self::DB as Database>::Arguments<'_>> {
        Ok(entity.gen_upsert_arguments_sqlite()?)
    }

    #[inline(always)]
    fn extract_update_arguments<'a, M: Mutation>(mutation: &'a M, unique: &'a dyn Unique<Mutation=M>) -> crate::Result<<Self::DB as Database>::Arguments<'a>> {
        Ok(unique.gen_update_arguments_sqlite(mutation)?)
    }

    #[inline(always)]
    fn extract_change_arguments<'a, M: Mutation>(mutation: &'a M, location: &'a M::Location) -> crate::Result<<Self::DB as Database>::Arguments<'a>> {
        Ok(mutation.gen_change_arguments_sqlite(location)?)
    }

    #[inline(always)]
    fn extract_delete_arguments<M: Mutation>(unique: &dyn Unique<Mutation=M>) -> crate::Result<<Self::DB as Database>::Arguments<'_>> {
        Ok(unique.gen_unique_arguments_sqlite()?)
    }

    #[inline(always)]
    fn extract_purify_arguments(location: &dyn Location) -> crate::Result<<Self::DB as Database>::Arguments<'_>> {
        Ok(location.gen_location_arguments_sqlite()?)
    }

    #[inline(always)]
    fn extract_template_arguments(template: &dyn TemplateRecord) -> crate::Result<<Self::DB as Database>::Arguments<'_>> {
        Ok(template.gen_template_arguments_sqlite()?)
    }

    #[inline(always)]
    fn extract_template_count_arguments(template: &dyn TemplateRecord) -> crate::Result<<Self::DB as Database>::Arguments<'_>> {
        Ok(template.gen_template_count_arguments_sqlite()?)
    }
}