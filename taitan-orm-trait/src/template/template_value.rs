use std::fmt::Display;
use std::fs::OpenOptions;

pub trait ToSql {
    fn to_sql(&self) -> String;
}

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
    fn to_sql(&self) -> String {
        self.to_string()
    }
}

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
    fn to_sql(&self) -> String {
        self.to_string()
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TemplateVariableChain {
    pub variables: Vec<TemplateVariable>,
}

impl Display for TemplateVariableChain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // 使用 Iterator 的 map 和 collect 来构建一个字符串向量，
        // 然后使用 join 方法将它们连接起来。
        let strings: Vec<String> = self.variables.iter().map(|v| v.to_string()).collect();
        write!(f, "{}", strings.join("."))
    }
}

impl ToSql for TemplateVariableChain {
    fn to_sql(&self) -> String {
        self.to_string()
    }
}



#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TemplatePlaceholder {
    Dollar(TemplateVariableChain),
    Hash(TemplateVariableChain),
    Percent(TemplateVariableChain),
}

impl Display for TemplatePlaceholder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TemplatePlaceholder::Dollar(val) => write!(f, "{}", val),
            TemplatePlaceholder::Hash(val) => write!(f, "{}", val),
            TemplatePlaceholder::Percent(val) => write!(f, "{}", val),
        }
    }
}

impl ToSql for TemplatePlaceholder {
    fn to_sql(&self) -> String {
        match self {
            TemplatePlaceholder::Dollar(val) => val.to_sql(),
            TemplatePlaceholder::Hash(val) => val.to_sql(),
            TemplatePlaceholder::Percent(val) => val.to_sql(),
        }
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TemplateExprFirstPart {
    Dollar(TemplatePlaceholder),
    Variable(TemplateVariableChain),
}

impl Display for TemplateExprFirstPart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TemplateExprFirstPart::Dollar(val) => write!(f, "{}", val),
            TemplateExprFirstPart::Variable(val) => write!(f, "{}", val),
        }
    }
}

impl ToSql for TemplateExprFirstPart {
    fn to_sql(&self) -> String {
        match self {
            TemplateExprFirstPart::Dollar(val) => val.to_sql(),
            TemplateExprFirstPart::Variable(val) => val.to_sql(),
        }
    }
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
    pub second_part: TemplateExprSecondPart,
    pub connective: Option<TemplateConnective>
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TemplateConnective {
    And(String),
    Or(String),
    Comma(String)
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
    Connective(TemplateConnective),
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