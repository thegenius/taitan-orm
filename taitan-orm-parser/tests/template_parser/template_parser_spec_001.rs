use proc_macro2::TokenStream;
use taitan_orm_parser::ToSqlSegment;
use taitan_orm_parser::{Atomic, Expr, Sign};
use taitan_orm_parser::{SimpleExpr, SqlPart, SqlTemplate, Variable, VariableChain};

use crate::setups::logger::setup_logger;

#[test]
fn template_parser_spec_001() {
    setup_logger();
    // let template = "select * from users";
    // let parsed_template = SqlTemplate::parse(template).unwrap();
    // let expected = SqlTemplate::new(vec![
    //     SqlPart::Expr(Expr::Simple(SimpleExpr::Single(Atomic::VariableChain(
    //         VariableChain::new(vec![Variable::Simple("select".to_string())]),
    //     )))),
    //     SqlPart::Expr(Expr::Simple(SimpleExpr::Single(Atomic::Sign(Sign::Star)))),
    //     SqlPart::Expr(Expr::Simple(SimpleExpr::Single(Atomic::VariableChain(
    //         VariableChain::new(vec![Variable::Simple("from".to_string())]),
    //     )))),
    //     SqlPart::Expr(Expr::Simple(SimpleExpr::Single(Atomic::VariableChain(
    //         VariableChain::new(vec![Variable::Simple("users".to_string())]),
    //     )))),
    // ]);
    //
    // assert_eq!(parsed_template, expected);

    // let segments = parsed_template.gen_sql_segments();
    // let streams: Vec<TokenStream> = segments.iter().map(|f| f.to_sql(false)).collect();
    // let streams_str: String = streams
    //     .iter()
    //     .map(|f| f.to_string())
    //     .collect::<Vec<_>>()
    //     .join(" ");
    // assert_eq!(streams_str, "s . push_str (\"select \") ; s . push_str (\"* \") ; s . push_str (\"from \") ; s . push_str (\"users \") ;");
}
