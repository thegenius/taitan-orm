use crate::template_parser::number::Number;
use nom::branch::alt;
use nom::bytes::complete::{tag, tag_no_case};
use nom::character::complete::multispace0;
use nom::combinator::map;
use nom::sequence::{preceded, tuple};
use nom::IResult;

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
            // 确保单个符号也可以被解析
            map(preceded(multispace0, tag(">=")), |s: &str| {
                BinaryOp::GreaterEqual
            }),
            map(preceded(multispace0, tag("<=")), |s: &str| {
                BinaryOp::LessEqual
            }),
            map(preceded(multispace0, tag("!=")), |s: &str| {
                BinaryOp::NotEqual
            }),
            map(preceded(multispace0, tag("<>")), |s: &str| {
                BinaryOp::NotEqual
            }),
            map(preceded(multispace0, tag_no_case("like")), |_| {
                BinaryOp::Like
            }),
        ))(input)?;

        Ok((remaining, parsed))
    }
}
