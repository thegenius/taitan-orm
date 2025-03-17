use nom::branch::alt;
use nom::bytes::complete::tag_no_case;
use nom::combinator::map;
use nom::IResult;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Bool {
    True,
    False,
}
impl Bool {
    pub fn parse(input: &str) -> IResult<&str, Bool> {
        alt((
            map(tag_no_case("TRUE"), |_| Bool::True),
            map(tag_no_case("FALSE"), |_| Bool::False),
        ))(input)
    }
}

impl Display for Bool {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Bool::True => write!(fmt, "TRUE"),
            Bool::False => write!(fmt, "FALSE"),
        }
    }
}
