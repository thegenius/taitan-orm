use crate::template::parsers::{parse_operator, parse_placeholder, parse_segment, parse_string};
use nom::branch::alt;
use nom::bytes::streaming::tag;
use nom::character::complete::{multispace0, multispace1};
use nom::multi::separated_list1;
use nom::IResult;

use crate::template::parsers::express_parser::parse_expr_as_value;
use crate::template::parsers::number_parser::parse_number_as_value;
use crate::template::parsers::operator_parser::parse_operator_as_value;
use crate::template::parsers::placeholder_parser::parse_placeholder_as_value;
use crate::template::parsers::segment_parser::parse_segment_as_value;
use crate::template::parsers::string_parser::parse_string_as_value;
use crate::template::parsers::variable_parser::parse_variable_chain_as_value;
use crate::template::{
    TemplateExpr, TemplatePlaceholder, TemplateSqlValue, TemplateString, TemplateVariableChain,
};
use nom::sequence::tuple;

pub fn parse_template_sql_value(input: &str) -> IResult<&str, TemplateSqlValue> {
    alt((
        parse_expr_as_value,
        parse_placeholder_as_value,
        parse_string_as_value,
        parse_segment_as_value,
        parse_variable_chain_as_value,
        parse_operator_as_value,
        parse_number_as_value,
    ))(input)
}

pub fn parse_template_sql_values(input: &str) -> IResult<&str, Vec<TemplateSqlValue>> {
    // 解析多个 SqlValue，由逗号分隔并允许空格
    separated_list1(
        multispace1,              // 逗号分隔符，允许前后有空格
        parse_template_sql_value, // 解析单个 SqlValue
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::template::TemplateConnective::And;
    use crate::template::TemplateVariable;
    use crate::template::{TemplateExprFirstPart, TemplateExprSecondPart, TemplateVariableChain};




    #[test]
    fn test_parse_template_sql_value() {
        let (remaining, parsed) =
            parse_template_sql_values("SELECT * `test` user ${v1. v2. v3} where id = 23 ").unwrap();
        assert_eq!(
            parsed,
            vec![
                TemplateSqlValue::VariableChain(TemplateVariableChain {
                    variables: vec![TemplateVariable::Simple("SELECT".to_string())]
                }),
                TemplateSqlValue::Segment("*".to_string()),
                TemplateSqlValue::VariableChain(TemplateVariableChain {
                    variables: vec![TemplateVariable::Quote("test".to_string())]
                }),
                TemplateSqlValue::VariableChain(TemplateVariableChain {
                    variables: vec![TemplateVariable::Simple("user".to_string())]
                }),
                TemplateSqlValue::Placeholder(TemplatePlaceholder::Dollar(TemplateVariableChain {
                    variables: vec![
                        TemplateVariable::Simple("v1".to_string()),
                        TemplateVariable::Simple("v2".to_string()),
                        TemplateVariable::Simple("v3".to_string())
                    ]
                })),
                TemplateSqlValue::VariableChain(TemplateVariableChain {
                    variables: vec![TemplateVariable::Simple("where".to_string())]
                }),
                TemplateSqlValue::Expression(TemplateExpr {
                    first_part: TemplateExprFirstPart::Variable(TemplateVariableChain {
                        variables: vec![TemplateVariable::Simple("id".to_string())]
                    }),
                    operator: "=".to_string(),
                    second_part: TemplateExprSecondPart::Number("23".to_string()),
                    connective: None
                }),
            ]
        );
    }

    #[test]
    fn test_parse_template_sql_value2() {
        let (remaining, parsed) = parse_template_sql_values(
            "select * from `user` WHERE name = %{name} AND age = #{age} ",
        )
        .unwrap();
        assert_eq!(
            parsed,
            vec![
                TemplateSqlValue::VariableChain(TemplateVariableChain {
                    variables: vec![TemplateVariable::Simple("select".to_string())]
                }),
                TemplateSqlValue::Segment("*".to_string()),
                TemplateSqlValue::VariableChain(TemplateVariableChain {
                    variables: vec![TemplateVariable::Simple("from".to_string())]
                }),
                TemplateSqlValue::VariableChain(TemplateVariableChain {
                    variables: vec![TemplateVariable::Quote("user".to_string())]
                }),
                TemplateSqlValue::VariableChain(TemplateVariableChain {
                    variables: vec![TemplateVariable::Simple("WHERE".to_string())]
                }),
                TemplateSqlValue::Expression(TemplateExpr {
                    first_part: TemplateExprFirstPart::Variable(TemplateVariableChain {
                        variables: vec![TemplateVariable::Simple("name".to_string())]
                    }),
                    operator: "=".to_string(),
                    second_part: TemplateExprSecondPart::Percent(TemplatePlaceholder::Percent(
                        TemplateVariableChain {
                            variables: vec![TemplateVariable::Simple("name".to_string())]
                        }
                    )),
                    connective: Some(And("AND".to_string()))
                }),
                TemplateSqlValue::Expression(TemplateExpr {
                    first_part: TemplateExprFirstPart::Variable(TemplateVariableChain {
                        variables: vec![TemplateVariable::Simple("age".to_string())]
                    }),
                    operator: "=".to_string(),
                    second_part: TemplateExprSecondPart::Hash( TemplatePlaceholder::Hash(
                        TemplateVariableChain {
                        variables: vec![TemplateVariable::Simple("age".to_string())]
                    })),
                    connective: None
                })
            ]
        );
    }
}
