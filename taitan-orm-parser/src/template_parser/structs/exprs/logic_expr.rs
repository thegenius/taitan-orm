
use crate::template_parser::structs::operators::{ListOp, LogicOp};
use crate::template_parser::structs::values::{BoolValue, TextValue};
use crate::Operator;
use crate::template_parser::MatchOp;
use crate::template_parser::structs::exprs::text_expr::TextExpr;

#[derive(Debug, Clone, PartialEq)]
pub enum LogicExpr {
    Value(BoolValue),

    // a like 'xxx', a like ('nested')
    // 'aaa' like 'bbb'
    // 'aaa' = 'bbb'
    TextMatchExpr {
        left: Box<TextExpr>,
        op: MatchOp,
        right: Box<TextExpr>,
    },
    ListMatchExpr {
        left: Box<TextValue>,
        op: ListOp,
        right: Box<TextValue>,
    },

    Nested(Box<LogicExpr>),
    Expr {
        left: Box<LogicExpr>,
        op: LogicOp,
        right: Box<LogicExpr>,
    },
}
