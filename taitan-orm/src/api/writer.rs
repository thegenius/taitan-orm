use crate::extractor::Extractor;
use crate::result::Result;
use crate::prelude::{SqlExecutor, SqlGeneratorContainer};
use crate::sql_generator::SqlGenerator;
use taitan_orm_trait::{Entity, Location, Mutation, Unique};
use tracing::debug;

impl<T> WriterApi for T where T: SqlExecutor + SqlGeneratorContainer + Extractor {}

pub trait WriterApi: SqlExecutor + SqlGeneratorContainer + Extractor {
    async fn insert(&self, entity: &dyn Entity) -> Result<bool> {
        debug!(target: "taitan_orm", command = "insert", entity = ?entity);
        let sql = self.get_generator().get_insert_sql(entity);
        debug!(target: "taitan_orm", command = "insert", sql = sql);
        let args = Self::extract_insert_arguments(entity)?;
        let result = self.execute(&sql, args).await?;
        debug!(target: "taitan_orm", command = "insert", result = ?result);
        Ok(result > 0)
    }
    async fn upsert(&self, entity: &dyn Entity) -> Result<bool> {
        debug!(target: "taitan_orm", command = "upsert", entity = ?entity);
        let sql = self.get_generator().get_upsert_sql(entity);
        debug!(target: "taitan_orm", command = "upsert", sql = sql);
        let args = Self::extract_upsert_arguments(entity)?;
        let result = self.execute(&sql, args).await?;
        debug!(target: "taitan_orm", command = "upsert", result = ?result);
        Ok(result > 0)
    }
    async fn update<M: Mutation>(
        &self,
        mutation: &M,
        unique: &dyn Unique<Mutation = M>,
    ) -> Result<bool> {
        debug!(target: "taitan_orm", command = "update", mutation = ?mutation, primary = ?unique);
        let sql = self.get_generator().get_update_sql(mutation, unique);
        debug!(target: "taitan_orm", command = "update", sql = sql);
        let args = Self::extract_update_arguments(mutation, unique)?;
        let result = self.execute(&sql, args).await?;
        debug!(target: "taitan_orm", command = "update", result = ?result);
        Ok(result > 0)
    }
    async fn change<M: Mutation>(&self, mutation: &M, location: &M::Location) -> Result<u64> {
        debug!(target: "taitan_orm", command = "change", mutation = ?mutation, location = ?location);
        let sql = self.get_generator().get_change_sql(mutation, location);
        debug!(target: "taitan_orm", command = "change", sql = sql);
        let args = Self::extract_change_arguments(mutation, location)?;
        let result = self.execute(&sql, args).await?;
        debug!(target: "taitan_orm", command = "change", result = ?result);
        Ok(result)
    }
    async fn delete<M: Mutation>(&self, unique: &dyn Unique<Mutation = M>) -> Result<bool> {
        debug!(target: "taitan_orm", command = "delete", primary = ?unique);
        let sql = self.get_generator().get_delete_sql(unique);
        debug!(target: "taitan_orm", command = "delete", sql = sql);
        let args = Self::extract_delete_arguments(unique)?;
        let result = self.execute(&sql, args).await?;
        debug!(target: "taitan_orm", command = "delete", result = ?result);
        Ok(result > 0)
    }
    async fn purify(&self, location: &dyn Location) -> Result<u64> {
        debug!(target: "taitan_orm", command = "purify", location = ?location);
        let sql = self.get_generator().get_purify_sql(location);
        debug!(target: "taitan_orm", command = "purify", sql = sql);
        let args = Self::extract_purify_arguments(location)?;
        let result = self.execute(&sql, args).await?;
        debug!(target: "taitan_orm", command = "purify", result = ?result);
        Ok(result)
    }
}
