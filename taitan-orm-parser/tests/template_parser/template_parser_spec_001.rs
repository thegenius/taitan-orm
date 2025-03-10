use taitan_orm_parser::{Atomic, Expr, Sign};
use taitan_orm_parser::{SimpleExpr, SqlSegment, SqlTemplate, Variable, VariableChain};

use crate::setups::logger::setup_logger;

#[test]
fn template_parser_spec_001() {
    setup_logger();
    let template = "select * from users";
    let parsed_template = SqlTemplate::parse(template).unwrap();
    let expected = SqlTemplate::new(vec![
        SqlSegment::Expr(Expr::Simple(SimpleExpr::Single(Atomic::VariableChain(
            VariableChain::new(vec![Variable::Simple("select".to_string())]),
        )))),
        SqlSegment::Expr(Expr::Simple(SimpleExpr::Single(Atomic::Sign(Sign::Star)))),
        SqlSegment::Expr(Expr::Simple(SimpleExpr::Single(Atomic::VariableChain(
            VariableChain::new(vec![Variable::Simple("from".to_string())]),
        )))),
        SqlSegment::Expr(Expr::Simple(SimpleExpr::Single(Atomic::VariableChain(
            VariableChain::new(vec![Variable::Simple("users".to_string())]),
        )))),
    ]);

    assert_eq!(parsed_template, expected);
}
