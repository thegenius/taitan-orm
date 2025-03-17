use std::collections::VecDeque;
use crate::template_parser::structs::exprs::text_expr::TextExpr;
use crate::template_parser::structs::values::{NumberValue, TextValue};
use crate::template_parser::ArithmeticOp;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::multispace0;
use nom::combinator::map;
use nom::sequence::{delimited, preceded};
use nom::IResult;
use tracing::debug;

#[derive(Debug, Clone, PartialEq)]
pub enum ArithmeticExpr {
    Value(NumberValue),
    Nested(Box<ArithmeticExpr>),
    Expr {
        left: Box<ArithmeticExpr>,
        op: ArithmeticOp,
        right: Box<ArithmeticExpr>,
    },
}
