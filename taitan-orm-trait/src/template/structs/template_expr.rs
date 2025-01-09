use crate::template::parsed_template_sql::TemplateField;
use crate::template::structs::template_connective::TemplateConnective;
use crate::template::to_sql::SqlTemplateSign;
use crate::template::{TemplatePlaceholder, TemplateVariableChain, ToSql};
use crate::Optional;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TemplateExprFirstPart {
    Dollar(TemplatePlaceholder),
    Variable(TemplateVariableChain),
    Number(String),
}

impl Display for TemplateExprFirstPart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TemplateExprFirstPart::Dollar(val) => write!(f, "{}", val),
            TemplateExprFirstPart::Variable(val) => write!(f, "{}", val),
            TemplateExprFirstPart::Number(val) => write!(f, "{}", val),
        }
    }
}

impl ToSql for TemplateExprFirstPart {
    fn to_set_sql(&self) -> String {
        match self {
            TemplateExprFirstPart::Dollar(val) => val.to_set_sql(),
            TemplateExprFirstPart::Variable(val) => val.to_set_sql(),
            TemplateExprFirstPart::Number(val) => val.to_owned(),
        }
    }

    fn to_where_sql(&self) -> String {
        match self {
            TemplateExprFirstPart::Dollar(val) => val.to_where_sql(),
            TemplateExprFirstPart::Variable(val) => val.to_where_sql(),
            TemplateExprFirstPart::Number(val) => val.to_owned(),
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
pub enum TemplateExpr {
    Simple {
        first_part: TemplateExprFirstPart,
        operator: String,
        second_part: TemplateExprSecondPart,
        index: i32,
        expr_symbol: String,
    },
    Not {
        expr: Box<TemplateExpr>,
        index: i32,
        expr_symbol: String,
    },
    Parenthesized {
        expr: Box<TemplateExpr>,
        index: i32,
        expr_symbol: String,
    },
    And{
        left: Box<TemplateExpr>,
        right: Box<TemplateExpr>,
        index: i32,
        expr_symbol: String,
    },
    Or {
        left: Box<TemplateExpr>,
        right: Box<TemplateExpr>,
        index: i32,
        expr_symbol: String,
    },
}

// #[derive(Debug, Clone, PartialEq, Eq)]
// pub struct IndexedTemplateExpr {
//     pub expr: TemplateExpr,
//     pub index: i32,
//     pub expr_symbol: String,
// }

// impl ToSql for IndexedTemplateExpr {
//     fn to_set_sql(&self) -> String {
//         self.expr.to_set_sql()
//     }
//     fn to_where_sql(&self) -> String {
//         self.expr.to_where_sql()
//     }
// }
//
// impl SqlTemplateSign for IndexedTemplateExpr {
//     fn get_template_signs(&self) -> Vec<String> {
//         self.expr.get_template_signs()
//     }
//     fn get_argument_signs(&self) -> Vec<TemplateField> {
//         self.expr.get_argument_signs()
//     }
// }
//
// impl Display for IndexedTemplateExpr {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", self.expr)
//     }
// }



impl SqlTemplateSign for TemplateExpr {
    fn get_template_signs(&self) -> Vec<String> {
        match self {
            TemplateExpr::Simple {
                first_part,
                operator,
                second_part,..
            } => {
                let mut signs: Vec<String> = vec![];
                let first_part_signs = first_part.get_template_signs();
                let second_part_signs = second_part.get_template_signs();
                signs.extend(first_part_signs);
                signs.extend(second_part_signs);
                signs
            }
            TemplateExpr::Not{expr, ..} => expr.get_template_signs(),
            TemplateExpr::Parenthesized{expr, ..} => expr.get_template_signs(),
            TemplateExpr::And{left, right, ..} => {
                let mut signs1 = left.get_template_signs();
                let mut signs2 = right.get_template_signs();
                signs1.extend(signs2);
                signs1
            }
            TemplateExpr::Or{left, right, ..} => {
                let mut signs1 = left.get_template_signs();
                let mut signs2 = right.get_template_signs();
                signs1.extend(signs2);
                signs1
            }
        }
    }

    fn get_argument_signs(&self) -> Vec<TemplateField> {
        match self {
            TemplateExpr::Simple {
                first_part,
                operator,
                second_part,..
            } => {
                let mut signs: Vec<TemplateField> = vec![];
                let first_part_signs = first_part.get_argument_signs();
                let second_part_signs = second_part.get_argument_signs();
                signs.extend(first_part_signs);
                signs.extend(second_part_signs);
                signs
            }
            TemplateExpr::Not{expr, ..} => expr.get_argument_signs(),
            TemplateExpr::Parenthesized{expr, ..} => expr.get_argument_signs(),
            TemplateExpr::And{left, right, ..} => {
                let mut signs1 = left.get_argument_signs();
                let mut signs2 = right.get_argument_signs();
                signs1.extend(signs2);
                signs1
            }
            TemplateExpr::Or{left, right, ..} => {
                let mut signs1 = left.get_argument_signs();
                let mut signs2 = right.get_argument_signs();
                signs1.extend(signs2);
                signs1
            }
        }
    }
}

impl Display for TemplateExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TemplateExpr::Simple {
                first_part,
                operator,
                second_part,..
            } => {
                write!(f, "{} {} {}", first_part, operator, second_part)
            }
            TemplateExpr::Not{expr,..} => write!(f, "NOT {}", expr),
            TemplateExpr::Parenthesized{expr, ..} => write!(f, "({})", expr),
            TemplateExpr::And{left, right,..} => {
                write!(f, "({} AND {})", left, right)
            }
            TemplateExpr::Or{left, right,..} => {
                write!(f, "({} OR {})", left, right)
            }
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
        match self {
            TemplateExpr::Simple {
                first_part,
                operator,
                second_part,..
            } => match second_part {
                TemplateExprSecondPart::Dollar(val) => {
                    format!("{} = {}", first_part.to_set_sql(), val.to_set_sql())
                }
                TemplateExprSecondPart::Variable(val) => {
                    format!("{} = {}", first_part.to_set_sql(), val.to_set_sql())
                }
                TemplateExprSecondPart::Hash(val) => {
                    format!("{} = {}", first_part.to_set_sql(), val.to_set_sql())
                }
                TemplateExprSecondPart::Number(val) => {
                    format!("{} = {}", first_part.to_set_sql(), val)
                }
                TemplateExprSecondPart::Percent(val) => {
                    format!(
                            "{{% if {}.is_some() %}}{} = ?{{% elif {}.is_null() %}}{}=NULL{{% else %}}{{% endif %}}",
                            val.to_string(),
                            first_part.to_set_sql(),
                            val.to_string(),
                            first_part.to_set_sql(),
                        )
                }
            },
            TemplateExpr::Not{expr, ..} => {
                format!("NOT {}", expr.to_set_sql())
            }
            TemplateExpr::Parenthesized{expr, ..} => {
                format!("({})", expr.to_set_sql())
            }
            TemplateExpr::And{left, right,..} => {
                format!("({} AND {})", left.to_set_sql(), right.to_set_sql())
            }
            TemplateExpr::Or{left, right,..}=> {
                format!("({} OR {})", left.to_set_sql(), right.to_set_sql())
            }
        }
    }

    fn to_where_sql(&self) -> String {
        match self {
            TemplateExpr::Simple {
                first_part,
                operator,
                second_part,..
            } => match second_part {
                TemplateExprSecondPart::Dollar(val) => {
                    format!(
                        "{} {} {}",
                        first_part.to_where_sql(),
                        operator,
                        val.to_where_sql()
                    )
                }
                TemplateExprSecondPart::Variable(val) => {
                    format!(
                        "{} {} {}",
                        first_part.to_where_sql(),
                        operator,
                        val.to_where_sql()
                    )
                }
                TemplateExprSecondPart::Hash(val) => {
                    format!(
                        "{} {} {}",
                        first_part.to_where_sql(),
                        operator,
                        val.to_where_sql()
                    )
                }
                TemplateExprSecondPart::Number(val) => {
                    format!("{} {} {}", first_part.to_where_sql(), operator, val)
                }
                TemplateExprSecondPart::Percent(val) => {
                    if operator.eq("=") {
                        format!(
                                "{{% if {}.is_some() %}}{} {} ?{{% elif {}.is_null() %}}{} IS NULL{{% else %}}{{% endif %}}",
                                val.to_string(),
                                first_part.to_where_sql(),
                                operator,
                                val.to_string(),
                                first_part.to_where_sql(),
                            )
                    } else if operator.eq("<>") {
                        format!(
                                "{{% if {}.is_some() %}}{} {} ?{{% elif {}.is_null() %}}{} IS NOT NULL{{% else %}}{{% endif %}}",
                                val.to_string(),
                                first_part.to_where_sql(),
                                operator,
                                val.to_string(),
                                first_part.to_where_sql(),
                            )
                    } else {
                        format!(
                            "{{% if {}.is_some() %}}{} {} ?{{% else %}}{{% endif %}}",
                            val.to_string(),
                            first_part.to_where_sql(),
                            operator,
                        )
                    }
                }
            },
            TemplateExpr::And{left, right,..} => {
                format!("({} AND {})", left.to_where_sql(), right.to_where_sql())
            }
            TemplateExpr::Or{left, right,..} => {
                format!("({} OR {})", left.to_where_sql(), right.to_where_sql())
            }
            TemplateExpr::Not{expr, ..} => {
                format!("NOT {}", expr.to_where_sql())
            }
            TemplateExpr::Parenthesized{expr, ..} => {
                format!("({})", expr.to_where_sql())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::template::TemplateVariable;
    use crate::template::TemplateVariableChain;
    use crate::template::ToSql;
    use crate::template::{
        TemplateExpr, TemplateExprFirstPart, TemplateExprSecondPart, TemplatePlaceholder,
    };
    use crate::Optional;
    use rinja::Template;

    #[test]
    fn test_expr_set_sql() {
        let simple_variable = TemplateVariableChain {
            variables: vec![TemplateVariable::Simple("first".to_string())],
        };
        let first_part = TemplateExprFirstPart::Variable(simple_variable);

        let placeholder = TemplatePlaceholder::Percent(TemplateVariableChain {
            variables: vec![TemplateVariable::Simple("second".to_string())],
        });
        let second_part = TemplateExprSecondPart::Percent(placeholder);

        let expr = TemplateExpr::Simple {
            first_part,
            operator: "=".to_string(),
            second_part,
            index: 0,
            expr_symbol: "".to_string(),
        };

        let sql = expr.to_set_sql();
        assert_eq!(sql, "{% if second.is_some() %}first = ?{% elif second.is_null() %}first=NULL{% else %}{% endif %}");
    }

    #[derive(Template)]
    #[template(
        source = "{% if second.is_some() %}first = ?{% elif second.is_null() %}first=NULL{% else %}{% endif %}",
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
        assert_eq!(rendered, "first = ?");

        let template = VariableExprTemplate {
            first: "Allen",
            second: Optional::Null,
        };
        let rendered = template.render().unwrap();
        assert_eq!(rendered, "first=NULL");

        let template = VariableExprTemplate {
            first: "Allen",
            second: Optional::None,
        };
        let rendered = template.render().unwrap();
        assert_eq!(rendered, "");
    }

    #[test]
    fn test_dollar_expr_set_sql() {
        let simple_variable = TemplateVariableChain {
            variables: vec![TemplateVariable::Simple("first".to_string())],
        };
        let dollar_variable = TemplatePlaceholder::Dollar(simple_variable);
        let first_part = TemplateExprFirstPart::Dollar(dollar_variable);

        let placeholder = TemplatePlaceholder::Percent(TemplateVariableChain {
            variables: vec![TemplateVariable::Simple("second".to_string())],
        });
        let second_part = TemplateExprSecondPart::Percent(placeholder);

        let expr = TemplateExpr::Simple {
            first_part,
            operator: "=".to_string(),
            second_part,
            index: 0,
            expr_symbol: "".to_string(),
        };

        let sql = expr.to_set_sql();
        assert_eq!(sql, "{% if second.is_some() %}{{first}} = ?{% elif second.is_null() %}{{first}}=NULL{% else %}{% endif %}");
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
        let simple_variable = TemplateVariableChain {
            variables: vec![TemplateVariable::Simple("first".to_string())],
        };
        let first_part = TemplateExprFirstPart::Variable(simple_variable);

        let placeholder = TemplatePlaceholder::Percent(TemplateVariableChain {
            variables: vec![TemplateVariable::Simple("second".to_string())],
        });
        let second_part = TemplateExprSecondPart::Percent(placeholder);

        let expr = TemplateExpr::Simple {
            first_part: first_part.clone(),
            operator: ">=".to_string(),
            second_part: second_part.clone(),
            index: 0,
            expr_symbol: "".to_string(),
        };

        let sql = expr.to_where_sql();
        assert_eq!(
            sql,
            "{% if second.is_some() %}first >= ?{% else %}{% endif %}"
        );

        let expr = TemplateExpr::Simple {
            first_part: first_part.clone(),
            operator: "=".to_string(),
            second_part: second_part.clone(),
            index: 0,
            expr_symbol: "".to_string(),
        };

        let sql = expr.to_where_sql();
        assert_eq!(sql, "{% if second.is_some() %}first = ?{% elif second.is_null() %}first IS NULL{% else %}{% endif %}");

        let expr = TemplateExpr::Simple {
            first_part: first_part.clone(),
            operator: "<>".to_string(),
            second_part: second_part.clone(),
            index: 0,
            expr_symbol: "".to_string(),
        };

        let sql = expr.to_where_sql();
        assert_eq!(sql, "{% if second.is_some() %}first <> ?{% elif second.is_null() %}first IS NOT NULL{% else %}{% endif %}");
    }
}
