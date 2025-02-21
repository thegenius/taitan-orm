use crate::common::SqlSpec;

pub fn sql_specs() -> Vec<SqlSpec> {
    vec![
        serde_yaml::from_str::<SqlSpec>(include_str!("../specs/sqls/mysql/insert_spec/insert_001.spec")).unwrap(),
        serde_yaml::from_str::<SqlSpec>(include_str!("../specs/sqls/mysql/insert_spec/insert_002.spec")).unwrap(),
        serde_yaml::from_str::<SqlSpec>(include_str!("../specs/sqls/postgres/insert_spec/insert_001.spec")).unwrap(),
        serde_yaml::from_str::<SqlSpec>(include_str!("../specs/sqls/postgres/insert_spec/insert_002.spec")).unwrap(),
    ]
}