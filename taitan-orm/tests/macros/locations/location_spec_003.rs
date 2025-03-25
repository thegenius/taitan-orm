use taitan_orm_macro::{Location, Parameter};
use taitan_orm_trait::traits::Location;
use taitan_orm_trait::op::{Cmp, Expr};

#[derive(Debug, Parameter, Location)]
struct LocationSpec003 {
    a: Expr<String>,
    # [field(name = c_name, db_type = BIGINT, nullable = true, auto_inc = true)]
    b: Expr<String>,
}

#[test]
fn location_spec_003() {
    let location = LocationSpec003 {
        a: Expr::new(Cmp::Eq, "a".to_string()),
        b: Expr::new(Cmp::LessOrEq, "b".to_string()),
    };
    let where_sql = Location::<sqlx::Sqlite>::gen_where_sql(&location);
    assert_eq!(where_sql, "a=? AND c_name<=?");
}
