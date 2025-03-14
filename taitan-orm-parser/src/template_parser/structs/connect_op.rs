use std::fmt::Display;
use nom::branch::alt;
use nom::bytes::complete::tag_no_case;
use nom::character::complete::multispace0;
use nom::combinator::map;
use nom::IResult;
use nom::sequence::preceded;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConnectOp {
    // Logic(LogicOp),
    Comma
}
impl ConnectOp {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            // map(LogicOp::parse, ConnectOp::Logic),
            map(preceded(multispace0, tag_no_case(",")), |s: &str| {
                ConnectOp::Comma
            }),
        ))(input)
    }
}

impl Display for ConnectOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConnectOp::Comma => f.write_str(","),
            // ConnectOp::Logic(logic) => logic.fmt(f),
        }
    }
}

