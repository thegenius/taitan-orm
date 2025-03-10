use crate::template_parser::structs::atomic::Atomic;
use crate::template_parser::structs::binary_op::BinaryOp;
use nom::character::complete::multispace0;
use nom::sequence::preceded;
use nom::IResult;

#[derive(Debug, PartialEq, Clone)]
pub struct SimpleExpr {
    left: Atomic,
    op: BinaryOp,
    right: Atomic,
}

impl SimpleExpr {
    pub fn parse(input: &str) -> IResult<&str, SimpleExpr> {
        let (input, left) = preceded(multispace0, Atomic::parse)(input)?; // 解析左操作数
        let (input, op) = preceded(multispace0, BinaryOp::parse)(input)?; // 解析操作符
        let (input, right) = preceded(multispace0, Atomic::parse)(input)?; // 解析右操作数
        Ok((input, SimpleExpr { left, op, right }))
    }
}
