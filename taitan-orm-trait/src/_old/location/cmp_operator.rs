use serde::{Deserialize, Serialize};
use crate::error::NotValidCmpError;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum CmpOperator {
    #[serde(alias = "<>")]
    Ne,
    #[serde(alias = "=")]
    Eq,
    #[serde(alias = "<")]
    LessThan,
    #[serde(alias = "<=")]
    LessOrEq,
    #[serde(alias = ">")]
    GreaterThan,
    #[serde(alias = ">=")]
    GreaterOrEq,
    #[serde(alias = "like")]
    Like,
}

impl CmpOperator {
    pub fn get_sql(&self) -> &'static str {
        match self {
            CmpOperator::Ne => "<>",
            CmpOperator::Eq => "=",
            CmpOperator::LessThan => "<",
            CmpOperator::LessOrEq => "<=",
            CmpOperator::GreaterThan => ">",
            CmpOperator::GreaterOrEq => ">=",
            CmpOperator::Like => "LIKE",
        }
    }

    pub fn from_str(cmp: &str)-> Result<Self, NotValidCmpError> {
        match cmp.trim() {
            "<>" => Ok(CmpOperator::Ne),
            "!=" => Ok(CmpOperator::Ne),
            "=" => Ok(CmpOperator::Eq),
            "<" => Ok(CmpOperator::LessThan),
            "<=" => Ok(CmpOperator::LessOrEq),
            ">" => Ok(CmpOperator::GreaterThan),
            ">=" => Ok(CmpOperator::GreaterOrEq),
            "like" => Ok(CmpOperator::Like),
            "Like" => Ok(CmpOperator::Like),
            "LIKE" => Ok(CmpOperator::Like),
            _ => Err(NotValidCmpError(format!("{} is not valid cmp", cmp.to_string())))
        }
    }
}



#[cfg(test)]
mod test {
    use crate::CmpOperator;

    #[test]
    fn test_cmp_operator() {
        let cmp = CmpOperator::from_str("<>").unwrap();
        assert_eq!(cmp, CmpOperator::Ne);
    }

}

