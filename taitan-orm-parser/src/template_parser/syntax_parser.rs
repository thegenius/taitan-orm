use crate::{Atomic, VariableChain};
use crate::template_parser::structs::binary_op::BinaryOp;

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Simple {
        left: Atomic,
        op: BinaryOp,
        right: Atomic,
    },
    BinaryExpr {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
    },
    FunctionCall {
        name: VariableChain,
        args: Vec<Expr>,
    },
    Nested(Box<Expr>),  // ()嵌套表达式
    Not(Box<Expr>),     // NOT 表达式
}

