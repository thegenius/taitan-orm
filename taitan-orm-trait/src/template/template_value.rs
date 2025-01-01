use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TemplateValue {
    SingleQuoteString(String),
    DoubleQuoteString(String),
    BackQuoteString(String),
    Star(String),
    Segment(String),
    HashVariable(String),
    DollarVariable(String),
}

pub trait InnerString {
    fn inner_string(&self) -> String;
}

impl InnerString for TemplateValue {
    fn inner_string(&self) -> String {
        match &self {
            Self::SingleQuoteString(s) | Self::DoubleQuoteString(s) => s.clone(),
            Self::BackQuoteString(s) | Self::Star(s) => s.clone(),
            Self::HashVariable(v) | Self::DollarVariable(v) => v.clone(),
            Self::Segment(s) => s.clone(),
        }
    }
}

impl Display for TemplateValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            TemplateValue::SingleQuoteString(v)
            | TemplateValue::DoubleQuoteString(v)
            | TemplateValue::BackQuoteString(v)
            | TemplateValue::Star(v)
            | TemplateValue::Segment(v)
            | TemplateValue::HashVariable(v) => v.to_string(),
            | TemplateValue::DollarVariable(v) => v.to_string(),
        };
        write!(f, "{}", str)
    }
}