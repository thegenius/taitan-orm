
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::{alpha1, alphanumeric1, char, digit1, multispace0, space0},
    combinator::{map, opt, recognize, value},
    multi::many0,
    sequence::{delimited, preceded, tuple},
    IResult,
};
use crate::template_parser::TemplateSqlValue;

pub fn parse_number(input: &str) -> IResult<&str, String> {
    map(
        recognize(tuple((
            opt(char('-')),                   // 可选的负号
            opt(char('+')),                   // 可选的正号
            digit1,                           // 整数部分
            opt(preceded(char('.'), digit1)), // 可选的小数部分
        ))),
        |s: &str| s.to_string(),
    )(input)
}

pub fn parse_number_as_value(input: &str) -> IResult<&str, TemplateSqlValue> {
    let (remaining, parsed) = parse_number(input)?;
    Ok((remaining, TemplateSqlValue::Number(parsed)))
}
