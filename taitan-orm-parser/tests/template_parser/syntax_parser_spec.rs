use taitan_orm_parser::{Atomic, AtomicStream, Number,  Variable, VariableChain, Placeholder, RawPlaceholder};
use taitan_orm_parser::template::GenericExpr;
use crate::setups::logger::setup_logger;

#[test]
fn test_syntax_parser() {
    setup_logger();
    let template = "a>=b and c=d or e!=null and f<>6 or test>?";
    let atomics = AtomicStream::parse(template).unwrap();
    assert_eq!(atomics.atomics.len(), 19);

    let expr = GenericExpr::parse(atomics.atomics).unwrap();
    // let expected = Expr::BinaryExpr {
    //     left: Box::new(Expr::Simple {
    //         left: Atomic::VariableChain(VariableChain { variables: vec![Variable::Simple("a".to_string())] }),
    //         op: Operator::Compare(ComparisonOp::GreaterThanOrEqual),
    //         right: Atomic::VariableChain(VariableChain { variables: vec![Variable::Simple("b".to_string())] }),
    //     }),
    //     op: Operator::Logic(LogicOp::And),
    //     right: Box::new(Expr::BinaryExpr {
    //         left: Box::new(Expr::Simple {
    //             left: Atomic::VariableChain(VariableChain { variables: vec![Variable::Simple("c".to_string())] }),
    //             op: Operator::Compare(ComparisonOp::Equal),
    //             right: Atomic::VariableChain(VariableChain { variables: vec![Variable::Simple("d".to_string())] }),
    //         }),
    //         op: Operator::Logic(LogicOp::Or),
    //         right: Box::new(Expr::BinaryExpr {
    //             left: Box::new(Expr::Simple {
    //                 left: Atomic::VariableChain(VariableChain { variables: vec![Variable::Simple("e".to_string())] }),
    //                 op: Operator::Compare(ComparisonOp::NotEqual),
    //                 right: Atomic::VariableChain(VariableChain { variables: vec![Variable::Simple("null".to_string())] }),
    //             }),
    //             op: Operator::Logic(LogicOp::And),
    //             right: Box::new(Expr::BinaryExpr {
    //                 left: Box::new(Expr::Simple {
    //                     left: Atomic::VariableChain(VariableChain { variables: vec![Variable::Simple("f".to_string())] }),
    //                     op: Operator::Compare(ComparisonOp::NotEqual),
    //                     right: Atomic::Number(Number("6".to_string())),
    //                 }),
    //                 op: Operator::Comma,
    //                 right: Box::new(Expr::Simple {
    //                     left: Atomic::VariableChain(VariableChain { variables: vec![Variable::Simple("test".to_string())] }),
    //                     op: Operator::Compare(ComparisonOp::GreaterThan),
    //                     right: Atomic::Placeholder(Placeholder::Raw(RawPlaceholder::QuestionMark)),
    //                 }),
    //             }),
    //         }),
    //     }),
    // };

    // assert_eq!(expr, expected);
}
