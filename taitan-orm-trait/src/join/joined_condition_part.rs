use crate::join::{get_on_clause, FromClause, JoinedFields, JoinedMode};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct JoinedConditionPart {
    mode: JoinedMode,
    table: String,
    #[serde(alias = "fields")]
    joined_fields: Vec<JoinedFields>,
}

impl FromClause for JoinedConditionPart {
    fn get_from_clause(&self) -> String {
        let on_clause = get_on_clause(&self.joined_fields);
        let join_operator = self.mode.get_join_operator();
        format!("{} {} ON {}", join_operator, self.table, on_clause)
    }
}
