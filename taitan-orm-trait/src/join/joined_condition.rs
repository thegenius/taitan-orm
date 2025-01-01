use crate::join::{FromClause, JoinedFields, JoinedMode};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct JoinedCondition {
    pub(crate) mode: JoinedMode,
    #[serde(alias = "left")]
    pub(crate) left_table: String,
    #[serde(alias = "right")]
    pub(crate) right_table: String,
    #[serde(alias = "fields")]
    pub(crate) joined_fields: Vec<JoinedFields>,
}

pub fn get_on_clause(joined_fields: &Vec<JoinedFields>) -> String {
    let mut on_clause_vec: Vec<String> = Vec::new();
    for field in joined_fields {
        let on_seg = format!(
            "{}.{} = {}.{}",
            field.0.table_name, field.0.field_name, field.1.table_name, field.1.field_name
        );
        on_clause_vec.push(on_seg);
    }
    on_clause_vec.join(",")
}

impl FromClause for JoinedCondition {
    fn get_from_clause(&self) -> String {
        let on_clause = get_on_clause(&self.joined_fields);
        let join_operator = self.mode.get_join_operator();
        format!(
            "{} {} {} ON {}",
            self.left_table, join_operator, self.right_table, on_clause
        )
    }
}
