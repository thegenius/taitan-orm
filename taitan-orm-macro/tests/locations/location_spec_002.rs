use taitan_orm_macro::{LocationNew, Parameter};
use taitan_orm_trait::{CmpOperator, LocationExpr};
use taitan_orm_trait::brave_new::Location;

#[derive(Debug, Parameter, LocationNew)]
enum LocationSpec002 {
    A(LocationExpr<String>),
}

#[test]
fn location_spec_002() {
    let location = LocationSpec002::A(LocationExpr::new(CmpOperator::Eq, "a".to_string()));
    let where_sql = location.gen_where_sql();
    assert_eq!(where_sql, "a=?");
}
