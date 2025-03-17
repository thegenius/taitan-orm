use nom::branch::alt;
use nom::bytes::complete::{tag, tag_no_case};
use nom::character::complete::multispace0;
use nom::combinator::map;
use nom::sequence::preceded;
use nom::IResult;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConnectOp {
    Comma
}

impl ConnectOp {
    pub fn parse(input: &str) -> IResult<&str, ConnectOp> {
        alt((
            map(preceded(multispace0, tag(",")), |_| ConnectOp::Comma),
        ))(input)
    }
}

impl Display for ConnectOp {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConnectOp::Comma => write!(fmt, ","),
        }
    }
}
