use crate::template::{TemplateConnective, TemplateExpr, TemplatePlaceholder, TemplateString, TemplateVariableChain};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TemplateSqlValue {
    String(TemplateString),
    Number(String),
    Segment(String),
    Operator(String),
    VariableChain(TemplateVariableChain),
    Expression(TemplateExpr),
    Placeholder(TemplatePlaceholder),
    Connective(TemplateConnective),
}
