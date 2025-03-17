use taitan_orm_parser::Number;
use taitan_orm_parser::template::{ArithmeticExpr, NumberValue};
use crate::setups::logger::setup_logger;

#[test]
fn test_arithmetic_expr() {
    setup_logger();
    let template = "a + b * c / d % e - f";
    // let (_, parsed) = ArithmeticExpr::parse(template).unwrap();
    // let a = ArithmeticExpr::Value(NumberValue::Value(Number("a".to_owned())));
    // let b = ArithmeticExpr::Value(NumberValue::Value(Number("b".to_owned())));
    // let c = ArithmeticExpr::Value(NumberValue::Value(Number("c".to_owned())));
    // let d = ArithmeticExpr::Value(NumberValue::Value(Number("d".to_owned())));
    // let e = ArithmeticExpr::Value(NumberValue::Value(Number("e".to_owned())));
    // let f = ArithmeticExpr::Value(NumberValue::Value(Number("f".to_owned())));
    // let expected = ArithmeticExpr::Nested(Box::new(a));
    // assert_eq!(parsed, expected);
}