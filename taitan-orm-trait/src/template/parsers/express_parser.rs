use crate::template::parsers::{parse_number, parse_operator, parse_placeholder, parse_variable_chain};
use crate::template::{
    TemplateExpr, TemplateExprFirstPart, TemplateExprSecondPart, TemplateSqlValue,
};

use crate::template::parsers::placeholder_parser::{
    parse_dollar_placeholder, parse_hash_placeholder, parse_percent_placeholder,
};
use nom::character::complete::multispace0;
use nom::error::ParseError;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::{alpha1, alphanumeric1, space0},
    combinator::{map, recognize},
    multi::many0,
    sequence::{delimited, pair, preceded, tuple},
    IResult,
};
use nom::combinator::opt;
use crate::template::parsers::connective::parse_connective;

fn parse_first_part(input: &str) -> IResult<&str, TemplateExprFirstPart> {
    // 解析表达式的第一个部分
    alt((
        map(parse_dollar_placeholder, TemplateExprFirstPart::Dollar),
        map(parse_variable_chain, TemplateExprFirstPart::Variable),
    ))(input)
}

fn parse_second_part(input: &str) -> IResult<&str, TemplateExprSecondPart> {
    // 解析表达式的第二个部分
    alt((
        map(parse_dollar_placeholder, TemplateExprSecondPart::Dollar),
        map(parse_hash_placeholder, TemplateExprSecondPart::Hash),
        map(parse_percent_placeholder, TemplateExprSecondPart::Percent),
        map(parse_variable_chain, TemplateExprSecondPart::Variable),
        map(parse_number, TemplateExprSecondPart::Number),
    ))(input)
}
pub fn parse_expr(input: &str) -> IResult<&str, TemplateExpr> {
    // 解析完整的表达式
    map(
        tuple((
            parse_first_part,  // 解析第一个部分
            space0,            // 允许第一个部分后有空格
            parse_operator,    // 解析操作符
            space0,            // 允许等号后有空格
            parse_second_part, // 解析第二个部分
            opt(preceded(multispace0, parse_connective)),
        )),
        |(first_part, _, operator, _, second_part, connective)| TemplateExpr {
            first_part,
            operator,
            second_part,
            connective,
        },
    )(input)
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

pub fn parse_expr_as_value(input: &str) -> IResult<&str, TemplateSqlValue> {
    let (remaining, parsed) = parse_expr(input)?;
    Ok((remaining, TemplateSqlValue::Expression(parsed)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::template::{TemplateConnective, TemplatePlaceholder, TemplateVariable};
    use crate::template::structs::TemplateExprFirstPart;
    use crate::template::TemplateExpr;
    use crate::template::TemplateVariableChain;

    #[test]
    fn test_parse_expr() {
        let (remaining, parsed) = parse_expr("a.b > = %  { sdf_d . sdf_sv_1 } AND").unwrap();
        assert_eq!(
            parsed,
            TemplateExpr {
                first_part: TemplateExprFirstPart::Variable(TemplateVariableChain {
                    variables: vec![TemplateVariable::Simple("a".to_string()), TemplateVariable::Simple("b".to_string())]
                }),
                operator: ">=".to_string(),
                second_part: TemplateExprSecondPart::Percent(TemplatePlaceholder::Percent(
                    TemplateVariableChain {
                        variables: vec![TemplateVariable::Simple("sdf_d".to_string()), TemplateVariable::Simple("sdf_sv_1".to_string())]
                    }
                )),
                connective: Some(TemplateConnective::And("AND".to_string()))
            }
        );
    }
}
