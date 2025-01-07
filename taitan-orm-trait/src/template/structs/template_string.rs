use std::fmt::Display;
use crate::template::ToSql;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TemplateString {
    SingleQuoteString(String),
    DoubleQuoteString(String)
}

impl Display for TemplateString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            TemplateString::SingleQuoteString(string) => string.to_owned(),
            TemplateString::DoubleQuoteString(string) => string.to_owned()
        };
        write!(f, "{}", str)
    }
}
impl ToSql for TemplateString {
    fn to_set_sql(&self) -> String {
        self.to_string()
    }

    fn to_where_sql(&self) -> String {
        self.to_string()
    }
}