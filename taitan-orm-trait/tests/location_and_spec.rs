use sqlx::{Arguments, Database, MySql};
use std::borrow::Cow;
use sqlx::mysql::MySqlArguments;
use taitan_orm_trait::result::Result;
use taitan_orm_trait::op::{Cmp, Expr};
use taitan_orm_trait::traits::Location;
use time::macros::datetime;
use time::PrimitiveDateTime;

use taitan_orm_trait::logic::And;
use taitan_orm_trait::traits::Parameter;

#[derive(Debug)]
struct UserLocation {
    name: Option<Expr<String>>,
    created: Option<Expr<PrimitiveDateTime>>,
}

impl Parameter<MySql> for UserLocation {
    fn add_to_args<'a, 'b>(&'a self, args: &'b mut <MySql as Database>::Arguments<'a>) -> Result<()> {
        if let Some(name) =  &self.name {
            args.add(&name.val)?;
        }
        if let Some(created) = &self.created {
            args.add(&created.val)?;
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
        name: Some(Expr::new(Cmp::Eq, "Allen".to_string() ) ),
        created: Some(Expr::new(Cmp::Eq,datetime!(2019-01-01 0:00)))
    };
    let location_where_sql = user.gen_where_sql();
    assert_eq!(location_where_sql, "name = ? AND created = ? ");

    let mut args = MySqlArguments::default();
    let location_where_args = user.add_to_args(&mut args).unwrap();
    assert_eq!(args.len(), 2);

    let location_left = UserLocation {
        name: Some(Expr::new(Cmp::Eq, "Allen".to_string() ) ),
        created: None
    };

    let location_right = UserLocation {
        name: None,
        created: Some(Expr::new(Cmp::Eq,datetime!(2019-01-01 0:00)))
    };

    let location_and = And::new(location_left, location_right);
    let location_where_sql = location_and.gen_where_sql();
    assert_eq!(location_where_sql, "(name = ?  AND created = ? )");

    let mut args = MySqlArguments::default();
    let location_where_args = user.add_to_args(&mut args).unwrap();
    assert_eq!(args.len(), 2);
}
