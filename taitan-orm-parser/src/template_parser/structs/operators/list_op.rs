use nom::branch::alt;
use nom::bytes::complete::tag_no_case;
use nom::character::complete::multispace0;
use nom::combinator::map;
use nom::sequence::preceded;
use nom::IResult;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ListInOp {
    In,
    NotIn,
}

impl ListInOp {
    pub fn parse(input: &str) -> IResult<&str, ListInOp> {
        alt((
            map(preceded(multispace0, tag_no_case("in")), |_| ListInOp::In),
            map(
                preceded(
                    multispace0,
                    preceded(tag_no_case("not"), preceded(multispace0, tag_no_case("in"))),
                ),
                |_| ListInOp::In,
            ),
        ))(input)
    }
}

impl Display for ListInOp {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ListInOp::In => write!(fmt, "IN"),
            ListInOp::NotIn => write!(fmt, "NOT IN"),
        }
    }
}
