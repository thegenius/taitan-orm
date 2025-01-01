use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum CmpOperator {
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

impl CmpOperator {
    pub fn get_sql(&self) -> &'static str {
        match self {
            CmpOperator::Eq => "=",
            CmpOperator::LessThan => "<",
            CmpOperator::LessOrEq => "<=",
            CmpOperator::GreaterThan => ">",
            CmpOperator::GreaterOrEq => ">=",
            CmpOperator::Like => "LIKE",
        }
    }
}
