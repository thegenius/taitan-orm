use std::fmt::Display;
use nom::branch::alt;
use nom::bytes::complete::{tag, tag_no_case};
use nom::character::complete::multispace0;
use nom::combinator::map;
use nom::IResult;
use nom::sequence::{preceded, tuple};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinaryOp {
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Equal,
    NotEqual,
    Like,
    In,
}

impl BinaryOp {
    pub fn parse(input: &str) -> IResult<&str, BinaryOp> {
        let (remaining, parsed) = alt((
            // 多字符操作符优先匹配，允许中间有空格
            map(
                tuple((
                    preceded(multispace0, tag(">")),
                    multispace0,
                    preceded(multispace0, tag("=")),
                )),
                |_| BinaryOp::GreaterEqual,
            ),
            map(
                tuple((
                    preceded(multispace0, tag("<")),
                    multispace0,
                    preceded(multispace0, tag("=")),
                )),
                |_| BinaryOp::LessEqual,
            ),
            map(
                tuple((
                    preceded(multispace0, tag("<")),
                    multispace0,
                    preceded(multispace0, tag(">")),
                )),
                |_| BinaryOp::NotEqual,
            ),
            map(
                tuple((
                    preceded(multispace0, tag("!")),
                    multispace0,
                    preceded(multispace0, tag("=")),
                )),
                |_| BinaryOp::NotEqual,
            ),
            // 单字符操作符
            map(preceded(multispace0, tag("=")), |s: &str| BinaryOp::Equal),
            map(preceded(multispace0, tag("<")), |s: &str| BinaryOp::Less),
            map(preceded(multispace0, tag(">")), |s: &str| BinaryOp::Greater),
            map(preceded(multispace0, tag_no_case("like")), |_| {
                BinaryOp::Like
            }),
            map(preceded(multispace0, tag_no_case("in")), |_| {
                BinaryOp::In
            }),
        ))(input)?;

        Ok((remaining, parsed))
    }
}

impl Display for BinaryOp {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BinaryOp::Less => write!(fmt, "<"),
            BinaryOp::LessEqual => write!(fmt, "<="),
            BinaryOp::Greater => write!(fmt, ">"),
            BinaryOp::GreaterEqual => write!(fmt, ">="),
            BinaryOp::Equal => write!(fmt, "="),
            BinaryOp::NotEqual => write!(fmt, "<>"),
            BinaryOp::Like => write!(fmt, "LIKE"),
            BinaryOp::In => write!(fmt, "IN"),
        }
    }
}