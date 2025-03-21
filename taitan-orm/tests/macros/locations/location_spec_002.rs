use taitan_orm_macro::{Location, Parameter};
use taitan_orm_trait::traits::Location;
use taitan_orm_trait::op::{Cmp, Expr};

#[derive(Debug, Parameter, Location)]
enum LocationSpec002 {
    A(Expr<String>),
    UserName(Expr<String>),
    Select(Expr<String>),
    Insert(
        # [field(name = c_name, db_type = BIGINT, nullable = true, auto_inc = true)]
        Expr<String>
    ),
}

#[test]
fn location_spec_002() {
    let location = LocationSpec002::A(Expr::new(Cmp::Eq, "a".to_string()));
    let where_sql = Location::<sqlx::Sqlite>::gen_where_sql(&location);
    assert_eq!(where_sql, "a=?");

    let location = LocationSpec002::UserName(Expr::new(Cmp::Eq, "t".to_string()));
    let where_sql = Location::<sqlx::Sqlite>::gen_where_sql(&location);
    assert_eq!(where_sql, "user_name=?");

    let location = LocationSpec002::Select(Expr::new(Cmp::GreaterThan, "t".to_string()));
    let where_sql = Location::<sqlx::Sqlite>::gen_where_sql(&location);
    assert_eq!(where_sql, "\"select\">?");

    let location = LocationSpec002::Insert(Expr::new(Cmp::GreaterThan, "t".to_string()));
    let where_sql = Location::<sqlx::Sqlite>::gen_where_sql(&location);
    assert_eq!(where_sql, "c_name>?");
}
