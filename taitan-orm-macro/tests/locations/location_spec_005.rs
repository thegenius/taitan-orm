// use std::borrow::Cow;
// use taitan_orm_macro::{LocationNew, Parameter};
// use taitan_orm_trait::brave_new::Location;
// use taitan_orm_trait::{CmpOperator, LocationExpr};
//
// #[derive(Debug, Parameter, LocationNew)]
// enum LocationSpec005 {
//     A(String),
//     B {
//         # [field(name = c_name, db_type = BIGINT, nullable = true, auto_inc = true)]
//         name: LocationExpr<String>,
//     },
// }
//
// #[test]
// fn location_spec_005() {
//     let location = LocationSpec005 {
//         a: LocationExpr::new(CmpOperator::Eq, "a".to_string()),
//         b: LocationExpr::new(CmpOperator::LessOrEq, "b".to_string()),
//     };
//     let where_sql = Location::<sqlx::Sqlite>::gen_where_sql(&location);
//     assert_eq!(where_sql, "a=? AND c_name<=?");
// }
