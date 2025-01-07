use std::fmt::Display;
use crate::template::ToSql;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TemplateVariable {
    Simple(String),
    Quote(String),
}

impl Display for TemplateVariable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TemplateVariable::Simple(val) => write!(f, "{}", val),
            TemplateVariable::Quote(val) => write!(f, "`{}`", val),
        }
    }
}

impl ToSql for TemplateVariable {
    fn to_set_sql(&self) -> String {
        self.to_string()
    }

    fn to_where_sql(&self) -> String {
        self.to_string()
    }
}