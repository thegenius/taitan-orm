// use std::borrow::Cow;
// use taitan_orm_macro::{LocationNew, Parameter};
// use taitan_orm_trait::brave_new::Location;
// use taitan_orm_trait::{CmpOperator, LocationExpr};
//
// #[derive(Debug, Parameter, LocationNew)]
// enum LocationSpec001 {
//     A {
//         name: LocationExpr<String>,
//         age: LocationExpr<i32>,
//     },
//     B {
//         # [field(name = c_name, db_type = BIGINT, nullable = true, auto_inc = true)]
//         cname: LocationExpr<String>,
//         # [field(name = c_age, db_type = BIGINT, nullable = true, auto_inc = true)]
//         cage: LocationExpr<i64>,
//     },
// }
//
//
// #[test]
// fn location_spec_001() {
//     let location = LocationSpec001::A {
//         name: LocationExpr::new(CmpOperator::Eq, "a".to_string()),
//         age: LocationExpr::new(CmpOperator::GreaterThan, 23)
//     };
//     let where_sql = Location::<sqlx::Sqlite>::gen_where_sql(&location);
//     assert_eq!(where_sql, "name=? AND age>?");
//
//     let location = LocationSpec001::B {
//         cname: LocationExpr::new(CmpOperator::LessOrEq, "a".to_string()),
//         cage: LocationExpr::new(CmpOperator::GreaterThan, 23)
//     };
//     let where_sql = Location::<sqlx::Sqlite>::gen_where_sql(&location);
//     assert_eq!(where_sql, "c_name<=? AND c_age>?");
// }
