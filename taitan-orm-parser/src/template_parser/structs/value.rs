use std::fmt::Display;
use nom::branch::alt;
use nom::combinator::map;
use nom::IResult;
use crate::{Number, Operator, Placeholder, VariableChain};
use crate::template_parser::structs::bool_value::BoolValue;
use crate::template_parser::structs::text::Text;
use crate::template_parser::{TemplatePart};


#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Bool(BoolValue),
    Number(Number),
    Text(Text),
    VariableChain(VariableChain),
    Template(TemplatePart),
}

impl Value {
    pub fn parse(input: &str) -> IResult<&str, Value> {
        alt((
            map(BoolValue::parse, Value::Bool),
            map(Number::parse, Value::Number),
            map(Text::parse, Value::Text),
            map(VariableChain::parse, Value::VariableChain),
            map(TemplatePart::parse, Value::Template),
        ))(input)
    }
}

impl Display for Value {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Bool(v) => write!(fmt, "{}", v),
            Value::Number(v) => write!(fmt, "{}", v),
            Value::Text(v) => write!(fmt, "{}", v),
            Value::VariableChain(v) => write!(fmt, "{}", v),
            Value::Template(v) => write!(fmt, "{}", v),
        }
    }
}
