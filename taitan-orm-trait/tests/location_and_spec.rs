use sqlx::{Arguments, Database, MySql};
use std::borrow::Cow;
use sqlx::mysql::MySqlArguments;
use taitan_orm_trait::brave_new::result::Result;
use taitan_orm_trait::{CmpOperator, LocationExpr, Optional};
use taitan_orm_trait::brave_new::location::Location;
use time::macros::datetime;
use time::PrimitiveDateTime;
use taitan_orm_trait::brave_new::error::wrap_encode;
use taitan_orm_trait::brave_new::location_logic::location_and::And;
use taitan_orm_trait::brave_new::param::Parameter;

#[derive(Debug)]
struct UserLocation {
    name: Optional<LocationExpr<String>>,
    created: Optional<LocationExpr<PrimitiveDateTime>>,
}

impl Parameter<MySql> for UserLocation {
    fn add_to_args<'a, 'b>(&'a self, args: &'b mut <MySql as Database>::Arguments<'a>) -> Result<()> {
        if let Optional::Some(name) =  &self.name {
            wrap_encode(args.add(&name.val))?;
        }
        if let Optional::Some(created) = &self.created {
            wrap_encode(args.add(&created.val))?;
        }
        Ok(())
    }
}

impl Location<MySql> for UserLocation {
    fn table_name(&self) -> Cow<'static, str> {
        Cow::Borrowed("user")
    }
    fn gen_where_sql<'a>(&self) -> Cow<'a, str> {
        let mut sql = String::default();
        let mut has_prev = false;
        if self.name.is_some() {
            sql.push_str("name = ? ");
            has_prev = true;
        }
        if self.created.is_some() {
            if has_prev {
                sql.push_str("AND ");
            }
            sql.push_str("created = ? ");
            has_prev = true;
        }
        Cow::from(sql)
    }

    // fn add_where_args<'a>(&'a self, args: &mut <MySql as Database>::Arguments<'a>) -> Result<()> {
    //     if let Optional::Some(name) =  &self.name {
    //         wrap_encode(args.add(&name.val))?;
    //     }
    //     if let Optional::Some(created) = &self.created {
    //         wrap_encode(args.add(&created.val))?;
    //     }
    //     Ok(())
    // }

    fn all_none(&self) -> bool {
        self.name.is_none() && self.created.is_none()
    }
}

#[test]
fn location_trait_spec() {
    let user = UserLocation {
        name: Optional::Some(LocationExpr::new(CmpOperator::Eq, "Allen".to_string() ) ),
        created: Optional::Some(LocationExpr::new(CmpOperator::Eq,datetime!(2019-01-01 0:00)))
    };
    let location_where_sql = user.gen_where_sql();
    assert_eq!(location_where_sql, "name = ? AND created = ? ");

    let mut args = MySqlArguments::default();
    let location_where_args = user.add_to_args(&mut args).unwrap();
    assert_eq!(args.len(), 2);

    let location_left = UserLocation {
        name: Optional::Some(LocationExpr::new(CmpOperator::Eq, "Allen".to_string() ) ),
        created: Optional::None
    };

    let location_right = UserLocation {
        name: Optional::None,
        created: Optional::Some(LocationExpr::new(CmpOperator::Eq,datetime!(2019-01-01 0:00)))
    };

    let location_and = And::new(location_left, location_right);
    let location_where_sql = location_and.gen_where_sql();
    assert_eq!(location_where_sql, "(name = ?  AND created = ? )");

    let mut args = MySqlArguments::default();
    let location_where_args = user.add_to_args(&mut args).unwrap();
    assert_eq!(args.len(), 2);
}
