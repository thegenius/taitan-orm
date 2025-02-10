use crate::brave_new::ArgsExtractor;
use crate::result::Result;
use crate::brave_new::{SqlExecutor};
use crate::brave_new::SqlGenerator;
use taitan_orm_trait::brave_new::{Entity, Location, Mutation, Unique};

use tracing::debug;

impl<T> WriterApi for T where T: SqlExecutor + SqlGenerator + ArgsExtractor {}

//
// # basic write
// ```
// insert(entity) -> () # fail if conflict
// upsert(entity) -> () # update if conflict
// create(entity) -> () # fail if conflict, return generated field
//
// update(mutation, unique  ) -> bool # return true if update take effect
// change(mutation, location) -> u64  # return affected rows
//
// delete(unique  ) -> bool # return true if delete take effect
// purify(location) -> u64  # return deleted rows
// ```
//
// # batch write
// ```
// batch_insert([entity])                 -> ()  # success if no conflict
// batch_insert_ignore_conflict([entity]) -> ()  # always success, ignore conflict
// batch_upsert([entity])                 -> ()  # always success, update conflict
// batch_delete([unique])                 -> u64 # return deleted rows
// ```

pub trait WriterApi: SqlExecutor + SqlGenerator + ArgsExtractor {
    async fn insert(&self, entity: &dyn Entity<Self::DB>) -> Result<()> {
        debug!(target: "taitan_orm", command = "insert", entity = ?entity);
        let sql = self.gen_insert_sql(entity);
        debug!(target: "taitan_orm", command = "insert", sql = sql);
        let args = Self::extract_insert_arguments(entity)?;
        let result = self.execute(&sql, args).await?;
        debug!(target: "taitan_orm", command = "insert", result = ?result);
        Ok(())
    }
    async fn upsert(&self, entity: &dyn Entity<Self::DB>) -> Result<()> {
        debug!(target: "taitan_orm", command = "upsert", entity = ?entity);
        let sql = self.gen_upsert_sql(entity);
        debug!(target: "taitan_orm", command = "upsert", sql = sql);
        let args = Self::extract_upsert_arguments(entity)?;
        let result = self.execute(&sql, args).await?;
        debug!(target: "taitan_orm", command = "upsert", result = ?result);
        Ok(())
    }

    async fn create(&self, entity: &mut dyn Entity<Self::DB>) -> Result<()> {
        debug!(target: "taitan_orm", command = "insert", entity = ?entity);
        let sql = self.gen_create_sql(entity);
        debug!(target: "taitan_orm", command = "insert", sql = sql);
        let args = Self::extract_insert_arguments(entity)?;
        let result = self.execute(&sql, args).await?;
        debug!(target: "taitan_orm", command = "insert", result = ?result);
        Ok(())
    }

    async fn update<M: Mutation<Self::DB>>(
        &self,
        mutation: &M,
        unique: &dyn Unique<Self::DB, Mutation = M>,
    ) -> Result<bool> {
        debug!(target: "taitan_orm", command = "update", mutation = ?mutation, primary = ?unique);
        let sql = self.gen_update_sql(mutation, unique);
        debug!(target: "taitan_orm", command = "update", sql = ?sql);
        let args = Self::extract_update_arguments(mutation, unique)?;
        let result = self.execute(&sql, args).await?;
        debug!(target: "taitan_orm", command = "update", result = ?result);
        Ok(result > 0)
    }
    async fn change(&self, mutation: &dyn Mutation<Self::DB>, location: &dyn Location<Self::DB>) -> Result<u64> {
        debug!(target: "taitan_orm", command = "change", mutation = ?mutation, location = ?location);
        let sql = self.gen_change_sql(mutation, location);
        debug!(target: "taitan_orm", command = "change", sql = ?sql);
        let args = Self::extract_change_arguments(mutation, location)?;
        let result = self.execute(&sql, args).await?;
        debug!(target: "taitan_orm", command = "change", result = ?result);
        Ok(result)
    }
    async fn delete<M: Mutation<Self::DB>>(&self, unique: &dyn Unique<Self::DB, Mutation = M>) -> Result<bool> {
        debug!(target: "taitan_orm", command = "delete", primary = ?unique);
        let sql = self.gen_delete_sql(unique);
        debug!(target: "taitan_orm", command = "delete", sql = sql);
        let args = Self::extract_delete_arguments(unique)?;
        let result = self.execute(&sql, args).await?;
        debug!(target: "taitan_orm", command = "delete", result = ?result);
        Ok(result > 0)
    }
    async fn purify(&self, location: &dyn Location<Self::DB>) -> Result<u64> {
        debug!(target: "taitan_orm", command = "purify", location = ?location);
        let sql = self.gen_purify_sql(location);
        debug!(target: "taitan_orm", command = "purify", sql = sql);
        let args = Self::extract_purify_arguments(location)?;
        let result = self.execute(&sql, args).await?;
        debug!(target: "taitan_orm", command = "purify", result = ?result);
        Ok(result)
    }
}
