use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub enum JoinedMode {
    #[serde(alias = "left")]
    Left,
    #[serde(alias = "right")]
    Right,
    #[serde(alias = "outer")]
    Outer,
    #[serde(alias = "inner")]
    Inner,
}

impl JoinedMode {
    pub fn get_join_operator(&self) -> &'static str {
        match self {
            JoinedMode::Left => "LEFT JOIN",
            JoinedMode::Right => "RIGHT JOIN",
            JoinedMode::Outer => "OUTER JOIN",
            JoinedMode::Inner => "INNER JOIN",
        }
    }
}
