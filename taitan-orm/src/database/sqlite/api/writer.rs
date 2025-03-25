// use crate::args_extractor::ArgsExtractor;
// use crate::sql_executor::SqlExecutor;
// use crate::sql_generator::SqlGenerator;
use sqlx::Sqlite;
use taitan_orm_trait::result::Result;
use taitan_orm_trait::traits::{Entity, SqliteEntity, SqliteLocation, SqliteMutation, SqliteUnique};

use tracing::debug;
use crate::database::sqlite::generator::SqliteSqlGenerator;
use crate::new_executor::SqlExecutorNew;
use crate::writer_impl;

impl<T> WriterApiNew for T
where T: SqlExecutorNew<Sqlite>
{
}

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

pub trait WriterApiNew:  SqlExecutorNew<Sqlite> {
    writer_impl!(SqliteSqlGenerator, SqliteEntity, SqliteMutation, SqliteLocation, SqliteUnique);
    // async fn insert(&self, entity: &dyn SqliteEntity) -> Result<()> {
    //     debug!(target: "taitan_orm", command = "insert", entity = ?entity);
    //     let sql = SqliteSqlGenerator::gen_insert_sql(entity);
    //     debug!(target: "taitan_orm", command = "insert", sql = sql);
    //     let args = entity.gen_args()?;
    //     let result = self.execute(&sql, args).await?;
    //     debug!(target: "taitan_orm", command = "insert", result = ?result);
    //     Ok(())
    // }
    // async fn upsert(&self, entity: &dyn SqliteEntity) -> Result<()> {
    //     debug!(target: "taitan_orm", command = "upsert", entity = ?entity);
    //     let sql = SqliteSqlGenerator::gen_upsert_sql(entity);
    //     debug!(target: "taitan_orm", command = "upsert", sql = sql);
    //     let args = entity.gen_args()?;
    //     let result = self.execute(&sql, args).await?;
    //     debug!(target: "taitan_orm", command = "upsert", result = ?result);
    //     Ok(())
    // }
    //
    // async fn create(&self, entity: &mut dyn SqliteEntity) -> Result<()> {
    //     debug!(target: "taitan_orm", command = "insert", entity = ?entity);
    //     let sql = SqliteSqlGenerator::gen_create_sql(entity);
    //     debug!(target: "taitan_orm", command = "insert", sql = sql);
    //     let args = entity.gen_args()?;
    //     let result = self.execute(&sql, args).await?;
    //     debug!(target: "taitan_orm", command = "insert", result = ?result);
    //     Ok(())
    // }
    //
    // async fn update<M: SqliteMutation>(
    //     &self,
    //     mutation: &M,
    //     unique: &dyn SqliteUnique<Mutation = M>,
    // ) -> Result<bool> {
    //     debug!(target: "taitan_orm", command = "update", mutation = ?mutation, primary = ?unique);
    //     let sql = SqliteSqlGenerator::gen_update_sql(mutation, unique);
    //     debug!(target: "taitan_orm", command = "update", sql = ?sql);
    //     let mut args = mutation.gen_args()?;
    //     unique.add_to_args(&mut args)?;
    //     let result = self.execute(&sql, args).await?;
    //     debug!(target: "taitan_orm", command = "update", result = ?result);
    //     Ok(result > 0)
    // }
    // async fn change(
    //     &self,
    //     mutation: &dyn SqliteMutation,
    //     location: &dyn SqliteLocation,
    // ) -> Result<u64> {
    //     debug!(target: "taitan_orm", command = "change", mutation = ?mutation, location = ?location);
    //     let sql = SqliteSqlGenerator::gen_change_sql(mutation, location);
    //     debug!(target: "taitan_orm", command = "change", sql = ?sql);
    //     let mut args = mutation.gen_args()?;
    //     location.add_to_args(&mut args)?;
    //     let result = self.execute(&sql, args).await?;
    //     debug!(target: "taitan_orm", command = "change", result = ?result);
    //     Ok(result)
    // }
    // async fn delete<M: SqliteMutation>(
    //     &self,
    //     unique: &dyn SqliteUnique<Mutation = M>,
    // ) -> Result<bool> {
    //     debug!(target: "taitan_orm", command = "delete", primary = ?unique);
    //     let sql = SqliteSqlGenerator::gen_delete_sql(unique);
    //     debug!(target: "taitan_orm", command = "delete", sql = sql);
    //     let args = unique.gen_args()?;
    //     let result = self.execute(&sql, args).await?;
    //     debug!(target: "taitan_orm", command = "delete", result = ?result);
    //     Ok(result > 0)
    // }
    // async fn purify(&self, location: &dyn SqliteLocation) -> Result<u64> {
    //     debug!(target: "taitan_orm", command = "purify", location = ?location);
    //     let sql = SqliteSqlGenerator::gen_purify_sql(location);
    //     debug!(target: "taitan_orm", command = "purify", sql = sql);
    //     let args = location.gen_args()?;
    //     let result = self.execute(&sql, args).await?;
    //     debug!(target: "taitan_orm", command = "purify", result = ?result);
    //     Ok(result)
    // }
}
