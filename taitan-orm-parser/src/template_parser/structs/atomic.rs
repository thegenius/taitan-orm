use crate::template_parser::structs::operators::Operator;
use crate::template_parser::structs::bool_value::Bool;
use crate::template_parser::structs::number::Number;
use crate::template_parser::structs::placeholder::Placeholder;
use crate::template_parser::structs::sign::Sign;
use crate::template_parser::structs::template_part::TemplatePart;
use crate::template_parser::structs::text::Text;
use crate::template_parser::structs::variable::VariableChain;
use crate::template_parser::to_sql::{SqlSegment, ToSqlSegment};
use nom::branch::alt;
use nom::bytes::complete::tag_no_case;
use nom::character::complete::multispace0;
use nom::combinator::{map, not};
use nom::sequence::preceded;
use nom::IResult;
use std::fmt::{Display, Formatter};
use tracing::debug;
use crate::template_parser::structs::connect_op::ConnectOp;
use crate::template_parser::structs::values::GenericValue;
use crate::template_parser::to_sql::SqlSegment::Simple;

#[derive(Debug, Clone, PartialEq)]
pub enum Atomic {
    Value(GenericValue),
    Placeholder(Placeholder),
    Operator(Operator),
    ConnectOp(ConnectOp),
    Sign(Sign),
}

impl Atomic {
    pub fn parse(input: &str) -> IResult<&str, Atomic> {
        debug!("Atomic parse({})", &input);
        let (remaining, parsed) = alt((
            map(GenericValue::parse, Atomic::Value),
            map(Operator::parse, Atomic::Operator),
            map(ConnectOp::parse, Atomic::ConnectOp),
            map(Placeholder::parse, Atomic::Placeholder),
            map(Sign::parse, Atomic::Sign),
        ))(input)?;
        debug!("Atomic parse -> {:?}", &parsed);
        Ok((remaining, parsed))
    }

    pub fn is_binary_op(&self) -> bool {
        match self {
            Atomic::Operator(_) => true,
            _ => false,
        }
    }
    pub fn is_operand(&self) -> bool {
        match self {
            Atomic::Operator(_) | Atomic::Sign(_)  => false,
            _ => true,
        }
    }

    pub fn extract_binary_op(&self) -> Option<Operator> {
        if let Atomic::Operator(o) = self {
            return Some(o.clone());
        }
        None
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
            Atomic::Sign(s) => SqlSegment::Simple(s.to_string()),
            Atomic::Value(v) => SqlSegment::Simple(v.gen_sql_segment().to_sql(false).to_string()),
            Atomic::Operator(b) => SqlSegment::Simple(b.to_string()),
            Atomic::ConnectOp(c) => SqlSegment::Simple(c.to_string()),
            Atomic::Placeholder(p) => p.gen_sql_segment(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AtomicStream {
    pub atomics: Vec<Atomic>,
}

impl AtomicStream {
    pub fn parse(input: &str) -> Result<Self, String> {
        debug!("SqlTemplate::parse({})", input);
        let mut atomics = Vec::new();
        let mut remainder = input;
        loop {
            let parse_result = preceded(multispace0, Atomic::parse)(remainder);
            match parse_result {
                Ok((remaining, parsed)) => {
                    debug!("SqlTemplate::parse({})->{:?}", remaining, parsed);
                    atomics.push(parsed);
                    remainder = remaining;
                }
                Err(err_msg) => {
                    debug!("SqlTemplate::parse error: {}", err_msg);
                    let err_msg = format!("failed to parse atomic: {}", input);
                    return Err(err_msg);
                }
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
    use crate::template_parser::structs::values::MaybeValue;
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
            Atomic::Value(GenericValue::Text(Text::SingleQuote("'hello.`test`'".to_string())))
        );

        let template = "\"hello\"";
        let (_, parsed) = Atomic::parse(template).unwrap();
        let variable_chain = vec![Variable::DoubleQuote("hello".to_string())];
        assert_eq!(
            parsed,
            Atomic::Value( GenericValue::Maybe(MaybeValue::VariableChain(VariableChain::new(variable_chain.clone()))))
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
        assert_eq!(parsed, Atomic::Value( GenericValue::Maybe(MaybeValue::TemplatePart(expected))));
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
        assert_eq!(parsed, Atomic::Value(GenericValue::Number(Number("1234".to_string()))));
    }
}
