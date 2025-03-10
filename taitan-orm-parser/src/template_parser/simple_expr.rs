use crate::template_parser::structs::atomic::Atomic;
use crate::template_parser::structs::binary_op::BinaryOp;
use nom::branch::alt;
use nom::character::complete::multispace0;
use nom::combinator::{map, opt};
use nom::sequence::preceded;
use nom::IResult;
use tracing::debug;

#[derive(Debug, PartialEq, Clone)]
pub enum SimpleExpr {
    Single(Atomic),
    Binary {
        left: Atomic,
        op: BinaryOp,
        right: Atomic,
    },
}

impl SimpleExpr {
    pub fn parse(input: &str) -> IResult<&str, SimpleExpr> {
        alt((parse_binary_expr, map(Atomic::parse, SimpleExpr::Single)))(input)
    }
}

fn parse_binary_expr(input: &str) -> IResult<&str, SimpleExpr> {
    debug!("SimpleExpr parse({})", input);
    let (input, left) = preceded(multispace0, Atomic::parse)(input)?; // 解析左操作数
    let (input, op) = preceded(multispace0, BinaryOp::parse)(input)?; // 解析操作符
    let (input, right) = preceded(multispace0, Atomic::parse)(input)?; // 解析右操作数
    let parsed = SimpleExpr::Binary { left, op, right };
    debug!("SimpleExpr parse -> {:?}", parsed);
    debug!("SimpleExpr parse -> remaining: {}", input);
    Ok((input, parsed))
}

#[cfg(test)]
mod simple_expr_test {

    use super::*;
    use crate::template_parser::structs::variable::{Variable, VariableChain};
    #[test]
    fn test_simple_expr_parse() {
        let template = "a = b";
        let (_, parsed) = SimpleExpr::parse(template).unwrap();
        let expected = SimpleExpr::Binary {
            left: Atomic::VariableChain(VariableChain::new(vec![Variable::Simple(
                "a".to_string(),
            )])),
            op: BinaryOp::Equal,
            right: Atomic::VariableChain(VariableChain::new(vec![Variable::Simple(
                "b".to_string(),
            )])),
        };
        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_simple_expr_parse_02() {
        let template = "a = b and c > d";
        let (_, parsed) = SimpleExpr::parse(template).unwrap();

        let expected = SimpleExpr::Binary {
            left: Atomic::VariableChain(VariableChain::new(vec![Variable::Simple("a".to_string())])),
            op: BinaryOp::Equal,
            right: Atomic::VariableChain(VariableChain::new(vec![Variable::Simple("b".to_string())]))
        };
        // assert_eq!(remaining, "select * from users");
        assert_eq!(parsed, expected);
    }
}
