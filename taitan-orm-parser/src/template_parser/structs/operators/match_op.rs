use std::fmt::Display;
use nom::branch::alt;
use nom::bytes::complete::tag_no_case;
use nom::character::complete::multispace0;
use nom::combinator::map;
use nom::IResult;
use nom::sequence::preceded;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MatchOp {
    Like
}

impl MatchOp {
    pub fn parse(input: &str) -> IResult<&str, MatchOp> {
        alt((
            map(preceded(multispace0, tag_no_case("like")), |_| {
                MatchOp::Like
            }),
        ))(input)
    }
}

impl Display for MatchOp {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MatchOp::Like => write!(fmt, "LIKE"),
        }
    }
}