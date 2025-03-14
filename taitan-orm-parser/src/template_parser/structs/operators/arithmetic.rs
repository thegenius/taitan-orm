use std::fmt::Display;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::multispace0;
use nom::combinator::map;
use nom::IResult;
use nom::sequence::preceded;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArithmeticOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

impl ArithmeticOp {
    pub fn parse(input: &str) -> IResult<&str, ArithmeticOp> {
        alt((
            map(preceded(multispace0, tag("+")), |s: &str| ArithmeticOp::Add),
            map(preceded(multispace0, tag("-")), |s: &str| ArithmeticOp::Sub),
            map(preceded(multispace0, tag("*")), |s: &str| ArithmeticOp::Mul),
            map(preceded(multispace0, tag("/")), |s: &str| ArithmeticOp::Div),
            map(preceded(multispace0, tag("%")), |s: &str| ArithmeticOp::Mod),
        ))(input)
    }
}

impl Display for ArithmeticOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArithmeticOp::Add => f.write_str("+"),
            ArithmeticOp::Sub => f.write_str("-"),
            ArithmeticOp::Mul => f.write_str("*"),
            ArithmeticOp::Div => f.write_str("/"),
            ArithmeticOp::Mod => f.write_str("%"),
        }
    }
}