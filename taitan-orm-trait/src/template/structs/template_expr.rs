use crate::template::structs::template_connective::TemplateConnective;
use crate::template::{TemplatePlaceholder, TemplateVariableChain, ToSql};
use std::fmt::Display;
use crate::Optional;
use crate::template::parsed_template_sql::TemplateField;
use crate::template::to_sql::SqlTemplateSign;

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
    fn to_set_sql(&self) -> String {
        match self {
            TemplateExprFirstPart::Dollar(val) => val.to_set_sql(),
            TemplateExprFirstPart::Variable(val) => val.to_set_sql(),
        }
    }

    fn to_where_sql(&self) -> String {
        match self {
            TemplateExprFirstPart::Dollar(val) => val.to_where_sql(),
            TemplateExprFirstPart::Variable(val) => val.to_where_sql(),
        }
    }
}

impl SqlTemplateSign for TemplateExprFirstPart {
    fn get_template_signs(&self) -> Vec<String> {
        match self {
            TemplateExprFirstPart::Dollar(val) => val.get_template_signs(),
            _ => vec![],
        }
    }

    fn get_argument_signs(&self) -> Vec<TemplateField> {
        vec![]
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

impl Display for TemplateExprSecondPart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TemplateExprSecondPart::Dollar(val) => write!(f, "{}", val),
            TemplateExprSecondPart::Hash(val) => write!(f, "{}", val),
            TemplateExprSecondPart::Percent(val) => write!(f, "{}", val),
            TemplateExprSecondPart::Variable(val) => write!(f, "{}", val),
            TemplateExprSecondPart::Number(val) => write!(f, "{}", val),
        }
    }
}

impl ToSql for Option<TemplateConnective> {
    fn to_set_sql(&self) -> String {
        match self {
            Some(val) => val.to_set_sql(),
            None => String::new(),
        }
    }
    fn to_where_sql(&self) -> String {
        match self {
            Some(val) => val.to_where_sql(),
            None => String::new(),
        }
    }
}

impl SqlTemplateSign for TemplateExprSecondPart {
    fn get_template_signs(&self) -> Vec<String> {
        match self {
            TemplateExprSecondPart::Percent(val) => val.get_template_signs(),
            TemplateExprSecondPart::Dollar(val) => val.get_template_signs(),
            _ => vec![],
        }
    }
    fn get_argument_signs(&self) -> Vec<TemplateField> {
        match self {
            TemplateExprSecondPart::Percent(val) => val.get_argument_signs(),
            TemplateExprSecondPart::Hash(val) => val.get_argument_signs(),
            _ => vec![],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TemplateExpr {
    pub first_part: TemplateExprFirstPart,
    pub operator: String,
    pub second_part: TemplateExprSecondPart,
    pub connective: Option<TemplateConnective>,
}

impl SqlTemplateSign for TemplateExpr {
    fn get_template_signs(&self) -> Vec<String> {
        let mut signs: Vec<String> = vec![];
        let first_part_signs = self.first_part.get_template_signs();
        let second_part_signs = self.second_part.get_template_signs();
        signs.extend(first_part_signs);
        signs.extend(second_part_signs);
        signs
    }

    fn get_argument_signs(&self) -> Vec<TemplateField> {
        let mut signs: Vec<TemplateField> = vec![];
        let first_part_signs = self.first_part.get_argument_signs();
        let second_part_signs = self.second_part.get_argument_signs();
        signs.extend(first_part_signs);
        signs.extend(second_part_signs);
        signs
    }
}

impl Display for TemplateExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.connective {
            Some(connective) => write!(
                f,
                "{} {} {} {}",
                self.first_part, self.operator, self.second_part, connective
            ),
            None => write!(
                f,
                "{} {} {}",
                self.first_part, self.operator, self.second_part
            ),
        }
    }
}

/// percent expr 应该被转化为
/// {% if val.is_some() %}
/// {{ {} val.unwrap()}}
/// {% elif val.is_null() %}
/// val = NULL
/// {% else %}
/// {% endif %}
impl ToSql for TemplateExpr {
    fn to_set_sql(&self) -> String {
        match &self.second_part {
            TemplateExprSecondPart::Dollar(val) => {
                format!("{} = {}", self.first_part.to_set_sql(), val.to_set_sql())
            },
            TemplateExprSecondPart::Variable(val) => {
                format!("{} = {}", self.first_part.to_set_sql(), val.to_set_sql())
            },
            TemplateExprSecondPart::Hash(val) => {
                format!("{} = {}", self.first_part.to_set_sql(), val.to_set_sql())
            },
            TemplateExprSecondPart::Number(val) => {
                format!("{} = {}", self.first_part.to_set_sql(), val)
            },
            TemplateExprSecondPart::Percent(val) => {
                format!(
                    "{{% if {}.is_some() %}}{} = ? {}{{% elif {}.is_null() %}}{}=NULL {}{{% else %}}{{% endif %}}",
                    val.to_string(),
                    self.first_part.to_set_sql(),
                    self.connective.to_set_sql(),
                    val.to_string(),
                    self.first_part.to_set_sql(),
                    self.connective.to_set_sql()
                )
            }
        }
    }

