use taitan_orm_macro::{LocationNew, Parameter};
use taitan_orm_trait::{CmpOperator, LocationExpr};
use taitan_orm_trait::brave_new::Location;

#[derive(Debug, Parameter, LocationNew)]
struct LocationSpec001 {
    a: LocationExpr<String>,
}

#[test]
fn location_spec_001() {
    let location = LocationSpec001 {
        a: LocationExpr::new(CmpOperator::Eq, "a".to_string()),
    };
    let where_sql = location.gen_where_sql();
    assert_eq!(where_sql, "a=?");
}
