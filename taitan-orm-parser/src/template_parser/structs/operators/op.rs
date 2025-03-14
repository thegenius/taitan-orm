use nom::branch::alt;
use nom::combinator::map;
use nom::IResult;
use std::fmt::Display;
use crate::template_parser::structs::operators::arithmetic::ArithmeticOp;
use crate::template_parser::structs::operators::comparison_op::ComparisonOp;
use crate::template_parser::structs::operators::logic_op::LogicOp;
use crate::template_parser::structs::operators::match_op::MatchOp;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operator {
    Compare(ComparisonOp),
    Arithmetic(ArithmeticOp),
    Match(MatchOp),
    Logic(LogicOp),
}




impl Operator {
    pub fn parse(input: &str) -> IResult<&str, Operator> {
        alt((
            map(ComparisonOp::parse, Operator::Compare),
            map(ArithmeticOp::parse, Operator::Arithmetic),
            map(MatchOp::parse, Operator::Match),
            map(LogicOp::parse, Operator::Logic),
            // map(preceded(multispace0, tag_no_case(",")), |s: &str| {
            //     BinaryOp::Comma
            // }),
        ))(input)
    }
    pub fn extract_and(&self) -> Option<Operator> {
        if let Operator::Logic(logic_op) = self {
            if logic_op.to_string() == "AND" {
                return Some(Operator::Logic(LogicOp::And));
            }
        }
        None
    }
    pub fn extract_or(&self) -> Option<Operator> {
        if let Operator::Logic(logic_op) = self {
            if logic_op.to_string() == "OR" {
                return Some(Operator::Logic(LogicOp::And));
            }
        }
        None
    }
    // pub fn extract_comma(&self) -> Option<Operator> {
    //     if let Operator::Comma = self {
    //         return Some(Operator::Comma);
    //     }
    //     None
    // }
}

impl Display for Operator {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operator::Arithmetic(a) => a.fmt(fmt),
            Operator::Match(m) => m.fmt(fmt),
            Operator::Logic(l) => l.fmt(fmt),
            Operator::Compare(c) => c.fmt(fmt),
            // Operator::Comma => write!(fmt, ","),
        }
    }
}
