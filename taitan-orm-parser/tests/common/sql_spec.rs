use crate::common::named_input::NamedDeriveInput;
use crate::common::named_map::{Named, NamedMap};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::sync::OnceLock;
use taitan_orm_parser::{DatabaseType, SqlType};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct SqlSpec {
    pub input_name: String,
    pub db_type: DatabaseType,
    pub sql_type: SqlType,
    pub expected: String,
}

impl Named for SqlSpec {
    fn name(&self) -> Cow<str> {
        Cow::Owned(format!(
            "{}.{}.{}",
            self.db_type, self.sql_type, self.input_name
        ))
    }
}

static EXPECT_SQL_MAP: OnceLock<NamedMap<SqlSpec>> = OnceLock::new();

pub fn get_sql_specs<'a>() -> NamedMap<SqlSpec> {
    let input_map = EXPECT_SQL_MAP.get_or_init(|| {
        let mut inputs = NamedMap::new();
        sql_specs()
            .into_iter()
            .for_each(|n| inputs.insert(n));
        inputs
    });
    input_map.clone()
}
fn sql_specs() -> Vec<SqlSpec> {
    vec![
        serde_yaml::from_str::<SqlSpec>(include_str!("../specs/sqls/mysql/insert_spec/insert_001.spec")).unwrap(),
        serde_yaml::from_str::<SqlSpec>(include_str!("../specs/sqls/mysql/insert_spec/insert_002.spec")).unwrap(),
        serde_yaml::from_str::<SqlSpec>(include_str!("../specs/sqls/postgres/insert_spec/insert_001.spec")).unwrap(),
        serde_yaml::from_str::<SqlSpec>(include_str!("../specs/sqls/postgres/insert_spec/insert_002.spec")).unwrap(),
    ]
}


