use std::fmt::Display;
use nom::branch::alt;
use nom::bytes::complete::tag_no_case;
use nom::character::complete::multispace0;
use nom::combinator::map;
use nom::IResult;
use nom::sequence::preceded;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LogicOp {
    And,
    Or,
    Not
}
impl LogicOp {
    pub fn parse(input: &str) -> IResult<&str, LogicOp> {
        alt((
            map(preceded(multispace0, tag_no_case("and")), |_| LogicOp::And),
            map(preceded(multispace0, tag_no_case("or")), |_| LogicOp::Or),
            map(preceded(multispace0, tag_no_case("not")), |_| LogicOp::Not),
        ))(input)
    }
}

impl Display for LogicOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogicOp::And => f.write_str("AND"),
            LogicOp::Or => f.write_str("OR"),
            LogicOp::Not => f.write_str("NOT"),
        }
    }
}
