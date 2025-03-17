use std::fmt::Display;
use nom::branch::alt;
use nom::bytes::complete::{tag, tag_no_case};
use nom::character::complete::multispace0;
use nom::combinator::map;
use nom::IResult;
use nom::sequence::{preceded, tuple};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompareOp {
    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    Like
}

impl CompareOp {
    pub fn parse(input: &str) -> IResult<&str, CompareOp> {
        let (remaining, parsed) = alt((
            // 多字符操作符优先匹配，允许中间有空格
            map(
                tuple((
                    preceded(multispace0, tag(">")),
                    multispace0,
                    preceded(multispace0, tag("=")),
                )),
                |_| CompareOp::GreaterThanOrEqual,
            ),
            map(
                tuple((
                    preceded(multispace0, tag("<")),
                    multispace0,
                    preceded(multispace0, tag("=")),
                )),
                |_| CompareOp::LessThanOrEqual,
            ),
            map(
                tuple((
                    preceded(multispace0, tag("<")),
                    multispace0,
                    preceded(multispace0, tag(">")),
                )),
                |_| CompareOp::NotEqual,
            ),
            map(
                tuple((
                    preceded(multispace0, tag("!")),
                    multispace0,
                    preceded(multispace0, tag("=")),
                )),
                |_| CompareOp::NotEqual,
            ),
            // 单字符操作符
            map(preceded(multispace0, tag("=")), |s: &str| {
                CompareOp::Equal
            }),
            map(preceded(multispace0, tag("<")), |s: &str| {
                CompareOp::LessThan
            }),
            map(preceded(multispace0, tag(">")), |s: &str| {
                CompareOp::GreaterThan
            }),
            map(preceded(multispace0, tag_no_case("like")), |_| {
                CompareOp::Like
            }),
        ))(input)?;

        Ok((remaining, parsed))
    }
}

impl Display for CompareOp {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompareOp::LessThan => write!(fmt, "<"),
            CompareOp::LessThanOrEqual => write!(fmt, "<="),
            CompareOp::GreaterThan => write!(fmt, ">"),
            CompareOp::GreaterThanOrEqual => write!(fmt, ">="),
            CompareOp::Equal => write!(fmt, "="),
            CompareOp::NotEqual => write!(fmt, "<>"),
            CompareOp::Like => write!(fmt, "LIKE"),
        }
    }
}