use std::borrow::Cow;
use taitan_orm_macro::{Location, Parameter};
use taitan_orm_trait::traits::Location;
use taitan_orm_trait::op::{Cmp, Expr};

#[derive(Debug, Parameter, Location)]
enum LocationSpec001 {
    A {
        name: Expr<String>,
        age: Expr<i32>,
    },
    B {
        # [field(name = c_name, db_type = BIGINT, nullable = true, auto_inc = true)]
        cname: Expr<String>,
        # [field(name = c_age, db_type = BIGINT, nullable = true, auto_inc = true)]
        cage: Expr<i64>,
    },
}


#[test]
fn location_spec_001() {
    let location = LocationSpec001::A {
        name: Expr::new(Cmp::Eq, "a".to_string()),
        age: Expr::new(Cmp::GreaterThan, 23)
    };
    let where_sql = Location::<sqlx::Sqlite>::gen_where_sql(&location);
    assert_eq!(where_sql, "name=? AND age>?");

    let location = LocationSpec001::B {
        cname: Expr::new(Cmp::LessOrEq, "a".to_string()),
        cage: Expr::new(Cmp::GreaterThan, 23)
    };
    let where_sql = Location::<sqlx::Sqlite>::gen_where_sql(&location);
    assert_eq!(where_sql, "c_name<=? AND c_age>?");
}
