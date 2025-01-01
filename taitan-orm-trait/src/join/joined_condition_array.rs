use crate::join::joined_condition_part::JoinedConditionPart;
use crate::join::{FromClause, JoinedCondition};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

#[serde_as]
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct JoinedConditionArray<const N: usize> {
    root: JoinedCondition,
    #[serde_as(as = "[_; N]")]
    next: [JoinedConditionPart; N],
}

impl<const N: usize> JoinedConditionArray<N> {
    pub fn get_from_clause(&self) -> String {
        let root_join = self.root.get_from_clause();
        let mut part_clauses: Vec<String> = Vec::new();
        for part in &self.next {
            let part_clause = part.get_from_clause();
            part_clauses.push(part_clause);
        }
        let part_clause = part_clauses.join(" ");
        format!("{} {}", root_join, part_clause)
    }
}
