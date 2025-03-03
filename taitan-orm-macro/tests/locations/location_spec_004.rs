use std::borrow::Cow;
use taitan_orm_macro::{LocationNew, Parameter};
use taitan_orm_trait::brave_new::Location;
use taitan_orm_trait::{CmpOperator, LocationExpr};

#[derive(Debug, Parameter, LocationNew)]
struct LocationSpec004 {
    a: String,
    # [field(name = c_name, db_type = BIGINT, nullable = true, auto_inc = true)]
    b: String,
}


#[test]
fn location_spec_004() {
    let location = LocationSpec004 {
        a: "a".to_string(),
        b: "b".to_string(),
    };
    let where_sql = Location::<sqlx::Sqlite>::gen_where_sql(&location);
    assert_eq!(where_sql, "a=? AND c_name<=?");
}
