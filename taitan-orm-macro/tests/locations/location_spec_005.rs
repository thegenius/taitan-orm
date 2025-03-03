use std::borrow::Cow;
use taitan_orm_macro::{LocationNew, Parameter};
use taitan_orm_trait::brave_new::Location;
use taitan_orm_trait::{CmpOperator, LocationExpr};

#[derive(Debug, Parameter, LocationNew)]
enum LocationSpec005 {
    A(String),
    B {
        # [field(name = c_name, db_type = BIGINT, nullable = true, auto_inc = true)]
        name: LocationExpr<String>,

        # [field(name = c_age, db_type = BIGINT, nullable = true, auto_inc = true)]
        age: i64,
    },
}

#[test]
fn location_spec_005() {
    let location = LocationSpec005::A("a".to_string());
    let where_sql = Location::<sqlx::Sqlite>::gen_where_sql(&location);
    assert_eq!(where_sql, "a=?");

    let location = LocationSpec005::B {
        name: LocationExpr::new(CmpOperator::GreaterOrEq, "a".to_string()),
        age: 0,
    };
    let where_sql = Location::<sqlx::Sqlite>::gen_where_sql(&location);
    assert_eq!(where_sql, "c_name>=? AND c_age=?");
}
