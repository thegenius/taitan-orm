use std::str::FromStr;
use crate::template_parser::structs::variable::VariableChain;
use crate::template_parser::to_sql::{SqlSegment, ToSqlSegment};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, multispace0};
use nom::combinator::map;
use nom::sequence::{delimited, preceded, tuple};
use nom::IResult;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum RawPlaceholder {
    QuestionMark,
    Indexed(usize),
    Named(String),
}

impl RawPlaceholder {
    pub fn parse(input: &str) -> IResult<&str, RawPlaceholder> {
        alt((
            map(tag("?"), |_| RawPlaceholder::QuestionMark),
            map(preceded(tag("$"), digit1), |d| {
                RawPlaceholder::Indexed(usize::from_str(d).unwrap())
            }),
            map(
                preceded(tag(":"), preceded(multispace0, VariableChain::parse)),
                |v| RawPlaceholder::Named(v.to_string()),
            ),
        ))(input)
    }
}

impl ToSqlSegment for RawPlaceholder {
    fn gen_sql_segment(&self) -> SqlSegment {
        match self {
            RawPlaceholder::QuestionMark => SqlSegment::Simple("?".to_string()),
            RawPlaceholder::Indexed(i) => SqlSegment::Simple(format!("${}", i)),
            RawPlaceholder::Named(i) => SqlSegment::Simple(i.to_string()),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Placeholder {
    Raw(RawPlaceholder),
    Dollar(VariableChain),
    Hash(VariableChain),
    At(VariableChain),
}
impl Placeholder {
    pub fn parse(input: &str) -> IResult<&str, Placeholder> {
        alt((
            map(RawPlaceholder::parse, Placeholder::Raw),
            delimited(
                preceded(tag("$"), preceded(multispace0, tag("{"))),
                preceded(multispace0, map(VariableChain::parse, Placeholder::Dollar)),
                preceded(multispace0, tag("}")),
            ),
            delimited(
                preceded(tag("#"), preceded(multispace0, tag("{"))),
                preceded(multispace0, map(VariableChain::parse, Placeholder::Hash)),
                preceded(multispace0, tag("}")),
            ),
            delimited(
                preceded(tag("@"), preceded(multispace0, tag("{"))),
                preceded(multispace0, map(VariableChain::parse, Placeholder::At)),
                preceded(multispace0, tag("}")),
            ),
        ))(input)
    }
}

impl ToSqlSegment for Placeholder {
    fn gen_sql_segment(&self) -> SqlSegment {
        match self {
            Placeholder::Dollar(p) => SqlSegment::Dollar(p.to_string()),
            Placeholder::Hash(p) => SqlSegment::Hash(p.to_string()),
            Placeholder::At(p) => SqlSegment::At(p.to_string()),
            Placeholder::Raw(p)=>p.gen_sql_segment(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::template_parser::structs::variable::Variable;
    #[test]
    fn placeholder_parser_spec_001() {
        let template = "@{ hello. name. attr1 }";
        let (_, placeholder) = Placeholder::parse(template).unwrap();
        let variable_chain = vec![
            Variable::Simple("hello".to_string()),
            Variable::Simple("name".to_string()),
            Variable::Simple("attr1".to_string()),
        ];
        assert_eq!(
            placeholder,
            Placeholder::At(VariableChain::new(variable_chain))
        );

        let template = "#{ [ hello ]. \" name \". ` attr1 ` }";
        let (_, placeholder) = Placeholder::parse(template).unwrap();
        let variable_chain = vec![
            Variable::Brackets("hello".to_string()),
            Variable::DoubleQuote("name".to_string()),
            Variable::Backquote("attr1".to_string()),
        ];
        assert_eq!(
            placeholder,
            Placeholder::Hash(VariableChain::new(variable_chain))
        );

        let template = "${ [ hello ]. name . ` attr1 ` }";
        let (_, placeholder) = Placeholder::parse(template).unwrap();
        let variable_chain = vec![
            Variable::Brackets("hello".to_string()),
            Variable::Simple("name".to_string()),
            Variable::Backquote("attr1".to_string()),
        ];
        assert_eq!(
            placeholder,
            Placeholder::Dollar(VariableChain::new(variable_chain))
        );
    }

    #[test]
    fn placeholder_parser_spec_002() {
        let template = "@{ hello. name. attr1. }";
        let parse_result = Placeholder::parse(template);
        assert!(parse_result.is_err());
        if let Err(nom::Err::Error(error)) = parse_result {
            assert_eq!(error.to_string(), "error Tag at: . }");
        }
    }
}
