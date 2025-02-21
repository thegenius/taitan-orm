use std::fmt::Display;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum SqlType {
    Insert,
    Upsert
}

impl Display for SqlType {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SqlType::Insert => write!(fmt, "Insert"),
            SqlType::Upsert => write!(fmt, "Upsert"),
        }
    }
}