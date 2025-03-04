use std::fmt::Display;
use crate::template_parser::to_sql::SqlTemplateSign;
use crate::template_parser::ToSql;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TemplateConnective {
    And(String),
    Or(String),
    Comma(String)
}

impl Display for TemplateConnective {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::And(s) => fmt.write_str(s),
            Self::Or(s) => fmt.write_str(s),
            Self::Comma(s) => fmt.write_str(s)
        }
    }
}

impl ToSql for TemplateConnective {
    fn to_set_sql(&self) -> String {
        self.to_string()
    }

    fn to_where_sql(&self) -> String {
        self.to_string()
    }
}

impl SqlTemplateSign for TemplateConnective {}