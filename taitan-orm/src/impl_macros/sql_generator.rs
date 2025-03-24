#[doc(hidden)]
#[macro_export]
macro_rules! sql_generator_impl {
    ($db:ty, $entity: ident,  $se: path, $unique: ident, $location:path, $mutation: path)=>{
        pub fn gen_unique_count_sql< M: $mutation>(
            unique: &dyn $unique<Mutation = M>,
        ) -> String {
            let where_sql = unique.gen_where_sql();
            let table_name = unique.table_name();
            format!("SELECT COUNT(1) FROM {} WHERE {}", table_name, where_sql)
        }

        pub fn gen_location_count_sql(location: &dyn $location) -> String {
            let where_sql = location.gen_where_sql();
            let table_name = location.table_name();
            format!("SELECT COUNT(1) FROM {} WHERE {}", table_name, where_sql)
        }

        pub fn gen_select_sql<M, SE>(selected: &SE, unique: &dyn $unique<Mutation = M>) -> String
        where
            M: $mutation,
            SE: $se + Send + Unpin,
        {
            let where_sql = unique.gen_where_sql();
            let table_name = unique.table_name();
            let selected_sql = selected.gen_select_sql();
            format!(
                "SELECT {} FROM {} WHERE {}",
                selected_sql, table_name, where_sql
            )
        }

        pub fn gen_search_sql<SE>(
            selected: &SE,
            location: &dyn $location,
            order_by: &dyn OrderBy,
            pagination: &Pagination,
        ) -> String
        where
            SE: $se + Send + Unpin,
        {
            let selected_sql = selected.gen_select_sql();
            let where_sql = location.gen_where_sql();
            let table_name = location.table_name();
            let order_sql = order_by.get_fields().join(",");
            let limit_sql = Pagination::gen_limit_sql();
            format!(
                "SELECT {} FROM {} WHERE {} ORDER BY {} LIMIT {}",
                selected_sql, table_name, where_sql, order_sql, limit_sql
            )
        }

        pub fn gen_search_all_sql<SE>(
            selected: &SE,
            location: &dyn $location,
            order_by: &dyn OrderBy,
        ) -> String
        where
            SE: $se + Send + Unpin,
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

        pub fn gen_insert_sql(entity: &dyn $entity) -> String {
            entity.gen_insert_sql().into()
        }

        pub fn gen_upsert_sql(entity: &dyn $entity) -> String {
            entity.gen_upsert_sql().into()
        }

        pub fn gen_create_sql(entity: &dyn $entity) -> String {
            entity.gen_create_sql().into()
        }

        pub fn gen_update_sql<M>(mutation: &M, unique: &dyn $unique<Mutation = M>) -> String
        where
            M: $mutation,
        {
            let set_sql = mutation.gen_update_set_sql();
            let table_name = unique.table_name();
            let where_sql = unique.gen_where_sql();
            format!("UPDATE {} SET {} WHERE {}", table_name, set_sql, where_sql)
        }

        pub fn gen_change_sql(mutation: &dyn $mutation, location: &dyn $location) -> String {
            let set_sql = mutation.gen_update_set_sql();
            let table_name = location.table_name();
            let where_sql = location.gen_where_sql();
            format!("UPDATE {} SET {} WHERE {}", table_name, set_sql, where_sql)
        }

        pub fn gen_delete_sql<M: $mutation>(unique: &dyn $unique<Mutation = M>) -> String {
            let where_sql = unique.gen_where_sql();
            let table_name = unique.table_name();
            format!("DELETE FROM {} WHERE {}", table_name, where_sql)
        }

        pub fn gen_purify_sql(location: &dyn $location) -> String {
            let where_sql = location.gen_where_sql();
            let table_name = location.table_name();
            format!("DELETE FROM {} WHERE {}", table_name, where_sql)
        }
    }
}