use std::fmt::Display;
use nom::branch::alt;
use nom::combinator::map;
use nom::IResult;
use crate::template_parser::structs::bool_value::Bool;
use crate::{ToSqlSegment};
use crate::template_parser::structs::values::maybe_value::MaybeValue;
use crate::template_parser::to_sql::SqlSegment;

#[derive(Debug, Clone, PartialEq)]
pub enum BoolValue {
    Value(Bool),
    Maybe(MaybeValue)
}

impl BoolValue {
    pub fn parse(input: &str) -> IResult<&str, BoolValue> {
        alt((
            map(Bool::parse, BoolValue::Value),
            map(MaybeValue::parse, BoolValue::Maybe),
        ))(input)
    }
}
impl ToSqlSegment for BoolValue {
    fn gen_sql_segment(&self) -> SqlSegment {
        match self {
            Self::Value(v)=>SqlSegment::Simple(v.to_string()),
            Self::Maybe(m)=>m.gen_sql_segment(),
        }
    }
}

