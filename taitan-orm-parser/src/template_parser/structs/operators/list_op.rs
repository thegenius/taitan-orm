use std::fmt::Display;
use nom::branch::alt;
use nom::bytes::complete::tag_no_case;
use nom::character::complete::multispace0;
use nom::combinator::map;
use nom::IResult;
use nom::sequence::preceded;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ListOp {
    In,
}

impl ListOp {
    pub fn parse(input: &str) -> IResult<&str, ListOp> {
        alt((
            map(preceded(multispace0, tag_no_case("in")), |_| ListOp::In),
        ))(input)
    }
}

impl Display for ListOp {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ListOp::In => write!(fmt, "IN"),
        }
    }
}