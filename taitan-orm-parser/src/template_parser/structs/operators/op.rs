use crate::template_parser::structs::operators::arithmetic::ArithmeticOp;
use crate::template_parser::structs::operators::comparison_op::ComparisonOp;
use crate::template_parser::structs::operators::list_op::ListOp;
use crate::template_parser::structs::operators::logic_op::LogicOp;
use nom::branch::alt;
use nom::combinator::map;
use nom::IResult;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operator {
    Compare(ComparisonOp),
    Arithmetic(ArithmeticOp),
    Logic(LogicOp),
    ListOp(ListOp),
}

impl Operator {
    pub fn parse(input: &str) -> IResult<&str, Operator> {
        alt((
            map(ComparisonOp::parse, Operator::Compare),
            map(ArithmeticOp::parse, Operator::Arithmetic),
            map(LogicOp::parse, Operator::Logic),
            map(ListOp::parse, Operator::ListOp),
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
}

impl Display for Operator {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operator::Arithmetic(a) => a.fmt(fmt),
            Operator::Logic(l) => l.fmt(fmt),
            Operator::Compare(c) => c.fmt(fmt),
            Operator::ListOp(l) => l.fmt(fmt),
        }
    }
}
