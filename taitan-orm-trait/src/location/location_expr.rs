use crate::error::NotValidCmpError;
use crate::CmpOperator;
use log::warn;
use serde::{Deserialize, Serialize};
use std::str::Chars;




#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct LocationExpr<T> {
    pub val: Option<T>,
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
        Self { cmp, val: Some(val) }
    }

    pub fn from(cmp: &str, val: T) -> Result<Self, NotValidCmpError> {
        let cmp = CmpOperator::from_str(cmp)?;
        Ok(Self { cmp, val: Some(val) })
    }

    pub fn get_cmp_sql(&self) -> &str {
        self.cmp.get_sql()
    }
}
