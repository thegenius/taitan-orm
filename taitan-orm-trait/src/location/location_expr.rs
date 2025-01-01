use crate::CmpOperator;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct LocationExpr<T> {
    pub val: T,
    pub cmp: CmpOperator,
}

pub trait LocationTrait {
    fn get_cmp_sql(&self) -> &str;
}
impl<T> LocationTrait for LocationExpr<T> {
    fn get_cmp_sql(&self) -> &str {
        self.cmp.get_sql()
    }
}

impl<T> LocationExpr<T> {
    pub fn new(cmp: CmpOperator, val: T) -> Self {
        Self { cmp, val }
    }
}
