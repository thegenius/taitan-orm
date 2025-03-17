use crate::template_parser::structs::values::{TextValue};

#[derive(Debug, Clone, PartialEq)]
pub enum TextExpr {
    Value(TextValue),
    Nested(Box<TextExpr>),
}