use crate::template_parser::structs::binary_op::BinaryOp;
use crate::template_parser::structs::bool_value::BoolValue;
use crate::template_parser::structs::number::Number;
use crate::template_parser::structs::placeholder::Placeholder;
use crate::template_parser::structs::sign::Sign;
use crate::template_parser::structs::template_part::TemplatePart;
use crate::template_parser::structs::text::Text;
use crate::template_parser::structs::variable::VariableChain;
use crate::template_parser::to_sql::{SqlSegment, ToSqlSegment};
use nom::branch::alt;
use nom::bytes::complete::tag_no_case;
use nom::combinator::{map, not};
use nom::IResult;
use std::fmt::{Display, Formatter};
use tracing::debug;

#[derive(Debug, Clone, PartialEq)]
pub enum Atomic {
    Bool(BoolValue),
    Number(Number),
    Text(Text),
    VariableChain(VariableChain),
    Placeholder(Placeholder),
    Template(TemplatePart),
    BinaryOp(BinaryOp),
    Sign(Sign),
    Not,
}

impl Atomic {
    pub fn parse(input: &str) -> IResult<&str, Atomic> {
        debug!("Atomic parse({})", &input);
        let (remaining, parsed) = alt((
            map(BoolValue::parse, Atomic::Bool),
            map(Text::parse, Atomic::Text),
            map(Number::parse, Atomic::Number),
            map(BinaryOp::parse, Atomic::BinaryOp),
            map(Placeholder::parse, Atomic::Placeholder),
            map(VariableChain::parse, Atomic::VariableChain),
            map(TemplatePart::parse, Atomic::Template),
            map(Sign::parse, Atomic::Sign),
            map(tag_no_case("not"), |f| Atomic::Not),
        ))(input)?;
        debug!("Atomic parse -> {:?}", &parsed);
        Ok((remaining, parsed))
    }

    pub fn extract_left_bracket(&self) -> Option<Sign> {
        if let Atomic::Sign(Sign::Bracket(c)) = self {
            if c == &'(' {
                return Some(Sign::Bracket('('));
            }
        }
        None
    }
    pub fn extract_right_bracket(&self) -> Option<Sign> {
        if let Atomic::Sign(Sign::Bracket(c)) = self {
            if c == &')' {
                return Some(Sign::Bracket(')'));
            }
        }
        None
    }
}

impl ToSqlSegment for Atomic {
    fn gen_sql_segment(&self) -> SqlSegment {
        match self {
            Atomic::Not => SqlSegment::Simple("NOT".to_string()),
            Atomic::Sign(s) => SqlSegment::Simple(s.to_string()),
            Atomic::Text(t) => SqlSegment::Simple(t.to_string()),
            Atomic::Bool(b) => SqlSegment::Simple(b.to_string()),
            Atomic::Number(n) => SqlSegment::Simple(n.to_string()),
            Atomic::BinaryOp(b) => SqlSegment::Simple(b.to_string()),
            Atomic::Template(t) => SqlSegment::Simple(t.to_string()),
            Atomic::VariableChain(v) => SqlSegment::Simple(v.to_string()),
            Atomic::Placeholder(p) => p.gen_sql_segment(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AtomicStream {
    atomics: Vec<Atomic>,
}

impl AtomicStream {
    pub fn parse(input: &str) -> Result<Self, String> {
        debug!("SqlTemplate::parse({})", input);
        let mut atomics = Vec::new();
        let mut remainder = input;
        loop {
            let parse_result = Atomic::parse(remainder);
            if let Ok((remaining, part)) = parse_result {
                atomics.push(part);
                remainder = remaining;
            } else {
                let err_msg = format!("failed to parse atomic: {}", input);
                return Err(err_msg);
            }

            if remainder.is_empty() {
                break;
            }
        }
        Ok(AtomicStream { atomics })
    }
}

#[cfg(test)]
mod atomic_tests {
    use super::*;
    use crate::template_parser::structs::template_part::{EndBlock, StartBlock};
    use crate::template_parser::structs::variable::Variable;
    #[test]
    fn atomic_parser_spec_001() {
        let template = "#{hello.`test`}";
        let (_, parsed) = Atomic::parse(template).unwrap();
        let variable_chain = vec![
            Variable::Simple("hello".to_string()),
            Variable::Backquote("test".to_string()),
        ];
        assert_eq!(
            parsed,
            Atomic::Placeholder(Placeholder::Hash(VariableChain::new(
                variable_chain.clone()
            )))
        );

        let template = "'hello.`test`'";
        let (_, parsed) = Atomic::parse(template).unwrap();
        assert_eq!(
            parsed,
            Atomic::Text(Text::SingleQuote("'hello.`test`'".to_string()))
        );

        let template = "\"hello\"";
        let (_, parsed) = Atomic::parse(template).unwrap();
        let variable_chain = vec![Variable::DoubleQuote("hello".to_string())];
        assert_eq!(
            parsed,
            Atomic::VariableChain(VariableChain::new(variable_chain.clone()))
        );

        let template = r#"
        {% for item in items %}
            Item: {{ item | upper }}
        {% endfor %}"#;
        let (remaining, parsed) = Atomic::parse(template).unwrap();
        let expected = TemplatePart::ControlBlock {
            start_block: StartBlock {
                name: "for".to_string(),
                start_modifier: None,
                end_modifier: None,
                expr: "item in items".to_string(),
            },
            content: "Item: {{ item | upper }}".to_string(),
            end_block: EndBlock {
                name: "endfor".to_string(),
                start_modifier: None,
                end_modifier: None,
            },
        };
        assert_eq!(parsed, Atomic::Template(expected));
    }

    #[test]
    fn atomic_parser_spec_002() {
        let template = "¥";
        let (_, parsed) = Atomic::parse(template).unwrap();
        assert_eq!(parsed, Atomic::Sign(Sign::Unknown('¥')));
    }

    #[test]
    fn atomic_parser_spec_003() {
        let template = "1234";
        let (_, parsed) = Atomic::parse(template).unwrap();
        assert_eq!(parsed, Atomic::Number(Number("1234".to_string())));
    }
}
