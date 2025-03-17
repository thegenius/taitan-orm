use std::fmt::Display;
use nom::branch::alt;
use nom::combinator::map;
use nom::IResult;
use crate::template_parser::TemplatePart;
use crate::{Placeholder, ToSqlSegment, VariableChain};
use crate::template_parser::to_sql::SqlSegment;

#[derive(Debug, Clone, PartialEq)]
pub enum MaybeValue {
    VariableChain(VariableChain),
    TemplatePart(TemplatePart),
    Placeholder(Placeholder),
}

impl MaybeValue {
    pub fn parse(input: &str) -> IResult<&str, MaybeValue> {
        alt((
            map(VariableChain::parse, MaybeValue::VariableChain),
            map(TemplatePart::parse, MaybeValue::TemplatePart),
            map(Placeholder::parse, MaybeValue::Placeholder),
        ))(input)
    }
}

impl ToSqlSegment for MaybeValue {
    fn gen_sql_segment(&self) -> SqlSegment {
        match self {
            Self::VariableChain(v) => SqlSegment::Simple(v.to_string()),
            Self::TemplatePart(v)=>SqlSegment::Simple(v.to_string()),
            Self::Placeholder(v) => v.gen_sql_segment(),
        }
    }
}