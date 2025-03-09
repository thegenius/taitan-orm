use nom::character::complete::{char, digit1};
use nom::combinator::{map, opt, recognize};
use nom::IResult;
use nom::sequence::{preceded, tuple};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Number(pub String);

impl Number {
    pub fn parse(input: &str) -> IResult<&str, Number> {
        map(
            recognize(tuple((
                opt(char('-')),                   // 可选的负号
                opt(char('+')),                   // 可选的正号
                digit1,                           // 整数部分
                opt(preceded(char('.'), digit1)), // 可选的小数部分
            ))),
            |s: &str| Number(s.to_string()),
        )(input)
    }
}