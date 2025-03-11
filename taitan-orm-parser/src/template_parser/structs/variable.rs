use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, alphanumeric1, multispace0};
use nom::combinator::{map, recognize};
use nom::error::context;
use nom::multi::many0;
use nom::sequence::{delimited, pair, preceded, tuple};
use nom::IResult;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Variable {
    Simple(String),
    Backquote(String),
    Brackets(String),
    DoubleQuote(String),
}

impl Variable {
    pub fn parse(input: &str) -> IResult<&str, Variable> {
        alt((
            parse_quoted_variable,                        // 尝试解析带引号的变量名
            map(parse_simple_variable, Variable::Simple), // 如果失败，则尝试解析不带引号的变量名
        ))(input)
    }
    pub fn get_ident(&self) -> &str {
        match self {
            Variable::Simple(ident)
            | Variable::Backquote(ident)
            | Variable::Brackets(ident)
            | Variable::DoubleQuote(ident) => ident,
        }
    }
}

impl Display for Variable {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Variable::Simple(ident) => write!(f, "{}", ident),
            Variable::Backquote(ident) => write!(f, "`{}`", ident),
            Variable::Brackets(ident) => write!(f, "[{}]", ident),
            Variable::DoubleQuote(ident) => write!(f, "\"{}\"", ident),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VariableChain {
    pub variables: Vec<Variable>,
}
impl VariableChain {
    pub fn new(variables: Vec<Variable>) -> VariableChain {
        VariableChain { variables }
    }
    pub fn parse(input: &str) -> IResult<&str, VariableChain> {
        // 解析完整的变量名链条 a.b.c
        context(
            "VariableChain",
            map(
                tuple((
                    Variable::parse, // 解析第一个变量名部分
                    many0(preceded(
                        multispace0,
                        preceded(tag("."), preceded(multispace0, Variable::parse)),
                    )), // 解析后续的部分，每个部分前有 '.'
                )),
                |(first, rest)| VariableChain {
                    variables: std::iter::once(first).chain(rest.into_iter()).collect(),
                },
            ),
        )(input)
    }
}

impl Display for VariableChain {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();
        for (index, variable) in self.variables.iter().enumerate() {
            if index > 0 {
                result.push('.');
            }
            result.push_str(&variable.to_string());
        }
        write!(f, "{}", result)
    }
}

fn parse_simple_variable(input: &str) -> IResult<&str, String> {
    // 解析变量名，允许字母数字字符和下划线
    map(
        recognize(pair(
            alpha1,                                // 变量名以字母开头
            many0(alt((alphanumeric1, tag("_")))), // 后面可以跟字母、数字或下划线
        )),
        |s: &str| s.to_string(),
    )(input)
}

fn parse_quoted_variable(input: &str) -> IResult<&str, Variable> {
    // 解析带引号的变量名，例如 `var`
    alt((
        map(
            delimited(
                tag("`"),
                preceded(multispace0, parse_simple_variable),
                preceded(multispace0, tag("`")),
            ),
            |var| Variable::Backquote(var.to_string()),
        ),
        map(
            delimited(
                tag("["),
                preceded(multispace0, parse_simple_variable),
                preceded(multispace0, tag("]")),
            ),
            |var| Variable::Brackets(var.to_string()),
        ),
        map(
            delimited(
                tag("\""),
                preceded(multispace0, parse_simple_variable),
                preceded(multispace0, tag("\"")),
            ),
            |var| Variable::DoubleQuote(var.to_string()),
        ),
    ))(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_variable_chain_spec_001() {
        let template = "sdfs.[ sdf ].` uire123`.\"gsdg \".  dfdsl";
        let (_, variable_chain) = VariableChain::parse(template).unwrap();
        assert_eq!(
            variable_chain.variables,
            vec![
                Variable::Simple("sdfs".to_string()),
                Variable::Brackets("sdf".to_string()),
                Variable::Backquote("uire123".to_string()),
                Variable::DoubleQuote("gsdg".to_string()),
                Variable::Simple("dfdsl".to_string()),
            ]
        );
    }

    #[test]
    fn test_parse_variable_chain_spec_002() {
        let template = "123";
        let parse_result = VariableChain::parse(template);
        assert!(parse_result.is_err());

        let template = "'asdfv'";
        let parse_result = VariableChain::parse(template);
        assert!(parse_result.is_err());
    }
}
