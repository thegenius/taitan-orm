use crate::template::parsers::{
    parse_number, parse_operator, parse_placeholder, parse_variable_chain,
};
use crate::template::{
    PairOptionalContext, TemplateExpr, TemplateExprFirstPart, TemplateExprSecondPart,
    TemplateSqlValue, UnitOptionalContext,
};

use crate::template::parsers::connective::parse_connective;
use crate::template::parsers::placeholder_parser::{
    parse_dollar_placeholder, parse_hash_placeholder, parse_percent_placeholder,
};
use crate::template::TemplateExpr::{Not, Parenthesized};
use nom::bytes::complete::tag_no_case;
use nom::character::complete::multispace0;
use nom::combinator::opt;
use nom::error::{Error, ParseError};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::{alpha1, alphanumeric1, space0},
    combinator::{map, recognize},
    multi::many0,
    sequence::{delimited, pair, preceded, tuple},
    IResult,
};
use rinja::filters::e;

type ParseResult<'a, O> = IResult<&'a str, O, Error<&'a str>>;
fn parse_first_part(input: &str) -> ParseResult<TemplateExprFirstPart> {
    // 解析表达式的第一个部分
    alt((
        map(parse_dollar_placeholder, TemplateExprFirstPart::Dollar),
        map(parse_variable_chain, TemplateExprFirstPart::Variable),
        map(parse_number, TemplateExprFirstPart::Number),
    ))(input)
}

fn parse_second_part(input: &str) -> ParseResult<TemplateExprSecondPart> {
    // 解析表达式的第二个部分
    alt((
        map(parse_dollar_placeholder, TemplateExprSecondPart::Dollar),
        map(parse_hash_placeholder, TemplateExprSecondPart::Hash),
        map(parse_percent_placeholder, TemplateExprSecondPart::Percent),
        map(parse_variable_chain, TemplateExprSecondPart::Variable),
        map(parse_number, TemplateExprSecondPart::Number),
    ))(input)
}
pub fn parse_simple_expr(input: &str) -> ParseResult<TemplateExpr> {
    // 解析完整的表达式
    map(
        tuple((
            parse_first_part, // 解析第一个部分
            space0,           // 允许第一个部分后有空格
            parse_operator,   // 解析操作符
            space0,           // 允许等号后有空格
            parse_second_part, // 解析第二个部分
                              // opt(preceded(multispace0, parse_connective)),
        )),
        |(first_part, _, operator, _, second_part)| match &second_part {
            TemplateExprSecondPart::Percent(percent) => TemplateExpr::Simple {
                optional_context: UnitOptionalContext::Optional {
                    variables: vec![percent.get_optional_variable().unwrap()],
                },
                first_part,
                operator,
                second_part,
            },
            _ => TemplateExpr::Simple {
                first_part,
                operator,
                second_part,
                optional_context: UnitOptionalContext::NotOptional,
            },
        },
    )(input)
}

fn parse_parenthesized_expr(input: &str) -> ParseResult<TemplateExpr> {
    // 解析带括号的表达式
    map(delimited(tag("("), parse_expr, tag(")")), |mut expr| {
        let context = expr.pop_optional_context();
        Parenthesized {
            optional_context: context,
            expr: Box::new(expr),
        }
    })(input)
}

fn parse_not_expr(input: &str) -> ParseResult<TemplateExpr> {
    // 解析 NOT 表达式
    map(
        tuple((tag_no_case("not"), space0, parse_primary_expr)),
        |(_, _, mut expr)| {
            let context = expr.pop_optional_context();
            Not {
                optional_context: context,
                expr: Box::new(expr),
            }
        },
    )(input)
}

fn parse_primary_expr(input: &str) -> ParseResult<TemplateExpr> {
    // 解析带括号的表达式或简单表达式
    alt((parse_not_expr, parse_parenthesized_expr, parse_simple_expr))(input)
}

// AND expressions have higher precedence than OR expressions.
fn parse_and_expr(input: &str) -> ParseResult<TemplateExpr> {
    let (mut remaining, mut expr) = parse_primary_expr(input)?;

    while let Ok((new_remaining, mut next_expr)) = preceded(
        tuple((space0, tag_no_case("and"), space0)),
        parse_primary_expr,
    )(remaining)
    {
        remaining = new_remaining;
        let left_ctx = expr.get_optional_context();
        let right_ctx = next_expr.get_optional_context();
        expr = if expr.is_optional() && next_expr.is_optional() {
            TemplateExpr::And {
                optional_context: PairOptionalContext::BothOptional {
                    left_variables: expr.get_optional_variables(),
                    right_variables: next_expr.get_optional_variables(),
                },
                left: Box::new(expr),
                right: Box::new(next_expr),
            }
        } else if expr.is_optional() {
            TemplateExpr::And {
                optional_context: PairOptionalContext::LeftOptional {
                  variables: expr.get_optional_variables(),
                },
                left: Box::new(expr),
                right: Box::new(next_expr),
            }
        } else if next_expr.is_optional() {
            TemplateExpr::And {
                optional_context: PairOptionalContext::RightOptional {
                    variables: next_expr.get_optional_variables(),
                },
                left: Box::new(expr),
                right: Box::new(next_expr),
            }
        } else {
            TemplateExpr::And {
                left: Box::new(expr),
                right: Box::new(next_expr),
                optional_context: PairOptionalContext::NotOptional,
            }
        }
    }
    Ok((remaining, expr))
}

