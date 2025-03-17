use nom::branch::alt;
use nom::combinator::map;
use nom::IResult;
use crate::template_parser::structs::text::Text;
use crate::template_parser::structs::values::number_value::NumberValue;
use crate::template_parser::TemplatePart;
use crate::{Number, ToSqlSegment, VariableChain};
use crate::template_parser::structs::values::maybe_value::MaybeValue;
use crate::template_parser::to_sql::SqlSegment;

#[derive(Debug, Clone, PartialEq)]
pub enum TextValue {
    Value(Text),
    Maybe(MaybeValue),
}

impl TextValue {
    pub fn parse(input: &str) -> IResult<&str, TextValue> {
        alt((
            map(Text::parse, TextValue::Value),
            map(MaybeValue::parse, TextValue::Maybe),
        ))(input)
    }
}

impl ToSqlSegment for TextValue {
    fn gen_sql_segment(&self) -> SqlSegment {
        match self {
            TextValue::Value(v) => SqlSegment::Simple(v.to_string()),
            TextValue::Maybe(maybe_value) => maybe_value.gen_sql_segment(),
        }
    }
}