use taitan_orm_parser::{Atomic, AtomicStream, Number, Variable, VariableChain, Placeholder, RawPlaceholder};

use taitan_orm_parser::template::{CompareOp, GenericExpr, MaybeValue, LogicOp, GenericAtomicStream, GenericAtomic, MySqlAtomic};
use crate::setups::logger::setup_logger;

#[test]
fn test_syntax_parser() {
    setup_logger();
    let template = "a>=b";
    let atomics = GenericAtomicStream::parse::<MySqlAtomic>(template).unwrap();
    let (_,expr1) = GenericExpr::parse(atomics.atomics).unwrap();
    let expected = GenericExpr::CompareExpr {
        left: Box::new(GenericExpr::Atomic(GenericAtomic::Maybe(MaybeValue::VariableChain(VariableChain::new(vec![Variable::Simple("a".to_string())]))))),
        op: CompareOp::GreaterThanOrEqual,
        right: Box::new(GenericExpr::Atomic(GenericAtomic::Maybe(MaybeValue::VariableChain(VariableChain::new(vec![Variable::Simple("b".to_string())])))))
    };
    assert_eq!(expected, expr1);

    let expr2 = GenericExpr::parse_str::<MySqlAtomic>("c=d").unwrap();
    let expr3 = GenericExpr::parse_str::<MySqlAtomic>("e!=null").unwrap();
    let expr4 = GenericExpr::parse_str::<MySqlAtomic>("f<>6").unwrap();
    let expr5 = GenericExpr::parse_str::<MySqlAtomic>("test>?").unwrap();


    let template = "a>=b and c=d or e!=null and f<>6 or test>?";
    let expr = GenericExpr::parse_str::<MySqlAtomic>(template).unwrap();
    let expected = GenericExpr::LogicExpr {
        left: Box::new(GenericExpr::LogicExpr {
            left: Box::new(GenericExpr::LogicExpr {
                left: Box::new(expr1),
                op: LogicOp::And,
                right: Box::new(expr2),
            }),
            op: LogicOp::Or,
            right: Box::new(GenericExpr::LogicExpr {
                left: Box::new(expr3),
                op: LogicOp::And,
                right: Box::new(expr4),
            }),
        }),
        op: LogicOp::Or,
        right: Box::new(expr5),
    };

    assert_eq!(expr, expected);
}
