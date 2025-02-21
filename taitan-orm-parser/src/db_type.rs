use std::fmt::{Display};
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Copy, Serialize, Deserialize)]
pub enum DatabaseType {
    MySql,
    Postgres,
    Sqlite,
}

impl Display for DatabaseType {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DatabaseType::MySql => write!(fmt, "MySql"),
            DatabaseType::Postgres => write!(fmt, "Postgres"),
            DatabaseType::Sqlite => write!(fmt, "Sqlite"),
        }
    }
}
