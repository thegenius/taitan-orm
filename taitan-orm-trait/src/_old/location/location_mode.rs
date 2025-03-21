use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum LocationMode {
    #[serde(alias = "AND")]
    And,
    #[serde(alias = "OR")]
    Or,
}

impl LocationMode {
    pub fn as_str(&self) -> &str {
        match self {
            Self::And => "AND",
            Self::Or => "OR",
        }
    }
    pub fn as_connective(&self) -> &str {
        match self {
            Self::And => " AND ",
            Self::Or => " OR ",
        }
    }
}

impl Default for LocationMode {
    fn default() -> Self {
        Self::And
    }
}