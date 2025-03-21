

use crate::{change_fn, delete_fn, insert_fn, purify_fn, update_fn, upsert_fn, Result};
use crate::{SqlExecutor, SqlGenerator};

use crate::sql_generator_container::SqlGeneratorContainer;
use sqlx::Sqlite;


use taitan_orm_trait::{
    Entity, Location, Mutation, Unique,
};




pub trait SqliteWriteCommander: SqlExecutor<DB = Sqlite> + SqlGeneratorContainer {



    // async fn insert(&mut self, entity: &dyn Entity) -> Result<bool> {
    //     debug!(target: "taitan_orm", command = "insert",  entity = ?entity);
    //     let sql = self.get_generator().get_insert_sql(entity);
    //     debug!(target: "taitan_orm", command = "insert", sql = sql);
    //     let args = entity.gen_insert_arguments_sqlite()?;
    //     let result = self.execute::<SqliteArguments>(&sql, args).await?;
    //     debug!(target: "taitan_orm", command = "insert", result = ?result);
    //     Ok(result > 0)
    // }
    insert_fn!(SqliteArguments, Entity::gen_insert_arguments_sqlite);



    // async fn upsert(&mut self, entity: &dyn Entity) -> Result<bool> {
    //     debug!(target: "taitan_orm", command = "upsert", entity = ?entity);
    //     let sql = self.get_generator().get_upsert_sql(entity);
    //     debug!(target: "taitan_orm", command = "upsert", sql = sql);
    //     let args = entity.gen_upsert_arguments_sqlite()?;
    //     let result = self.execute::<SqliteArguments>(&sql, args).await?;
    //     debug!(target: "taitan_orm", command = "upsert", result = ?result);
    //     Ok(result > 0)
    // }
    upsert_fn!(SqliteArguments, Entity::gen_upsert_arguments_sqlite);



    // async fn update<M: Mutation>(
    //     &mut self,
    //     mutation: &M,
    //     unique: &dyn Unique<Mutation = M>,
    // ) -> Result<bool> {
    //     debug!(target: "taitan_orm", command = "update", mutation = ?mutation, primary = ?unique);
    //     let sql = self.get_generator().get_update_sql(mutation, unique);
    //     debug!(target: "taitan_orm", command = "update", sql = sql);
    //     let args = unique.gen_update_arguments_sqlite(mutation)?;
    //     let result = self.execute::<SqliteArguments>(&sql, args).await?;
    //     debug!(target: "taitan_orm", command = "update", result = ?result);
    //     Ok(result > 0)
    // }
    update_fn!(SqliteArguments, Unique::gen_update_arguments_sqlite);



    // async fn change<L: Location>(
    //     &mut self,
    //     mutation: &dyn Mutation<Location = L>,
    //     location: &L,
    // ) -> Result<u64> {
    //     debug!(target: "taitan_orm", command = "change", mutation = ?mutation, location = ?location);
    //     let sql = self.get_generator().get_change_sql(mutation, location);
    //     debug!(target: "taitan_orm", command = "change", sql = sql);
    //     let args = mutation.gen_change_arguments_sqlite(location)?;
    //     let result = self.execute::<SqliteArguments>(&sql, args).await?;
    //     debug!(target: "taitan_orm", command = "change", result = ?result);
    //     Ok(result)
    // }
    change_fn!(SqliteArguments, Mutation::gen_change_arguments_sqlite);



    // async fn delete<M: Mutation>(&mut self, unique: &dyn Unique<Mutation = M>) -> Result<bool> {
    //     debug!(target: "taitan_orm", command = "delete", primary = ?unique);
    //     let sql = self.get_generator().get_delete_sql(unique);
    //     debug!(target: "taitan_orm", command = "delete", sql = sql);
    //     let args = unique.gen_unique_arguments_sqlite()?;
    //     let result = self.execute::<SqliteArguments>(&sql, args).await?;
    //     debug!(target: "taitan_orm", command = "delete", result = ?result);
    //     Ok(result > 0)
    // }
    delete_fn!(SqliteArguments, Unique::gen_unique_arguments_sqlite);


    // async fn purify(&mut self, location: &dyn Location) -> Result<u64> {
    //     debug!(target: "taitan_orm", command = "purify", location = ?location);
    //     let sql = self.get_generator().get_purify_sql(location);
    //     debug!(target: "taitan_orm", command = "purify", sql = sql);
    //     let args = location.gen_location_arguments_sqlite()?;
    //     let result = self.execute::<SqliteArguments>(&sql, args).await?;
    //     debug!(target: "taitan_orm", command = "purify", result = ?result);
    //     Ok(result)
    // }
    purify_fn!(SqliteArguments, Location::gen_location_arguments_sqlite);
}
