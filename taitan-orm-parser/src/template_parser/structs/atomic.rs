use crate::template_parser::structs::binary_op::BinaryOp;
use crate::template_parser::structs::bool_value::BoolValue;
use crate::template_parser::structs::number::Number;
use crate::template_parser::structs::placeholder::Placeholder;
use crate::template_parser::structs::sign::Sign;
use crate::template_parser::structs::text::Text;
use crate::template_parser::structs::variable::VariableChain;
use nom::branch::alt;
use nom::combinator::{map};
use nom::IResult;
use tracing::debug;
use crate::template_parser::structs::template_part::TemplatePart;

#[derive(Debug, Clone, PartialEq)]
pub enum Atomic {
    Bool(BoolValue),
    Number(Number),
    Text(Text),
    VariableChain(VariableChain),
    Placeholder(Placeholder),
    BinaryOp(BinaryOp),
    Sign(Sign),
    Template(TemplatePart),
}

impl Atomic {
    pub fn parse(input: &str) -> IResult<&str, Atomic> {
        debug!("Atomic parse({})", &input);
        let (remaining, parsed)= alt((
            map(BoolValue::parse, Atomic::Bool),
            map(Text::parse, Atomic::Text),
            map(Number::parse, Atomic::Number),
            map(BinaryOp::parse, Atomic::BinaryOp),
            map(Placeholder::parse, Atomic::Placeholder),
            map(VariableChain::parse, Atomic::VariableChain),
            map(TemplatePart::parse, Atomic::Template),
            map(Sign::parse, Atomic::Sign),
        ))(input)?;
        debug!("Atomic parse -> {:?}", &parsed);
        Ok((remaining, parsed))
    }
}

#[cfg(test)]
mod atomic_tests {
    use crate::template_parser::structs::template_part::{EndBlock, StartBlock};
    use crate::template_parser::structs::variable::Variable;
    use super::*;
    #[test]
    fn atomic_parser_spec_001() {
        let template = "#{hello.`test`}";
        let (_, parsed) = Atomic::parse(template).unwrap();
        let variable_chain = vec![
            Variable::Simple("hello".to_string()),
            Variable::Backquote("test".to_string()),
        ];
        assert_eq!(parsed, Atomic::Placeholder(Placeholder::Hash(VariableChain::new(variable_chain.clone()))));

        let template = "'hello.`test`'";
        let (_, parsed) = Atomic::parse(template).unwrap();
        assert_eq!(parsed, Atomic::Text(Text::SingleQuote("'hello.`test`'".to_string())));

        let template = "\"hello\"";
        let (_, parsed) = Atomic::parse(template).unwrap();
        let variable_chain = vec![
            Variable::DoubleQuote("hello".to_string()),
        ];
        assert_eq!(parsed, Atomic::VariableChain(VariableChain::new(variable_chain.clone())));

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
