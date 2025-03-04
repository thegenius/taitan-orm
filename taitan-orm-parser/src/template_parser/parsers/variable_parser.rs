use nom::character::complete::multispace0;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::{alpha1, alphanumeric1, space0},
    combinator::{map, recognize},
    multi::many0,
    sequence::{delimited, pair, preceded, tuple},
    IResult,
};

use crate::template_parser::{TemplateSqlValue, TemplateVariable, TemplateVariableChain};

fn parse_variable(input: &str) -> IResult<&str, TemplateVariable> {
    alt((
        parse_quoted_variable, // 尝试解析带引号的变量名
        parse_simple_variable,        // 如果失败，则尝试解析不带引号的变量名
    ))(input)
}

fn parse_simple_variable_as_str(input: &str) -> IResult<&str, String> {
    // 解析变量名，允许字母数字字符和下划线
    map(
        recognize(pair(
            alpha1,                                // 变量名以字母开头
            many0(alt((alphanumeric1, tag("_")))), // 后面可以跟字母、数字或下划线
        )),
        |s: &str| s.to_string(),
    )(input)
}
fn parse_simple_variable(input: &str) -> IResult<&str, TemplateVariable> {
    let (remaining, parsed) = parse_simple_variable_as_str(input)?;
    Ok((remaining, TemplateVariable::Simple(parsed)))
}

fn parse_quoted_variable(input: &str) -> IResult<&str, TemplateVariable> {
    // 解析带引号的变量名，例如 `var`
    let (remaining, parsed) =  delimited(
        tag("`"),
        parse_simple_variable_as_str,
        tag("`"),
    )(input)?;
    Ok((remaining, TemplateVariable::Quote(parsed)))
}

pub fn parse_variable_chain(input: &str) -> IResult<&str, TemplateVariableChain> {
    // 解析完整的变量名链条 a.b.c
    map(
        tuple((
            parse_variable, // 解析第一个变量名部分
            many0(preceded(
                multispace0,
                preceded(tag("."), preceded(multispace0, parse_variable)),
            )), // 解析后续的部分，每个部分前有 '.'
        )),
        |(first, rest)| TemplateVariableChain {
            variables: std::iter::once(first).chain(rest.into_iter()).collect(),
        },
    )(input)
}

pub fn parse_variable_chain_as_value(input: &str) -> IResult<&str, TemplateSqlValue> {
    let (remaining, parsed) = parse_variable_chain(input)?;
    Ok((remaining, TemplateSqlValue::VariableChain(parsed)))
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_parse_variable() {
        let (remaining, parsed) = parse_variable_chain("sdf_d . sdf_sv_1 ").unwrap();
        assert_eq!(
            parsed,
            TemplateVariableChain {
                variables: vec![TemplateVariable::Simple("sdf_d".to_string()), TemplateVariable::Simple("sdf_sv_1".to_string())]
            }
        );
        assert_eq!(remaining, " ");
    }
}
