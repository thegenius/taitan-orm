use crate::template_parser::structs::operators::arithmetic::ArithmeticOp;
use crate::template_parser::structs::operators::comparison_op::CompareOp;
use crate::template_parser::structs::operators::connect::ConnectOp;
use crate::template_parser::structs::operators::list_op::ListInOp;
use crate::template_parser::structs::operators::logic_op::LogicOp;
use crate::template_parser::structs::operators::paren::Paren;
use nom::branch::alt;
use nom::combinator::map;
use nom::IResult;
use std::fmt::Display;
use crate::VariableChain;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operator {
    Compare(CompareOp),
    Arithmetic(ArithmeticOp),
    Logic(LogicOp),
    ListInOp(ListInOp),
    Paren(Paren),
    Connect(ConnectOp),
    FnCall(VariableChain)
}

impl Operator {
    pub fn parse(input: &str) -> IResult<&str, Operator> {
        alt((
            map(CompareOp::parse, Operator::Compare),
            map(ArithmeticOp::parse, Operator::Arithmetic),
            map(LogicOp::parse, Operator::Logic),
            map(ListInOp::parse, Operator::ListInOp),
            map(Paren::parse, Operator::Paren),
            map(ConnectOp::parse, Operator::Connect),
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
            Operator::ListInOp(l) => l.fmt(fmt),
            Operator::Paren(p) => p.fmt(fmt),
            Operator::Connect(c) => c.fmt(fmt),
            Operator::FnCall(f) => f.fmt(fmt),
        }
    }
}
