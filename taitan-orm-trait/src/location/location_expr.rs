use std::str::Chars;
use crate::CmpOperator;
use log::warn;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct LocationExpr<T> {
    pub val: T,
    pub cmp: CmpOperator,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum LogicalOperator {
    And,
    Or,
    Not,
}

impl<T> LocationExpr<T> {
    pub fn new(cmp: CmpOperator, val: T) -> Self {
        Self { cmp, val }
    }

    pub fn get_cmp_sql(&self) -> &str {
        self.cmp.get_sql()
    }
}

