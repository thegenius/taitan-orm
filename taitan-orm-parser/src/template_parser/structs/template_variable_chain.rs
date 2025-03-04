use std::fmt::Display;
use crate::template_parser::{TemplateVariable, ToSql};
use crate::template_parser::to_sql::SqlTemplateSign;

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
    fn to_set_sql(&self) -> String {
        self.to_string()
    }

    fn to_where_sql(&self) -> String {
        self.to_string()
    }
}

impl SqlTemplateSign for TemplateVariableChain {}