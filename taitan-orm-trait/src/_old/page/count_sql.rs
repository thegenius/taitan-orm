pub enum CountSql {
    Empty,
    PlainSql(String),
    VariableSql(String),
}
