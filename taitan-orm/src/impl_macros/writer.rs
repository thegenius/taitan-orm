#[doc(hidden)]
#[macro_export]
macro_rules! writer_impl {
    ($gen: ident, $entity: path, $mutation: path, $location: path, $unique: ident) => {
          async fn insert(&self, entity: &dyn $entity) -> Result<()> {
                debug!(target: "taitan_orm", command = "insert", entity = ?entity);
                let sql = $gen::gen_insert_sql(entity);
                debug!(target: "taitan_orm", command = "insert", sql = sql);
                let args = entity.gen_args()?;
                let result = self.execute(&sql, args).await?;
                debug!(target: "taitan_orm", command = "insert", result = ?result);
                Ok(())
            }
            async fn upsert(&self, entity: &dyn $entity) -> Result<()> {
                debug!(target: "taitan_orm", command = "upsert", entity = ?entity);
                let sql = $gen::gen_upsert_sql(entity);
                debug!(target: "taitan_orm", command = "upsert", sql = sql);
                let args = entity.gen_args()?;
                let result = self.execute(&sql, args).await?;
                debug!(target: "taitan_orm", command = "upsert", result = ?result);
                Ok(())
            }

            async fn create(&self, entity: &mut dyn $entity) -> Result<()> {
                debug!(target: "taitan_orm", command = "insert", entity = ?entity);
                let sql = $gen::gen_create_sql(entity);
                debug!(target: "taitan_orm", command = "insert", sql = sql);
                let args = entity.gen_args()?;
                let result = self.execute(&sql, args).await?;
                debug!(target: "taitan_orm", command = "insert", result = ?result);
                Ok(())
            }

            async fn update<M: $mutation>(
                &self,
                mutation: &M,
                unique: &dyn $unique<Mutation = M>,
            ) -> Result<bool> {
                debug!(target: "taitan_orm", command = "update", mutation = ?mutation, primary = ?unique);
                let sql = $gen::gen_update_sql(mutation, unique);
                debug!(target: "taitan_orm", command = "update", sql = ?sql);
                let mut args = mutation.gen_args()?;
                unique.add_to_args(&mut args)?;
                let result = self.execute(&sql, args).await?;
                debug!(target: "taitan_orm", command = "update", result = ?result);
                Ok(result > 0)
            }
            async fn change(
                &self,
                mutation: &dyn $mutation,
                location: &dyn $location,
            ) -> Result<u64> {
                debug!(target: "taitan_orm", command = "change", mutation = ?mutation, location = ?location);
                let sql = $gen::gen_change_sql(mutation, location);
                debug!(target: "taitan_orm", command = "change", sql = ?sql);
                let mut args = mutation.gen_args()?;
                location.add_to_args(&mut args)?;
                let result = self.execute(&sql, args).await?;
                debug!(target: "taitan_orm", command = "change", result = ?result);
                Ok(result)
            }
            async fn delete<M: $mutation>(
                &self,
                unique: &dyn $unique<Mutation = M>,
            ) -> Result<bool> {
                debug!(target: "taitan_orm", command = "delete", primary = ?unique);
                let sql = $gen::gen_delete_sql(unique);
                debug!(target: "taitan_orm", command = "delete", sql = sql);
                let args = unique.gen_args()?;
                let result = self.execute(&sql, args).await?;
                debug!(target: "taitan_orm", command = "delete", result = ?result);
                Ok(result > 0)
            }
            async fn purify(&self, location: &dyn $location) -> Result<u64> {
                debug!(target: "taitan_orm", command = "purify", location = ?location);
                let sql = $gen::gen_purify_sql(location);
                debug!(target: "taitan_orm", command = "purify", sql = sql);
                let args = location.gen_args()?;
                let result = self.execute(&sql, args).await?;
                debug!(target: "taitan_orm", command = "purify", result = ?result);
                Ok(result)
            }
    }
}