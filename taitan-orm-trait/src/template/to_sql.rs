pub trait ToSql {
    fn to_set_sql(&self) -> String;
    fn to_where_sql(&self) -> String;
}