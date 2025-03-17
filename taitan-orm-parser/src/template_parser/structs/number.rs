use nom::branch::alt;
use nom::bytes::complete::{tag, take_while1};
use nom::character::complete::{digit1, multispace0};
use nom::combinator::{map, opt, recognize};
use nom::sequence::{preceded, terminated, tuple};
use nom::IResult;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Number {
    pub value: String,
    pub unary_op: Option<char>,
}

impl Number {
    pub fn parse(input: &str) -> IResult<&str, Number> {
        alt((parse_hex, parse_decimal))(input)
    }

    pub fn set_unary_op(mut self, unary_op: char) {
        assert!(unary_op == '-' || unary_op == '+');
        self.unary_op = Some(unary_op)
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
                preceded(
                    nom::character::complete::char('.'),
                    preceded(multispace0, digit1),
                ),
            )), // 可选的小数部分
        ))),
        |s: &str| Number {
            value: s.to_string(),
            unary_op: None,
        },
    )(input)
}

fn parse_hex(input: &str) -> IResult<&str, Number> {
    map(
        recognize(alt((
            // 解析 0x1A 格式
            preceded(tag("0x"), take_while1(|c: char| c.is_ascii_hexdigit())),
            // 解析 x'1A' 格式
            preceded(
                tuple((tag("x"), nom::character::complete::char('\''))),
                terminated(
                    take_while1(|c: char| c.is_ascii_hexdigit()),
                    nom::character::complete::char('\''),
                ),
            ),
        ))),
        |s: &str| Number {
            value: s.to_string(),
            unary_op: None,
        },
    )(input)
}

impl Display for Number {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.unary_op {
            Some(unary_op) => fmt.write_fmt(format_args!("{}{}", unary_op, self.value)),
            None => fmt.write_fmt(format_args!("{}", self.value)),
        }
    }
}

#[cfg(test)]
mod num_parse_tests {
    use crate::Number;

    #[test]
    fn test_number_parse() {
        let template = "123 . 123";
        let (_, parsed) = Number::parse(template).unwrap();
        assert_eq!(
            parsed,
            Number {
                value: "123 . 123".to_string(),
                unary_op: None
            }
        );
    }
    #[test]
    fn test_hex_number_parse() {
        let template = "0x123";
        let (_, parsed) = Number::parse(template).unwrap();
        assert_eq!(
            parsed,
            Number {
                value: "0x123".to_string(),
                unary_op: None
            }
        );

        let template = "x\'123\'";
        let (_, parsed) = Number::parse(template).unwrap();
        assert_eq!(
            parsed,
            Number {
                value: "x\'123\'".to_string(),
                unary_op: None
            }
        );
    }
}
