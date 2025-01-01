use taitan_orm_trait::{Location, Mutation, Unique};

#[macro_export]
macro_rules! insert_fn {
    ($args_type:ty, $gen_args_fn:path) => {
        async fn insert(&mut self, entity: &dyn Entity) -> Result<bool> {
            tracing::debug!(target: "taitan_orm", command = "insert", entity = ?entity);
            let sql = self.get_generator().get_insert_sql(entity);
            tracing::debug!(target: "taitan_orm", command = "insert", sql = sql);
            let args = ($gen_args_fn)(entity)?;
            let result = self.execute(&sql, args).await?;
            tracing::debug!(target: "taitan_orm", command = "insert", result = ?result);
            Ok(result > 0)
        }
    };
}

#[macro_export]
macro_rules! upsert_fn {
    ($args_type:ty, $gen_args_fn:path) => {
        async fn upsert(&mut self, entity: &dyn Entity) -> Result<bool> {
            tracing::debug!(target: "taitan_orm", command = "upsert", entity = ?entity);
            let sql = self.get_generator().get_upsert_sql(entity);
            tracing::debug!(target: "taitan_orm", command = "upsert", sql = sql);
            let args = ($gen_args_fn)(entity)?;
            let result = self.execute(&sql, args).await?;
            tracing::debug!(target: "taitan_orm", command = "upsert", result = ?result);
            Ok(result > 0)
        }
    };
}

#[macro_export]
macro_rules! update_fn {
    ($args_type:ty, $gen_args_fn:path) => {
        async fn update<M: Mutation>(
            &mut self,
            mutation: &M,
            unique: &dyn Unique<Mutation = M>,
        ) -> Result<bool> {
            tracing::debug!(target: "taitan_orm", command = "update", mutation = ?mutation, primary = ?unique);
            let sql = self.get_generator().get_update_sql(mutation, unique);
            tracing::debug!(target: "taitan_orm", command = "update", sql = sql);
            let args = ($gen_args_fn)(unique, mutation)?;
            let result = self.execute(&sql, args).await?;
            tracing::debug!(target: "taitan_orm", command = "update", result = ?result);
            Ok(result > 0)
        }
    };
}

#[macro_export]
macro_rules! change_fn {
    ($args_type:ty, $gen_args_fn:path) => {
        async fn change<L: Location>(
            &mut self,
            mutation: &dyn Mutation<Location = L>,
            location: &L,
        ) -> Result<u64> {
            tracing::debug!(target: "taitan_orm", command = "change", mutation = ?mutation, location = ?location);
            let sql = self.get_generator().get_change_sql(mutation, location);
            tracing::debug!(target: "taitan_orm", command = "change", sql = sql);
            let args = ($gen_args_fn)(mutation, location)?;
            let result = self.execute(&sql, args).await?;
            tracing::debug!(target: "taitan_orm", command = "change", result = ?result);
            Ok(result)
        }
    };
}

#[macro_export]
macro_rules! delete_fn {
    ($args_type:ty, $gen_args_fn:path) => {
       async fn delete<M: Mutation>(&mut self, unique: &dyn Unique<Mutation = M>) -> crate::Result<bool> {
           tracing::debug!(target: "taitan_orm", command = "delete", primary = ?unique);
           let sql = self.get_generator().get_delete_sql(unique);
           tracing::debug!(target: "taitan_orm", command = "delete", sql = sql);
           let args = ($gen_args_fn)(unique)?;
           let result = self.execute(&sql, args).await?;
           tracing::debug!(target: "taitan_orm", command = "delete", result = ?result);
           Ok(result > 0)
       }
    };
}

#[macro_export]
macro_rules! purify_fn {
    ($args_type:ty, $gen_args_fn:path) => {
        async fn purify(&mut self, location: &dyn Location) -> crate::Result<u64> {
            tracing::debug!(target: "taitan_orm", command = "purify", location = ?location);
            let sql = self.get_generator().get_purify_sql(location);
            tracing::debug!(target: "taitan_orm", command = "purify", sql = sql);
            let args = ($gen_args_fn)(location)?;
            let result = self.execute(&sql, args).await?;
            tracing::debug!(target: "taitan_orm", command = "purify", result = ?result);
            Ok(result)
        }
    };
}
