use std::fmt::Display;
use nom::branch::alt;
use nom::bytes::complete::{tag, tag_no_case};
use nom::character::complete::multispace0;
use nom::combinator::map;
use nom::IResult;
use nom::sequence::{preceded, tuple};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ComparisonOp {
    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    Like
}

impl ComparisonOp {
    pub fn parse(input: &str) -> IResult<&str, ComparisonOp> {
        let (remaining, parsed) = alt((
            // 多字符操作符优先匹配，允许中间有空格
            map(
                tuple((
                    preceded(multispace0, tag(">")),
                    multispace0,
                    preceded(multispace0, tag("=")),
                )),
                |_| ComparisonOp::GreaterThanOrEqual,
            ),
            map(
                tuple((
                    preceded(multispace0, tag("<")),
                    multispace0,
                    preceded(multispace0, tag("=")),
                )),
                |_| ComparisonOp::LessThanOrEqual,
            ),
            map(
                tuple((
                    preceded(multispace0, tag("<")),
                    multispace0,
                    preceded(multispace0, tag(">")),
                )),
                |_| ComparisonOp::NotEqual,
            ),
            map(
                tuple((
                    preceded(multispace0, tag("!")),
                    multispace0,
                    preceded(multispace0, tag("=")),
                )),
                |_| ComparisonOp::NotEqual,
            ),
            // 单字符操作符
            map(preceded(multispace0, tag("=")), |s: &str| {
                ComparisonOp::Equal
            }),
            map(preceded(multispace0, tag("<")), |s: &str| {
                ComparisonOp::LessThan
            }),
            map(preceded(multispace0, tag(">")), |s: &str| {
                ComparisonOp::GreaterThan
            }),
            map(preceded(multispace0, tag_no_case("like")), |_| {
                ComparisonOp::Like
            }),
        ))(input)?;

        Ok((remaining, parsed))
    }
}

impl Display for ComparisonOp {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ComparisonOp::LessThan => write!(fmt, "<"),
            ComparisonOp::LessThanOrEqual => write!(fmt, "<="),
            ComparisonOp::GreaterThan => write!(fmt, ">"),
            ComparisonOp::GreaterThanOrEqual => write!(fmt, ">="),
            ComparisonOp::Equal => write!(fmt, "="),
            ComparisonOp::NotEqual => write!(fmt, "<>"),
            ComparisonOp::Like => write!(fmt, "LIKE"),
        }
    }
}