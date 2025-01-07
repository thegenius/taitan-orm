use std::fmt::Display;
use std::fs::OpenOptions;

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TemplateVariableChain {
    pub variables: Vec<String>,
}

impl Display for TemplateVariableChain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.variables.join(".").fmt(f)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TemplatePlaceholder {
    Dollar(TemplateVariableChain),
    Hash(TemplateVariableChain),
    Percent(TemplateVariableChain),
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TemplateExprFirstPart {
    Dollar(TemplatePlaceholder),
    Variable(TemplateVariableChain),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TemplateExprSecondPart {
    Dollar(TemplatePlaceholder),
    Hash(TemplatePlaceholder),
    Percent(TemplatePlaceholder),
    Variable(TemplateVariableChain),
    Number(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TemplatePlaceholderExpr {
    pub variable_chain: TemplateVariableChain,
    pub operator: String,
    pub placeholder: TemplatePlaceholder,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TemplateVariableExpr {
    pub variable_chain: TemplateVariableChain,
    pub operator: String,
    pub placeholder: TemplateVariableChain,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TemplateExpr {
    pub first_part: TemplateExprFirstPart,
    pub operator: String,
    pub second_part: TemplateExprSecondPart
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TemplateSqlValue {
    String(TemplateString),
    Number(String),
    Segment(String),
    Operator(String),
    VariableChain(TemplateVariableChain),
    Expression(TemplateExpr),
    Placeholder(TemplatePlaceholder),
}


//
//
// #[derive(Debug, Clone, PartialEq, Eq)]
// pub enum TemplateValue {
//     SingleQuoteString(String),
//     DoubleQuoteString(String),
//     BackQuoteString(String),
//     Star(String),
//     Segment(String),
//     Operator(String),
//     HashVariable(String),
//     DollarVariable(String),
//     PercentVariable(String)
// }
//
// pub trait InnerString {
//     fn inner_string(&self) -> String;
// }
//
// impl InnerString for TemplateValue {
//     fn inner_string(&self) -> String {
//         match &self {
//             Self::SingleQuoteString(s) | Self::DoubleQuoteString(s) => s.clone(),
//             Self::BackQuoteString(s) | Self::Star(s) => s.clone(),
//             Self::HashVariable(v) | Self::DollarVariable(v) => v.clone(),
//             Self::Segment(s) => s.clone(),
//             Self::Operator(s) => s.clone(),
//             Self::PercentVariable(field) => field.clone(),
//         }
//     }
// }
//
// impl Display for TemplateValue {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let str = match self {
//             TemplateValue::SingleQuoteString(v)
//             | TemplateValue::DoubleQuoteString(v)
//             | TemplateValue::BackQuoteString(v)
//             | TemplateValue::Star(v)
//             | TemplateValue::Segment(v)
//             | TemplateValue::HashVariable(v) => v.to_string(),
//             | TemplateValue::DollarVariable(v) => v.to_string(),
//             | TemplateValue::PercentVariable(field)=> field.to_string(),
//             | TemplateValue::Operator(v) => v.to_string(),
//         };
//         write!(f, "{}", str)
//     }
// }