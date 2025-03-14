use nom::branch::alt;
use nom::bytes::complete::{tag, take_while1};
use nom::character::complete::{char, digit1, multispace0};
use nom::combinator::{map, opt, recognize};
use nom::sequence::{preceded, terminated, tuple};
use nom::IResult;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Number(pub String);

impl Number {
    pub fn parse(input: &str) -> IResult<&str, Number> {
        alt((parse_hex, parse_decimal))(input)
    }
}

fn parse_decimal(input: &str) -> IResult<&str, Number> {
    map(
        recognize(tuple((
            // opt(char('-')),                   // 可选的负号
            // opt(char('+')),                   // 可选的正号
            digit1, // 整数部分
            opt(preceded(
                multispace0,
                preceded(char('.'), preceded(multispace0, digit1)),
            )), // 可选的小数部分
        ))),
        |s: &str| Number(s.to_string()),
    )(input)
}

fn parse_hex(input: &str) -> IResult<&str, Number> {
    map(
        recognize(alt((
            // 解析 0x1A 格式
            preceded(tag("0x"), take_while1(|c: char| c.is_ascii_hexdigit())),
            // 解析 x'1A' 格式
            preceded(
                tuple((tag("x"), char('\''))),
                terminated(take_while1(|c: char| c.is_ascii_hexdigit()), char('\'')),
            ),
        ))),
        |s: &str| Number(s.to_string()),
    )(input)
}

impl Display for Number {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "{}", self.0)
    }
}

#[cfg(test)]
mod num_parse_tests {
    use crate::Number;

    #[test]
    fn test_number_parse() {
        let template = "123 . 123";
        let (_, parsed) = Number::parse(template).unwrap();
        assert_eq!(parsed, Number("123 . 123".to_string()));
    }
    #[test]
    fn test_hex_number_parse() {
        let template = "0x123";
        let (_, parsed) = Number::parse(template).unwrap();
        assert_eq!(parsed, Number("0x123".to_string()));

        let template = "x\'123\'";
        let (_, parsed) = Number::parse(template).unwrap();
        assert_eq!(parsed, Number("x\'123\'".to_string()));
    }
}
