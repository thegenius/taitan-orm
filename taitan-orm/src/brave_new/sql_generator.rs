use sqlx::Database;
use taitan_orm_trait::brave_new::{Entity, Location, Mutation, Pagination};
use taitan_orm_trait::brave_new::{OrderBy, Selected, Unique};

pub trait SqlGenerator {
    fn gen_unique_count_sql<DB: Database, M: Mutation<DB>>(
        &self,
        unique: &dyn Unique<DB, Mutation = M>,
    ) -> String {
        let where_sql = unique.gen_where_sql();
        let table_name = unique.table_name();
        format!("SELECT COUNT(1) FROM {} WHERE {}", table_name, where_sql)
    }

    fn gen_location_count_sql<DB: Database>(&self, location: &dyn Location<DB>) -> String {
        let where_sql = location.gen_where_sql();
        let table_name = location.table_name();
        format!("SELECT COUNT(1) FROM {} WHERE {}", table_name, where_sql)
    }

    fn gen_select_sql<DB, M, SE>(
        &self,
        selected: &SE,
        unique: &dyn Unique<DB, Mutation = M>,
    ) -> String
    where
        DB: Database,
        M: Mutation<DB>,
        SE: Selected<DB> + Send + Unpin,
    {
        let where_sql = unique.gen_where_sql();
        let table_name = unique.table_name();
        let selected_sql = selected.gen_select_sql();
        format!(
            "SELECT {} FROM {} WHERE {}",
            selected_sql, table_name, where_sql
        )
    }

    fn gen_search_sql<DB, SE>(
        &self,
        selected: &SE,
        location: &dyn Location<DB>,
        order_by: &dyn OrderBy,
        pagination: &Pagination,
    ) -> String
    where
        DB: Database,
        SE: Selected<DB> + Send + Unpin,
    {
        let selected_sql = selected.gen_select_sql();
        let where_sql = location.gen_where_sql();
        let table_name = location.table_name();
        let order_sql = order_by.get_fields().join(",");
        let limit_sql = pagination.gen_limit_sql();
        format!(
            "SELECT {} FROM {} WHERE {} ORDER BY {} LIMIT {}",
            selected_sql, table_name, where_sql, order_sql, limit_sql
        )
    }

    fn gen_search_all_sql<DB, SE>(
        &self,
        selected: &SE,
        location: &dyn Location<DB>,
        order_by: &dyn OrderBy,
    ) -> String
    where
        DB: Database,
        SE: Selected<DB> + Send + Unpin,
    {
        let selected_sql = selected.gen_select_sql();
        let where_sql = location.gen_where_sql();
        let table_name = location.table_name();
        let order_sql = order_by.get_fields().join(",");
        format!(
            "SELECT {} FROM {} WHERE {} ORDER BY {}",
            selected_sql, table_name, where_sql, order_sql
        )
    }

    fn gen_insert_sql<DB>(&self, entity: &dyn Entity<DB>) -> String
    where
        DB: Database,
    {
        entity.gen_insert_sql().into()
    }

    fn gen_upsert_sql<DB>(&self, entity: &dyn Entity<DB>) -> String
    where
        DB: Database,
    {
        entity.gen_upsert_sql().into()
    }

    fn gen_create_sql<DB>(&self, entity: &dyn Entity<DB>) -> String
    where
        DB: Database,
    {
        entity.gen_create_sql().into()
    }

    fn gen_update_sql<DB, M>(&self, mutation: &M, unique: &dyn Unique<DB, Mutation = M>) -> String
    where
        M: Mutation<DB>,
        DB: Database,
    {
        let set_sql = mutation.gen_update_set_sql();
        let table_name = unique.table_name();
        let where_sql = unique.gen_where_sql();
        format!("UPDATE {} SET {} WHERE {}", table_name, set_sql, where_sql)
    }

    fn gen_change_sql<DB>(&self, mutation: &dyn Mutation<DB>, location: &dyn Location<DB>) -> String
    where
        DB: Database,
    {
        let set_sql = mutation.gen_update_set_sql();
        let table_name = location.table_name();
        let where_sql = location.gen_where_sql();
        format!("UPDATE {} SET {} WHERE {}", table_name, set_sql, where_sql)
    }

    fn gen_delete_sql<DB: Database, M: Mutation<DB>>(
        &self,
        unique: &dyn Unique<DB, Mutation = M>,
    ) -> String {
        let where_sql = unique.gen_where_sql();
        let table_name = unique.table_name();
        format!("DELETE FROM {} WHERE {}", table_name, where_sql)
    }

    fn gen_purify_sql<DB: Database>(
        &self,
        location: &dyn Location<DB>
    ) -> String {
        let where_sql = location.gen_where_sql();
        let table_name = location.table_name();
        format!("DELETE FROM {} WHERE {}", table_name, where_sql)
    }
}
