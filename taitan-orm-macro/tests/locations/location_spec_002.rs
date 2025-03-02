// use taitan_orm_macro::{LocationNew, Parameter};
// use taitan_orm_trait::brave_new::Location;
// use taitan_orm_trait::{CmpOperator, LocationExpr};
//
// #[derive(Debug, Parameter, LocationNew)]
// enum LocationSpec002 {
//     A(LocationExpr<String>),
//     UserName(LocationExpr<String>),
//     Select(LocationExpr<String>),
//     Insert(
//         # [field(name = c_name, db_type = BIGINT, nullable = true, auto_inc = true)]
//         LocationExpr<String>
//     ),
// }
//
// #[test]
// fn location_spec_002() {
//     let location = LocationSpec002::A(LocationExpr::new(CmpOperator::Eq, "a".to_string()));
//     let where_sql = Location::<sqlx::Sqlite>::gen_where_sql(&location);
//     assert_eq!(where_sql, "a=?");
//
//     let location = LocationSpec002::UserName(LocationExpr::new(CmpOperator::Eq, "t".to_string()));
//     let where_sql = Location::<sqlx::Sqlite>::gen_where_sql(&location);
//     assert_eq!(where_sql, "user_name=?");
//
//     let location = LocationSpec002::Select(LocationExpr::new(CmpOperator::GreaterThan, "t".to_string()));
//     let where_sql = Location::<sqlx::Sqlite>::gen_where_sql(&location);
//     assert_eq!(where_sql, "\"select\">?");
//
//     let location = LocationSpec002::Insert(LocationExpr::new(CmpOperator::GreaterThan, "t".to_string()));
//     let where_sql = Location::<sqlx::Sqlite>::gen_where_sql(&location);
//     assert_eq!(where_sql, "c_name>?");
// }
