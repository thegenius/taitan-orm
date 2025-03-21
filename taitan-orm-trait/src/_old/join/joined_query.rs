use crate::join::joined_conditions::JoinedConditions;
use crate::location::LocatedQuery;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct JoinedQuery {
    query_vec: Vec<Box<dyn LocatedQuery>>,
    join_conditions: JoinedConditions,
}