    fn to_where_sql(&self) -> String {
        match &self.second_part {
            TemplateExprSecondPart::Dollar(val) => {
                format!("{} {} {}", self.first_part.to_where_sql(), self.operator, val.to_where_sql())
            },
            TemplateExprSecondPart::Variable(val) => {
                format!("{} {} {}", self.first_part.to_where_sql(), self.operator, val.to_where_sql())
            },
            TemplateExprSecondPart::Hash(val) => {
                format!("{} {} {}", self.first_part.to_where_sql(), self.operator, val.to_where_sql())
            },
            TemplateExprSecondPart::Number(val) => {
                format!("{} {} {}", self.first_part.to_where_sql(), self.operator, val)
            },
            TemplateExprSecondPart::Percent(val) => {
                if self.operator.eq("=") {
                    format!(
                        "{{% if {}.is_some() %}}{} {} ? {}{{% elif {}.is_null() %}}{} IS NULL {}{{% else %}}{{% endif %}}",
                        val.to_string(),
                        self.first_part.to_where_sql(),
                        self.operator,
                        self.connective.to_where_sql(),
                        val.to_string(),
                        self.first_part.to_where_sql(),
                        self.connective.to_set_sql()
                    )
                } else if self.operator.eq("<>") {
                    format!(
                        "{{% if {}.is_some() %}}{} {} ? {}{{% elif {}.is_null() %}}{} IS NOT NULL {}{{% else %}}{{% endif %}}",
                        val.to_string(),
                        self.first_part.to_where_sql(),
                        self.operator,
                        self.connective.to_where_sql(),
                        val.to_string(),
                        self.first_part.to_where_sql(),
                        self.connective.to_set_sql()
                    )
                } else {
                    format!(
                        "{{% if {}.is_some() %}}{} {} ? {}{{% else %}}{{% endif %}}",
                        val.to_string(),
                        self.first_part.to_where_sql(),
                        self.operator,
                        self.connective.to_set_sql()
                    )
                }
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use rinja::Template;
    use crate::Optional;
    use crate::template::{TemplateExpr, TemplateExprFirstPart, TemplateExprSecondPart, TemplatePlaceholder};
    use crate::template::TemplateVariable;
    use crate::template::TemplateVariableChain;
    use crate::template::ToSql;

    #[test]
    fn test_expr_set_sql() {
        let simple_variable = TemplateVariableChain { variables: vec![TemplateVariable::Simple("first".to_string())] };
        let first_part = TemplateExprFirstPart::Variable(simple_variable);


        let placeholder = TemplatePlaceholder::Percent(TemplateVariableChain {
            variables: vec![TemplateVariable::Simple("second".to_string())],
        });
        let second_part = TemplateExprSecondPart::Percent(placeholder);

        let expr = TemplateExpr {
            first_part,
            operator: "=".to_string(),
            second_part,
            connective: None,
        };

        let sql = expr.to_set_sql();
        assert_eq!(sql, "{% if second.is_some() %}first = ? {% elif second.is_null() %}first=NULL {% else %}{% endif %}");
    }

    #[derive(Template)]
    #[template(
        source = "{% if second.is_some() %}first = ? {% elif second.is_null() %}first=NULL {% else %}{% endif %}",
        ext = "txt"
    )]
    struct VariableExprTemplate<'a> {
        first: &'a str,
        second: Optional<&'a str>,
    }

