use sqlx::Database;

use taitan_orm_trait::brave_new::{Entity, Location, Mutation, Template, Unique};
use taitan_orm_trait::brave_new::Pagination;
use crate::prelude::SqlGenericExecutor;
use crate::result::Result;

pub trait ArgsExtractor: SqlGenericExecutor {
    fn extract_pagination_arguments(page: &Pagination)-> Result<<Self::DB as Database>::Arguments<'_>>;
    fn extract_unique_arguments<M: Mutation<Self::DB>>(unique: &dyn Unique<Self::DB, Mutation = M>) -> Result<<Self::DB as Database>::Arguments<'_>>;
    fn extract_location_arguments(location: &dyn Location<Self::DB>) -> Result<<Self::DB as Database>::Arguments<'_>>;
    fn extract_insert_arguments(entity: &dyn Entity<Self::DB>) -> Result<<Self::DB as Database>::Arguments<'_>>;
    fn extract_upsert_arguments(entity: &dyn Entity<Self::DB>) -> Result<<Self::DB as Database>::Arguments<'_>>;
    fn extract_update_arguments<'a, M: Mutation<Self::DB>>(mutation: &'a M, unique: &'a dyn Unique<Self::DB, Mutation = M>) -> Result<<Self::DB as Database>::Arguments<'a>>;
    fn extract_change_arguments<'a>(mutation: &'a dyn Mutation<Self::DB>, location: &'a dyn Location<Self::DB>) -> Result<<Self::DB as Database>::Arguments<'a>>;
    fn extract_delete_arguments<M: Mutation<Self::DB>>(unique: &dyn Unique<Self::DB, Mutation = M>) -> Result<<Self::DB as Database>::Arguments<'_>>;
    fn extract_purify_arguments(location: &dyn Location<Self::DB>) -> Result<<Self::DB as Database>::Arguments<'_>>;
    fn extract_template_arguments(template: &dyn Template<Self::DB>) -> Result<<Self::DB as Database>::Arguments<'_>>;
    fn extract_template_count_arguments(template: &dyn Template<Self::DB>) -> Result<<Self::DB as Database>::Arguments<'_>>;
}