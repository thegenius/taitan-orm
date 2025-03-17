use nom::branch::alt;
use nom::combinator::map;
use nom::IResult;
use crate::{Number, ToSqlSegment};
use crate::template_parser::structs::values::maybe_value::MaybeValue;
use crate::template_parser::to_sql::SqlSegment;

#[derive(Debug, Clone, PartialEq)]
pub enum NumberValue {
    Value(Number),
    Maybe(MaybeValue),
}

impl NumberValue {
    pub fn parse(input: &str) -> IResult<&str, NumberValue> {
        alt((
            map(Number::parse, NumberValue::Value),
            map(MaybeValue::parse, NumberValue::Maybe),
        ))(input)
    }
}

impl From<Number> for NumberValue {
    fn from(num: Number) -> Self {
        Self::Value(num)
    }
}

impl From<MaybeValue> for NumberValue {
    fn from(maybe: MaybeValue) -> Self {
        Self::Maybe(maybe)
    }
}

impl ToSqlSegment for NumberValue {
    fn gen_sql_segment(&self) -> SqlSegment {
        match self {
            NumberValue::Value(v) => SqlSegment::Simple(v.to_string()),
            NumberValue::Maybe(maybe_value) => maybe_value.gen_sql_segment(),
        }
    }
}