use taitan_orm_parser::{Atomic, AtomicStream, ComparisonOp, Expr, Parser, Variable, VariableChain};
use taitan_orm_parser::BinaryOp;
use crate::setups::logger::setup_logger;

#[test]
fn test_syntax_parser() {
    setup_logger();
    let template = "a>=b and c=d or e!=null and f<>6, test>?";
    let atomics = AtomicStream::parse(template).unwrap();
    assert_eq!(atomics.atomics.len(), 19);

    let expr = Parser::parse(atomics.atomics).unwrap();
    let expected_expr = Expr::Simple {
        left: Atomic::VariableChain(VariableChain::new(vec![Variable::Simple("a".to_string())])),
        op: BinaryOp::Compare(ComparisonOp::GreaterThanOrEqual),
        right: Atomic::VariableChain(VariableChain::new(vec![Variable::Simple("b".to_string())])),
    };
    assert_eq!(expr, expected_expr);
}
