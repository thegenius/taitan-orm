use crate::error::NotValidCmpError;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum Cmp {
    #[serde(alias = "<>")]
    Ne,
    #[serde(alias = "=")]
    Eq,
    #[serde(alias = "<")]
    LessThan,
    #[serde(alias = "<=")]
    LessOrEq,
    #[serde(alias = ">")]
    GreaterThan,
    #[serde(alias = ">=")]
    GreaterOrEq,
    #[serde(alias = "like")]
    Like,
}

impl Cmp {
    pub fn get_sql(&self) -> &'static str {
        match self {
            Cmp::Ne => "<>",
            Cmp::Eq => "=",
            Cmp::LessThan => "<",
            Cmp::LessOrEq => "<=",
            Cmp::GreaterThan => ">",
            Cmp::GreaterOrEq => ">=",
            Cmp::Like => "LIKE",
        }
    }

    pub fn from_str(cmp: &str) -> Result<Self, NotValidCmpError> {
        match cmp.trim() {
            "<>" => Ok(Cmp::Ne),
            "!=" => Ok(Cmp::Ne),
            "=" => Ok(Cmp::Eq),
            "<" => Ok(Cmp::LessThan),
            "<=" => Ok(Cmp::LessOrEq),
            ">" => Ok(Cmp::GreaterThan),
            ">=" => Ok(Cmp::GreaterOrEq),
            "like" => Ok(Cmp::Like),
            "Like" => Ok(Cmp::Like),
            "LIKE" => Ok(Cmp::Like),
            _ => Err(NotValidCmpError(format!(
                "{} is not valid cmp",
                cmp.to_string()
            ))),
        }
    }
}
