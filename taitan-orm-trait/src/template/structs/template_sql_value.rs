use crate::template::to_sql::SqlTemplateSign;
use crate::template::{
    TemplateConnective, TemplateExpr, TemplatePlaceholder, TemplateString, TemplateVariableChain,
    ToSql,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TemplateSqlValue {
    Number(String),
    Operator(String),
    Segment(String),
    String(TemplateString),
    Connective(TemplateConnective),
    VariableChain(TemplateVariableChain),
    Placeholder(TemplatePlaceholder),
    Expression(TemplateExpr),
}

impl ToSql for TemplateSqlValue {
    fn to_set_sql(&self) -> String {
        match self {
            Self::Number(n) => n.clone(),
            Self::Operator(o) => o.clone(),
            Self::Segment(s) => s.to_string(),
            Self::String(s) => s.to_set_sql(),
            Self::Connective(c) => c.to_set_sql(),
            Self::VariableChain(v) => v.to_set_sql(),
            Self::Placeholder(p) => p.to_set_sql(),
            Self::Expression(e) => e.to_set_sql(),
        }
    }

    fn to_where_sql(&self) -> String {
        match self {
            Self::Number(n) => n.clone(),
            Self::Operator(o) => o.clone(),
            Self::Segment(s) => s.to_string(),
            Self::String(s) => s.to_where_sql(),
            Self::Connective(c) => c.to_where_sql(),
            Self::VariableChain(v) => v.to_where_sql(),
            Self::Placeholder(p) => p.to_where_sql(),
            Self::Expression(e) => e.to_where_sql(),
        }
    }
}

impl SqlTemplateSign for TemplateSqlValue {
    fn get_template_signs(&self) -> Vec<String> {
        match self {
            Self::Placeholder(p) => p.get_template_signs(),
            Self::Expression(e) => e.get_template_signs(),
            _ => vec![],
        }
    }

    fn get_argument_signs(&self) -> Vec<String> {
        match self {
            Self::Placeholder(p) => p.get_argument_signs(),
            Self::Expression(e) => e.get_argument_signs(),
            _ => vec![],
        }
    }
}
