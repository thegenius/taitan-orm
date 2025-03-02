use taitan_orm_macro::{LocationNew, Parameter};
use taitan_orm_trait::brave_new::Location;
use taitan_orm_trait::{CmpOperator, LocationExpr};

#[derive(Debug, Parameter, LocationNew)]
struct LocationSpec003 {
    a: LocationExpr<String>,
    # [field(name = c_name, db_type = BIGINT, nullable = true, auto_inc = true)]
    b: LocationExpr<String>,
}

#[test]
fn location_spec_003() {
    let location = LocationSpec003{
        a: LocationExpr::new(CmpOperator::Eq, "a".to_string()),
        b: LocationExpr::new(CmpOperator::Eq, "b".to_string())
    };
    let where_sql = Location::<sqlx::Sqlite>::gen_where_sql(&location);
    assert_eq!(where_sql, "a=?");

    let location = LocationSpec003{
        a: LocationExpr::new(CmpOperator::Eq, "a".to_string()),
        b: LocationExpr::new(CmpOperator::Eq, "b".to_string())
    };
    let where_sql = Location::<sqlx::Sqlite>::gen_where_sql(&location);
    assert_eq!(where_sql, "user_name=?");

    let location = LocationSpec003{
        a: LocationExpr::new(CmpOperator::Eq, "a".to_string()),
        b: LocationExpr::new(CmpOperator::Eq, "b".to_string())
    };
    let where_sql = Location::<sqlx::Sqlite>::gen_where_sql(&location);
    assert_eq!(where_sql, "\"select\">?");

    let location = LocationSpec003{
        a: LocationExpr::new(CmpOperator::Eq, "a".to_string()),
        b: LocationExpr::new(CmpOperator::Eq, "b".to_string())
    };
    let where_sql = Location::<sqlx::Sqlite>::gen_where_sql(&location);
    assert_eq!(where_sql, "c_name>?");
}
