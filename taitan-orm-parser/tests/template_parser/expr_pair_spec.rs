use taitan_orm_parser::{Atomic, SqlPart, SqlTemplate, Number};


#[test]
pub fn test_expr_pair_spec() {
    let template = "a>=b and c=d or e!=null and f<>6";
    // let parsed = SqlTemplate::parse(template).unwrap();
    // let expected = SqlTemplate::new(vec![
    //     SqlPart::Atomic(Atomic::Number(Number("a".to_owned())))
    // ]);
    // assert_eq!(parsed, expected);
}