    #[test]
    fn test_variable_expr() {
        let template = VariableExprTemplate {
            first: "Allen",
            second: Optional::Some("Bob"),
        };
        let rendered = template.render().unwrap();
        assert_eq!(rendered, "first = ? ");

        let template = VariableExprTemplate {
            first: "Allen",
            second: Optional::Null,
        };
        let rendered = template.render().unwrap();
        assert_eq!(rendered, "first=NULL ");

        let template = VariableExprTemplate {
            first: "Allen",
            second: Optional::None,
        };
        let rendered = template.render().unwrap();
        assert_eq!(rendered, "");
    }

    #[test]
    fn test_dollar_expr_set_sql() {
        let simple_variable = TemplateVariableChain { variables: vec![TemplateVariable::Simple("first".to_string())] };
        let dollar_variable = TemplatePlaceholder::Dollar(simple_variable);
        let first_part = TemplateExprFirstPart::Dollar(dollar_variable);


        let placeholder = TemplatePlaceholder::Percent(TemplateVariableChain {
            variables: vec![TemplateVariable::Simple("second".to_string())],
        });
        let second_part = TemplateExprSecondPart::Percent(placeholder);

        let expr = TemplateExpr {
            first_part,
            operator: "=".to_string(),
            second_part,
            connective: None,
        };

        let sql = expr.to_set_sql();
        assert_eq!(sql, "{% if second.is_some() %}{{first}} = ? {% elif second.is_null() %}{{first}}=NULL {% else %}{% endif %}");
    }


    #[derive(Template)]
    #[template(
        source = "{% if second.is_some() %}{{first}} = ?{% elif second.is_null() %}{{first}}=NULL{% else %}{% endif %}",
        ext = "txt"
    )]
    struct DollarExprTemplate<'a> {
        first: &'a str,
        second: Optional<&'a str>,
    }

    #[test]
    fn test_dollar_expr() {
        let template = DollarExprTemplate {
            first: "Allen",
            second: Optional::Some("Bob"),
        };
        let rendered = template.render().unwrap();
        assert_eq!(rendered, "Allen = ?");

        let template = DollarExprTemplate {
            first: "Allen",
            second: Optional::Null,
        };
        let rendered = template.render().unwrap();
        assert_eq!(rendered, "Allen=NULL");

        let template = DollarExprTemplate {
            first: "Allen",
            second: Optional::None,
        };
        let rendered = template.render().unwrap();
        assert_eq!(rendered, "");
    }

    #[test]
    fn test_expr_where_sql() {
        let simple_variable = TemplateVariableChain { variables: vec![TemplateVariable::Simple("first".to_string())] };
        let first_part = TemplateExprFirstPart::Variable(simple_variable);


        let placeholder = TemplatePlaceholder::Percent(TemplateVariableChain {
            variables: vec![TemplateVariable::Simple("second".to_string())],
        });
        let second_part = TemplateExprSecondPart::Percent(placeholder);

        let expr = TemplateExpr {
            first_part: first_part.clone(),
            operator: ">=".to_string(),
            second_part: second_part.clone(),
            connective: None,
        };

        let sql = expr.to_where_sql();
        assert_eq!(sql, "{% if second.is_some() %}first >= ? {% else %}{% endif %}");

        let expr = TemplateExpr {
            first_part: first_part.clone(),
            operator: "=".to_string(),
            second_part: second_part.clone(),
            connective: None,
        };

        let sql = expr.to_where_sql();
        assert_eq!(sql, "{% if second.is_some() %}first = ? {% elif second.is_null() %}first IS NULL {% else %}{% endif %}");

        let expr = TemplateExpr {
            first_part: first_part.clone(),
            operator: "<>".to_string(),
            second_part: second_part.clone(),
            connective: None,
        };

        let sql = expr.to_where_sql();
        assert_eq!(sql, "{% if second.is_some() %}first <> ? {% elif second.is_null() %}first IS NOT NULL {% else %}{% endif %}");
    }
}