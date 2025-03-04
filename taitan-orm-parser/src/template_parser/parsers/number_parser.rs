
use nom::{
    character::complete::{char, digit1},
    combinator::{map, opt, recognize},
    sequence::{preceded, tuple},
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
