use crate::template_parser::structs::text::Text;
use crate::template_parser::structs::values::maybe_value::MaybeValue;
use crate::template_parser::to_sql::SqlSegment;
use crate::ToSqlSegment;
use nom::branch::alt;
use nom::combinator::map;
use nom::IResult;

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

impl From<Text> for TextValue {
    fn from(v: Text) -> Self {
        Self::Value(v)
    }
}

impl From<MaybeValue> for TextValue {
    fn from(v: MaybeValue) -> Self {
        Self::Maybe(v)
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
