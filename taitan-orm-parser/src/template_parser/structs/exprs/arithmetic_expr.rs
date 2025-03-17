use crate::template_parser::ArithmeticOp;
use crate::template_parser::structs::values::NumberValue;

#[derive(Debug, Clone, PartialEq)]
pub enum ArithmeticExpr {
    Value(NumberValue),
    Nested(Box<ArithmeticExpr>),
    Expr{
        left: Box<ArithmeticExpr>,
        op: ArithmeticOp,
        right: Box<ArithmeticExpr>,
    }
}