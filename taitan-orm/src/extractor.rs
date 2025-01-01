use sqlx::Database;

use taitan_orm_trait::{Entity, Location, Mutation, TemplateRecord, Unique};
use taitan_orm_trait::pagination::Pagination;
use crate::SqlGenericExecutor;
use crate::Result;

pub trait Extractor: SqlGenericExecutor {
    fn extract_pagination_arguments(page: &Pagination)-> Result<<Self::DB as Database>::Arguments<'_>>;
    fn extract_unique_arguments<M: Mutation>(unique: &dyn Unique<Mutation = M>) -> Result<<Self::DB as Database>::Arguments<'_>>;
    fn extract_location_arguments(location: &dyn Location) -> Result<<Self::DB as Database>::Arguments<'_>>;
    fn extract_insert_arguments(entity: &dyn Entity) -> Result<<Self::DB as Database>::Arguments<'_>>;
    fn extract_upsert_arguments(entity: &dyn Entity) -> Result<<Self::DB as Database>::Arguments<'_>>;
    fn extract_update_arguments<'a, M: Mutation>(mutation: &'a M, unique: &'a dyn Unique<Mutation = M>) -> Result<<Self::DB as Database>::Arguments<'a>>;
    fn extract_change_arguments<'a, M: Mutation>(mutation: &'a M, location: &'a M::Location) -> Result<<Self::DB as Database>::Arguments<'a>>;
    fn extract_delete_arguments<M: Mutation>(unique: &dyn Unique<Mutation = M>) -> Result<<Self::DB as Database>::Arguments<'_>>;
    fn extract_purify_arguments(location: &dyn Location) -> Result<<Self::DB as Database>::Arguments<'_>>;
    fn extract_template_arguments(template: &dyn TemplateRecord) -> Result<<Self::DB as Database>::Arguments<'_>>;
    fn extract_template_count_arguments(template: &dyn TemplateRecord) -> Result<<Self::DB as Database>::Arguments<'_>>;
}