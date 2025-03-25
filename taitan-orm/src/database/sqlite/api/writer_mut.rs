// use crate::args_extractor::ArgsExtractor;
use crate::database::sqlite::generator::SqliteSqlGenerator;
use crate::new_executor::SqlExecutorMutNew;
// use crate::sql_executor_mut::SqlExecutorMut;
// use crate::sql_generator::SqlGenerator;
use taitan_orm_trait::result::Result;
use taitan_orm_trait::traits::{
    Entity, SqliteEntity, SqliteLocation, SqliteMutation, SqliteUnique,
};
use tracing::debug;
use crate::writer_mut_impl;

impl<T> WriterMutApiNew for T where T: SqlExecutorMutNew<sqlx::Sqlite> {}

pub trait WriterMutApiNew: SqlExecutorMutNew<sqlx::Sqlite> {
    writer_mut_impl!(SqliteSqlGenerator, SqliteEntity, SqliteMutation, SqliteLocation, SqliteUnique);
    // async fn insert(&mut self, entity: &dyn SqliteEntity) -> Result<()> {
    //     debug!(target: "taitan_orm", command = "insert", entity = ?entity);
    //     let sql = SqliteSqlGenerator::gen_insert_sql(entity);
    //     debug!(target: "taitan_orm", command = "insert", sql = ?sql);
    //     let args = entity.gen_args()?;
    //     let result = self.execute(&sql, args).await?;
    //     debug!(target: "taitan_orm", command = "insert", result = ?result);
    //     Ok(())
    // }
    // async fn upsert(&mut self, entity: &dyn SqliteEntity) -> Result<()> {
    //     debug!(target: "taitan_orm", command = "upsert", entity = ?entity);
    //     let sql = SqliteSqlGenerator::gen_upsert_sql(entity);
    //     debug!(target: "taitan_orm", command = "upsert", sql = sql);
    //     let args = entity.gen_args()?;
    //     let result = self.execute(&sql, args).await?;
    //     debug!(target: "taitan_orm", command = "upsert", result = ?result);
    //     Ok(())
    // }
    //
    // async fn create(&mut self, entity: &mut dyn SqliteEntity) -> Result<()> {
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
    //     &mut self,
    //     mutation: &M,
    //     unique: &dyn SqliteUnique<Mutation = M>,
    // ) -> Result<bool> {
    //     debug!(target: "taitan_orm", command = "update", mutation = ?mutation, primary = ?unique);
    //     let sql = SqliteSqlGenerator::gen_update_sql(mutation, unique);
    //     debug!(target: "taitan_orm", command = "update", sql = sql);
    //     let mut args = mutation.gen_args()?;
    //     unique.add_to_args(&mut args)?;
    //     let result = self.execute(&sql, args).await?;
    //     debug!(target: "taitan_orm", command = "update", result = ?result);
    //     Ok(result > 0)
    // }
    // async fn change(
    //     &mut self,
    //     mutation: &dyn SqliteMutation,
    //     location: &dyn SqliteLocation,
    // ) -> Result<u64> {
    //     debug!(target: "taitan_orm", command = "change", mutation = ?mutation, location = ?location);
    //     let sql = SqliteSqlGenerator::gen_change_sql(mutation, location);
    //     debug!(target: "taitan_orm", command = "change", sql = sql);
    //     let mut args = mutation.gen_args()?;
    //     location.add_to_args(&mut args)?;
    //     let result = self.execute(&sql, args).await?;
    //     debug!(target: "taitan_orm", command = "change", result = ?result);
    //     Ok(result)
    // }
    // async fn delete<M: SqliteMutation>(
    //     &mut self,
    //     unique: &dyn SqliteUnique<Mutation = M>,
    // ) -> Result<bool> {
    //     debug!(target: "taitan_orm", command = "delete", primary = ?unique);
    //     let sql = SqliteSqlGenerator::gen_delete_sql(unique);
    //     debug!(target: "taitan_orm", command = "delete", sql = ?sql);
    //     let args = unique.gen_args()?;
    //     let result = self.execute(&sql, args).await?;
    //     debug!(target: "taitan_orm", command = "delete", result = ?result);
    //     Ok(result > 0)
    // }
    // async fn purify(&mut self, location: &dyn SqliteLocation) -> Result<u64> {
    //     debug!(target: "taitan_orm", command = "purify", location = ?location);
    //     let sql = SqliteSqlGenerator::gen_purify_sql(location);
    //     debug!(target: "taitan_orm", command = "purify", sql = ?sql);
    //     let args = location.gen_args()?;
    //     let result = self.execute(&sql, args).await?;
    //     debug!(target: "taitan_orm", command = "purify", result = ?result);
    //     Ok(result)
    // }
}
