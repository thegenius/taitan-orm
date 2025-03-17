use crate::template_parser::structs::bool_value::Bool;
use crate::template_parser::structs::text::Text;
use crate::template_parser::structs::values::maybe_value::MaybeValue;
use crate::template_parser::to_sql::SqlSegment;
use crate::{Number, ToSqlSegment};
use nom::branch::alt;
use nom::combinator::map;
use nom::IResult;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum GenericValue {
    Bool(Bool),
    Number(Number),
    Text(Text),
    Maybe(MaybeValue),
}

impl GenericValue {
    pub fn parse(input: &str) -> IResult<&str, GenericValue> {
        alt((
            map(Bool::parse, GenericValue::Bool),
            map(Number::parse, GenericValue::Number),
            map(Text::parse, GenericValue::Text),
            map(MaybeValue::parse, GenericValue::Maybe),
        ))(input)
    }
}

impl ToSqlSegment for GenericValue {
    fn gen_sql_segment(&self) -> SqlSegment {
        match self {
            GenericValue::Bool(v) => SqlSegment::Simple(v.to_string()),
            GenericValue::Text(v) => SqlSegment::Simple(v.to_string()),
            GenericValue::Number(v) => SqlSegment::Simple(v.to_string()),
            GenericValue::Maybe(maybe_value) => maybe_value.gen_sql_segment(),
        }
    }
}
