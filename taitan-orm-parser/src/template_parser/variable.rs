use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, alphanumeric1, multispace0};
use nom::combinator::{map, recognize};
use nom::multi::many0;
use nom::sequence::{delimited, pair, preceded, tuple};
use nom::IResult;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Variable {
    Simple(String),
    Quote(String),
}

impl Variable {
    pub fn parse(input: &str) -> IResult<&str, Variable> {
        alt((
            parse_quoted_variable, // 尝试解析带引号的变量名
            parse_simple_variable, // 如果失败，则尝试解析不带引号的变量名
        ))(input)
    }
    pub fn get_ident(&self) -> &str {
        match self {
            Variable::Simple(ident) => ident,
            Variable::Quote(ident) => ident,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VariableChain {
    pub variables: Vec<Variable>,
}
impl VariableChain {
    pub fn parse(input: &str) -> IResult<&str, VariableChain> {
        // 解析完整的变量名链条 a.b.c
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
        )(input)
    }
}



fn parse_simple_variable(input: &str) -> IResult<&str, Variable> {
    // 解析变量名，允许字母数字字符和下划线
    map(
        recognize(pair(
            alpha1,                                // 变量名以字母开头
            many0(alt((alphanumeric1, tag("_")))), // 后面可以跟字母、数字或下划线
        )),
        |s: &str| Variable::Simple(s.to_string()),
    )(input)
}

fn parse_quoted_variable(input: &str) -> IResult<&str, Variable> {
    // 解析带引号的变量名，例如 `var`
    let (remaining, parsed) = delimited(tag("`"), parse_simple_variable, tag("`"))(input)?;
    Ok((remaining, Variable::Quote(parsed.get_ident().to_string())))
}