// OR expressions have lower precedence than AND expressions.
fn parse_or_expr(input: &str) -> ParseResult<TemplateExpr> {
    let (mut remaining, mut expr) = parse_and_expr(input)?;

    while let Ok((new_remaining, next_expr)) =
        preceded(tuple((space0, tag_no_case("or"), space0)), parse_and_expr)(remaining)
    {
        remaining = new_remaining;
        let left_ctx = expr.get_optional_context();
        let right_ctx = next_expr.get_optional_context();
        expr = if expr.is_optional() && next_expr.is_optional() {
            TemplateExpr::Or {
                optional_context: PairOptionalContext::BothOptional {
                    left_variables: expr.get_optional_variables(),
                    right_variables: next_expr.get_optional_variables(),
                },
                left: Box::new(expr),
                right: Box::new(next_expr),
            }
        } else if expr.is_optional() {
            TemplateExpr::Or {
                optional_context: PairOptionalContext::LeftOptional {
                    variables: expr.get_optional_variables(),
                },
                left: Box::new(expr),
                right: Box::new(next_expr),
            }
        } else if next_expr.is_optional() {
            TemplateExpr::Or {
                optional_context: PairOptionalContext::RightOptional {
                    variables: next_expr.get_optional_variables(),
                },
                left: Box::new(expr),
                right: Box::new(next_expr),
            }
        } else {
            TemplateExpr::Or {
                left: Box::new(expr),
                right: Box::new(next_expr),
                optional_context: PairOptionalContext::NotOptional,
            }
        }
    }

    Ok((remaining, expr))
}

pub fn parse_expr(input: &str) -> ParseResult<TemplateExpr> {
    // 解析完整的表达式
    alt((
        parse_or_expr,  // OR 表达式的优先级最低，所以放在前面
        parse_and_expr, // AND 表达式的优先级较高
        parse_not_expr, // NOT 表达式的优先级最高
        parse_primary_expr,
    ))(input)
}

// pub fn parse_expr(input: &str) -> IResult<&str, TemplateExpr> {
//     // 解析完整的赋值语句 a = %{name}
//     map(
//         tuple((
//             parse_variable_chain,                           // 解析变量名
//             space0,                                         // 允许变量名后有空格
//             parse_operator,                                 // 解析操作符
//             space0,                                         // 允许等号后有空格
//             alt((
//                 map(parse_placeholder, ::Placeholder), // 尝试解析模板占位符
//                 map(parse_variable_chain, TemplateExprPart::VariableChain), // 尝试解析变量链条
//             )), // 解析模板占位符
//         )),
//         |(variable_chain, _, operator, _, placeholder)| TemplateExpr {
//             variable_chain,
//             operator,
//             placeholder,
//         },
//     )(input)
// }

pub fn parse_expr_as_value(input: &str) -> ParseResult<TemplateSqlValue> {
    let (remaining, parsed) = parse_expr(input)?;
    Ok((remaining, TemplateSqlValue::Expression(parsed)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::template::structs::TemplateExprFirstPart;
    use crate::template::TemplateExpr;
    use crate::template::TemplateVariableChain;
    use crate::template::{TemplateConnective, TemplatePlaceholder, TemplateVariable};

    #[test]
    fn test_parse_expr() {
        let (remaining, parsed) =
            parse_expr("a.b > = %  { sdf_d . sdf_sv_1 } OR 100 = 100 AND c <> d").unwrap();
        let first_expr = TemplateExpr::Simple {
            first_part: TemplateExprFirstPart::Variable(TemplateVariableChain {
                variables: vec![
                    TemplateVariable::Simple("a".to_string()),
                    TemplateVariable::Simple("b".to_string()),
                ],
            }),
            operator: ">=".to_string(),
            second_part: TemplateExprSecondPart::Percent(TemplatePlaceholder::Percent(
                TemplateVariableChain {
                    variables: vec![
                        TemplateVariable::Simple("sdf_d".to_string()),
                        TemplateVariable::Simple("sdf_sv_1".to_string()),
                    ],
                },
            )),
            optional_context: UnitOptionalContext::Optional {
                variables: vec!["sdf_d.sdf_sv_1".to_string()],
            },
        };

        let second_expr = TemplateExpr::Simple {
            first_part: TemplateExprFirstPart::Number("100".to_string()),
            operator: "=".to_string(),
            second_part: TemplateExprSecondPart::Number("100".to_string()),
            optional_context: UnitOptionalContext::NotOptional,
        };

        let third_expr = TemplateExpr::Simple {
            first_part: TemplateExprFirstPart::Variable(TemplateVariableChain {
                variables: vec![TemplateVariable::Simple("c".to_string())],
            }),
            operator: "<>".to_string(),
            second_part: TemplateExprSecondPart::Variable(TemplateVariableChain {
                variables: vec![TemplateVariable::Simple("d".to_string())],
            }),
            optional_context: UnitOptionalContext::NotOptional,
        };

        assert_eq!(
            parsed,
            TemplateExpr::Or {
                left: Box::new(first_expr),
                right: Box::new(TemplateExpr::And {
                    left: Box::new(second_expr),
                    right: Box::new(third_expr),
                    optional_context: PairOptionalContext::NotOptional,
                }),
                optional_context: PairOptionalContext::LeftOptional {
                    variables: vec!["sdf_d.sdf_sv_1".to_string()],
                },
            }
        );
    }
}
