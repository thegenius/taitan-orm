use crate::template::parsed_template_sql::TemplateField;
use crate::template::structs::template_connective::TemplateConnective;
use crate::template::to_sql::SqlTemplateSign;
use crate::template::{TemplatePlaceholder, TemplateVariableChain, ToSql};
use crate::Optional;
use nom::sequence::pair;
use rinja::filters::format;
use std::fmt::{Display, Formatter};
use std::ops::Deref;

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
pub enum UnitOptionalContext {
    NotOptional,
    Optional { variables: Vec<String> },
}
impl UnitOptionalContext {
    pub fn is_optional(&self) -> bool {
        match self {
            UnitOptionalContext::NotOptional => false,
            _ => true,
        }
    }
    pub fn get_variables(&self) -> Vec<String> {
        match self {
            UnitOptionalContext::NotOptional => vec![],
            UnitOptionalContext::Optional { variables } => variables.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PairOptionalContext {
    NotOptional,
    LeftOptional {
        variables: Vec<String>,
    },
    RightOptional {
        variables: Vec<String>,
    },
    BothOptional {
        left_variables: Vec<String>,
        right_variables: Vec<String>,
    },
}

impl PairOptionalContext {
    pub fn is_optional(&self) -> bool {
        match self {
            PairOptionalContext::BothOptional { .. } => true,
            _ => false,
        }
    }
    pub fn get_variables(&self) -> Vec<String> {
        match self {
            PairOptionalContext::NotOptional => vec![],
            PairOptionalContext::LeftOptional { variables } => variables.to_owned(),
            PairOptionalContext::RightOptional { variables } => variables.to_owned(),
            PairOptionalContext::BothOptional {
                left_variables,
                right_variables,
            } => {
                let mut variables = Vec::new();
                variables.extend(left_variables.to_owned());
                variables.extend(right_variables.to_owned());
                variables
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OptionalContext {
    UnitOptional(UnitOptionalContext),
    PairOptional(PairOptionalContext),
}

impl OptionalContext {
    pub fn is_optional(&self) -> bool {
        match self {
            OptionalContext::UnitOptional(ctx) => match ctx {
                UnitOptionalContext::NotOptional => false,
                _ => true,
            },
            OptionalContext::PairOptional(ctx) => match ctx {
                PairOptionalContext::BothOptional { .. } => true,
                _ => false,
            },
        }
    }

    pub fn get_variables(&self) -> Vec<String> {
        match self {
            OptionalContext::UnitOptional(unit) => unit.get_variables(),
            OptionalContext::PairOptional(pair) => pair.get_variables(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TemplateExpr {
    Simple {
        first_part: TemplateExprFirstPart,
        operator: String,
        second_part: TemplateExprSecondPart,
        optional_context: UnitOptionalContext,
    },
    Not {
        expr: Box<TemplateExpr>,
        optional_context: OptionalContext,
    },
    Parenthesized {
        expr: Box<TemplateExpr>,
        optional_context: OptionalContext,
    },
    And {
        left: Box<TemplateExpr>,
        right: Box<TemplateExpr>,
        optional_context: PairOptionalContext,
    },
    Or {
        left: Box<TemplateExpr>,
        right: Box<TemplateExpr>,
        optional_context: PairOptionalContext,
    },
}

impl TemplateExpr {
    pub fn get_optional_variables(&self) -> Vec<String> {
        match self {
            TemplateExpr::Simple {
                optional_context, ..
            } => match optional_context {
                UnitOptionalContext::NotOptional => vec![],
                UnitOptionalContext::Optional { variables } => variables.to_owned(),
            },
            TemplateExpr::Not {
                optional_context, ..
            }
            | TemplateExpr::Parenthesized {
                optional_context, ..
            } => match optional_context {
                OptionalContext::PairOptional(context) => context.get_variables(),
                OptionalContext::UnitOptional(context) => context.get_variables(),
            },
            TemplateExpr::And {
                optional_context, ..
            }
            | TemplateExpr::Or {
                optional_context, ..
            } => optional_context.get_variables(),
        }
    }

    pub fn pop_optional_context(&mut self) -> OptionalContext {
        match self {
            TemplateExpr::Simple {
                optional_context, ..
            } => {
                let ctx = OptionalContext::UnitOptional(optional_context.clone());
                *optional_context = UnitOptionalContext::NotOptional;
                ctx
            }
            TemplateExpr::Not {
                optional_context, ..
            }
            | TemplateExpr::Parenthesized {
                optional_context, ..
            } => {
                let ctx = optional_context.clone();
                match optional_context {
                    OptionalContext::PairOptional(context) => {
                        *optional_context =
                            OptionalContext::PairOptional(PairOptionalContext::NotOptional)
                    }
                    OptionalContext::UnitOptional(context) => {
                        *optional_context =
                            OptionalContext::UnitOptional(UnitOptionalContext::NotOptional);
                    }
                }
                ctx
            }
            TemplateExpr::And {
                optional_context, ..
            } => OptionalContext::PairOptional(optional_context.clone()),
            TemplateExpr::Or {
                optional_context, ..
            } => OptionalContext::PairOptional(optional_context.clone()),
        }
    }
    pub fn get_optional_context(&self) -> OptionalContext {
        match self {
            TemplateExpr::Simple {
                optional_context, ..
            } => OptionalContext::UnitOptional(optional_context.clone()),
            TemplateExpr::Not {
                optional_context, ..
            } => optional_context.clone(),
            TemplateExpr::Parenthesized {
                optional_context, ..
            } => optional_context.clone(),
            TemplateExpr::And {
                optional_context, ..
            } => OptionalContext::PairOptional(optional_context.clone()),
            TemplateExpr::Or {
                optional_context, ..
            } => OptionalContext::PairOptional(optional_context.clone()),
        }
    }

    pub fn is_optional(&self) -> bool {
        match self {
            TemplateExpr::Simple {
                optional_context, ..
            } => match optional_context {
                UnitOptionalContext::NotOptional => false,
                UnitOptionalContext::Optional { .. } => true,
            },
            TemplateExpr::Not {
                optional_context, ..
            }
            | TemplateExpr::Parenthesized {
                optional_context, ..
            } => match optional_context {
                OptionalContext::UnitOptional(ctx) => match ctx {
                    UnitOptionalContext::NotOptional => false,
                    _ => true,
                },
                OptionalContext::PairOptional(ctx) => match ctx {
                    PairOptionalContext::NotOptional => false,
                    _ => true,
                },
            },
            TemplateExpr::And {
                optional_context, ..
            } => match optional_context {
                PairOptionalContext::BothOptional { .. } => true,
                _ => false,
            },
            TemplateExpr::Or {
                optional_context, ..
            } => match optional_context {
                PairOptionalContext::BothOptional { .. } => true,
                _ => false,
            },
        }
    }
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
                second_part,
                ..
            } => {
                let mut signs: Vec<String> = vec![];
                let first_part_signs = first_part.get_template_signs();
                let second_part_signs = second_part.get_template_signs();
                signs.extend(first_part_signs);
                signs.extend(second_part_signs);
                signs
            }
            TemplateExpr::Not { expr, .. } => expr.get_template_signs(),
            TemplateExpr::Parenthesized { expr, .. } => expr.get_template_signs(),
            TemplateExpr::And { left, right, .. } => {
                let mut signs1 = left.get_template_signs();
                let mut signs2 = right.get_template_signs();
                signs1.extend(signs2);
                signs1
            }
            TemplateExpr::Or { left, right, .. } => {
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
                second_part,
                ..
            } => {
                let mut signs: Vec<TemplateField> = vec![];
                let first_part_signs = first_part.get_argument_signs();
                let second_part_signs = second_part.get_argument_signs();
                signs.extend(first_part_signs);
                signs.extend(second_part_signs);
                signs
            }
            TemplateExpr::Not { expr, .. } => expr.get_argument_signs(),
            TemplateExpr::Parenthesized { expr, .. } => expr.get_argument_signs(),
            TemplateExpr::And { left, right, .. } => {
                let mut signs1 = left.get_argument_signs();
                let mut signs2 = right.get_argument_signs();
                signs1.extend(signs2);
                signs1
            }
            TemplateExpr::Or { left, right, .. } => {
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
                second_part,
                ..
            } => {
                write!(f, "{} {} {}", first_part, operator, second_part)
            }
            TemplateExpr::Not { expr, .. } => write!(f, "NOT {}", expr),
            TemplateExpr::Parenthesized { expr, .. } => write!(f, "({})", expr),
            TemplateExpr::And { left, right, .. } => {
                write!(f, "({} AND {})", left, right)
            }
            TemplateExpr::Or { left, right, .. } => {
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
                second_part,
                ..
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
            /// Not语句不能转化为set语句
            TemplateExpr::Not { expr, .. } => "".to_string(),
            TemplateExpr::Parenthesized { expr, .. } => {
                format!("({})", expr.to_set_sql())
            }
            /// And语句不能转化为set语句
            TemplateExpr::And { left, right, .. } => "".to_string(),
            /// Or语句不能转化为set语句
            TemplateExpr::Or { left, right, .. } => "".to_string(),
        }
    }

    fn to_where_sql(&self) -> String {
        match self {
            TemplateExpr::Simple {
                first_part,
                operator,
                second_part,
                ..
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
                            "{{% if {}.is_some() %}}{} = ?{{% elif {}.is_null() %}}{} IS NULL{{% else %}}{{% endif %}}",
                            val.to_string(),
                            first_part.to_where_sql(),
                            val.to_string(),
                            first_part.to_where_sql(),
                        )
                    } else if operator.eq("<>") {
                        format!(
                            "{{% if {}.is_some() %}}{} <> ?{{% elif {}.is_null() %}}{} IS NOT NULL{{% else %}}{{% endif %}}",
                            val.to_string(),
                            first_part.to_where_sql(),
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
            TemplateExpr::And { left, right, .. } => {
                if self.is_optional() {
                    let variables = self.get_optional_variables();
                    let check_some_conditions = variables
                        .iter()
                        .map(|v| format!("{}.is_some()", v))
                        .collect::<Vec<String>>()
                        .join(" && ");
                    let render_and =
                        format!("{{% if {} %}} AND {{% endif %}}", check_some_conditions);
                    format!(
                        "({} {} {})",
                        left.to_where_sql(),
                        render_and,
                        right.to_where_sql()
                    )
                } else {
                    format!("({} AND {})", left.to_where_sql(), right.to_where_sql())
                }
            }
            TemplateExpr::Or { left, right, .. } => {
                if self.is_optional() {
                    let variables = self.get_optional_variables();
                    let check_some_conditions = variables
                        .iter()
                        .map(|v| format!("{}.is_some()", v))
                        .collect::<Vec<String>>()
                        .join(" && ");
                    let render_or =
                        format!("{{% if {} %}} OR {{% endif %}}", check_some_conditions);
                    format!(
                        "({} {} {})",
                        left.to_where_sql(),
                        render_or,
                        right.to_where_sql()
                    )
                } else {
                    format!("({} OR {})", left.to_where_sql(), right.to_where_sql())
                }
            }

            /// not 语句只能转化成where sql，不能转化成set sql
            /// 1. 关于not的嵌套
            ///    多个not直接嵌套后会被优化为单个not
            /// 2. 关于not的子元素为simple
            ///    2.1 not expr的子元素如果是optional的，且比较符是 = 和 <>
            ///         2.1.1 not age = %{age} 应该渲染为
            ///             {% if age.is_some() %}not age =  ?{% else if age.is_null() %} age IS NOT NULL {% else %}{% endif %}
            ///         2.1.2 not age <> %{age} 应该渲染为
            ///             {% if age.is_some() %}not age <> ?{% else if age.is_null() %} age IS NULL {% else %}{% endif %}
            ///    2.2 not expr的子元素如果是optional的，且比较符不是 = 和 <>
            ///         not age >= %{age} 应该渲染为
            ///         {% if age.is_some() %}not age >= ?{% else %}{% endif %}
            /// 3. 关于not的子元素是()或者and或者or
            /// not (age = %{age} AND name = %{name})应该渲染为
            /// {% if age.is_some() && name.is_some() %} NOT {% endif %} (age = ? AND name = ?)
            TemplateExpr::Not {
                expr,
                optional_context,
            } => {
                match expr.as_ref() {
                    TemplateExpr::Not { expr, .. } => expr.to_where_sql(),
                    TemplateExpr::Parenthesized {
                        expr,
                        optional_context,
                        ..
                    } => {
                        let variables = optional_context.get_variables();
                        let check_some_conditions = variables
                            .iter()
                            .map(|v| format!("{}.is_some()", v))
                            .collect::<Vec<String>>()
                            .join(" && ");
                        format!(
                            "{{% if {} %}} NOT {{% endif %}} ({})",
                            check_some_conditions,
                            expr.to_where_sql()
                        )
                    }
                    TemplateExpr::And {
                        optional_context, ..
                    }
                    | TemplateExpr::Or {
                        optional_context, ..
                    } => {
                        let variables = optional_context.get_variables();
                        let check_some_conditions = variables
                            .iter()
                            .map(|v| format!("{}.is_some()", v))
                            .collect::<Vec<String>>()
                            .join(" && ");
                        format!(
                            "{{% if {} %}} NOT {{% endif %}}{}",
                            check_some_conditions,
                            expr.to_where_sql()
                        )
                    }
                    TemplateExpr::Simple {
                        first_part,
                        second_part,
                        operator,
                        ..
                    } => {
                        // panic!("{:?}", expr);
                        if self.is_optional() {
                            let variables = self.get_optional_variables();
                            let variable = variables.first().unwrap();
                            // {% if age.is_some() %}NOT age =  ?{% else if age.is_null() %}NOT age IS NULL{% else %}{% endif %}
                            if operator.eq("=") {
                                format!(
                                    "{{% if {}.is_some() %}} NOT {} = ? {{% else if {}.is_null() %}} {} IS NOT NULL {{% else %}}{{% endif %}}",
                                    variable,
                                    variable,
                                    variable,
                                    variable,
                                )
                            } else if operator.eq("<>") {
                                format!(
                                    "{{% if {}.is_some() %}}NOT {} <> ?{{% else if {}.is_null() %}}{} IS NULL {{% else %}}{{% endif %}}",
                                    variable,
                                    variable,
                                    variable,
                                    variable,
                                )
                            } else {
                                format!(
                                    "{{% if {}.is_some() %}}NOT {} {} ?{{% else %}}{{% endif %}}",
                                    variable, variable, operator,
                                )
                            }
                        } else {
                            format!("NOT {}", expr.to_where_sql())
                        }
                    }
                }
            }
            TemplateExpr::Parenthesized { expr, .. } => {
                format!("({})", expr.to_where_sql())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::template::structs::template_expr::UnitOptionalContext;
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
            optional_context: UnitOptionalContext::NotOptional,
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
            optional_context: UnitOptionalContext::NotOptional,
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
            optional_context: UnitOptionalContext::NotOptional,
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
            optional_context: UnitOptionalContext::NotOptional,
        };

        let sql = expr.to_where_sql();
        assert_eq!(sql, "{% if second.is_some() %}first = ?{% elif second.is_null() %}first IS NULL{% else %}{% endif %}");

        let expr = TemplateExpr::Simple {
            first_part: first_part.clone(),
            operator: "<>".to_string(),
            second_part: second_part.clone(),
            optional_context: UnitOptionalContext::NotOptional,
        };

        let sql = expr.to_where_sql();
        assert_eq!(sql, "{% if second.is_some() %}first <> ?{% elif second.is_null() %}first IS NOT NULL{% else %}{% endif %}");
    }
}
