
use crate::template_parser::structs::operators::{ListInOp, LogicOp};
use crate::template_parser::structs::values::{BoolValue, TextValue};
use crate::Operator;
use crate::template_parser::MatchOp;
use crate::template_parser::structs::exprs::text_expr::TextExpr;

#[derive(Debug, Clone, PartialEq)]
pub enum LogicExpr {
    Value(BoolValue),

    // 'aaa' like 'bbb'
    TextMatchExpr {
        left: Box<TextExpr>,
        op: MatchOp,
        right: Box<TextExpr>,
    },

    Nested(Box<LogicExpr>),
    Expr {
        left: Box<LogicExpr>,
        op: LogicOp,
        right: Box<LogicExpr>,
    },
}
