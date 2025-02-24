use std::fmt::{Display};
use proc_macro2::Ident;
use quote::format_ident;
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


impl DatabaseType {
    pub fn gen_ident(&self) -> Ident {
        match self {
            DatabaseType::MySql => format_ident!("MySql"),
            DatabaseType::Sqlite => format_ident!("Sqlite"),
            DatabaseType::Postgres => format_ident!("Postgres"),
        }
    }
}