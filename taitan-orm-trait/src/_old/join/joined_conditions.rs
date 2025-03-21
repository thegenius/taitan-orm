use crate::join::joined_condition_array::JoinedConditionArray;
use crate::join::{FromClause, JoinedCondition};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(untagged)]
pub enum JoinedConditions {
    Two(JoinedCondition),
    Three(JoinedConditionArray<1>),
    Four(JoinedConditionArray<2>),
    Five(JoinedConditionArray<3>),
    Six(JoinedConditionArray<4>),
    Seven(JoinedConditionArray<5>),
    Eight(JoinedConditionArray<6>),
    Nine(JoinedConditionArray<7>),
    Ten(JoinedConditionArray<8>),
    Eleven(JoinedConditionArray<9>),
    Twelve(JoinedConditionArray<10>),
}

impl FromClause for JoinedConditions {
    fn get_from_clause(&self) -> String {
        match &self {
            JoinedConditions::Two(e) => e.get_from_clause(),
            JoinedConditions::Three(e) => e.get_from_clause(),
            JoinedConditions::Four(e) => e.get_from_clause(),
            JoinedConditions::Five(e) => e.get_from_clause(),
            JoinedConditions::Six(e) => e.get_from_clause(),
            JoinedConditions::Seven(e) => e.get_from_clause(),
            JoinedConditions::Eight(e) => e.get_from_clause(),
            JoinedConditions::Nine(e) => e.get_from_clause(),
            JoinedConditions::Ten(e) => e.get_from_clause(),
            JoinedConditions::Eleven(e) => e.get_from_clause(),
            JoinedConditions::Twelve(e) => e.get_from_clause(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::join::JoinedMode;

    use super::FromClause;
    use super::JoinedCondition;
    use super::JoinedConditions;
    use serde_json;

    #[test]
    pub fn test_joined_condition() {
        let content = r#"{ "mode": "inner", "left": "user", "right": "class", "fields": [ ["user.id", "class.id"] ]}"#;
        let joined_cond: JoinedCondition = serde_json::from_str(content).unwrap();
        assert_eq!(joined_cond.mode, JoinedMode::Inner);
        assert_eq!(joined_cond.left_table, "user");
        assert_eq!(joined_cond.right_table, "class");
        assert_eq!(joined_cond.joined_fields[0].0.table_name, "user");
        assert_eq!(joined_cond.joined_fields[0].1.table_name, "class");
        assert_eq!(joined_cond.joined_fields[0].0.field_name, "id");
        assert_eq!(joined_cond.joined_fields[0].1.field_name, "id");
    }

    #[test]
    pub fn test_joined_conditions() {
        let content = r#" {"root": { "mode": "inner", "left": "user", "right": "class", "fields": [ ["user.id", "class.id"] ]}, "next":[
            {"mode": "outer", "table": "school", "fields": [["school.id", "user.id"], ["user.name", "school.name"] ] },
            {"mode": "outer", "table": "country",   "fields": [["country.id", "school.id"], ["coutry.name", "user.name"] ]}
        ] }"#;
        let joined_conds: JoinedConditions = serde_json::from_str(content).unwrap();
        match joined_conds {
            JoinedConditions::Four(_) => {}
            _ => panic!("deserialize wrong"),
        }

        let from_clause = joined_conds.get_from_clause();
        assert_eq!(from_clause, "user INNER JOIN class ON user.id = class.id OUTER JOIN school ON school.id = user.id,user.name = school.name OUTER JOIN country ON country.id = school.id,coutry.name = user.name");
    }
}
