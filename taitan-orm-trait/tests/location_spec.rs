use sqlx::{Arguments, Database, MySql};
use std::borrow::Cow;
use sqlx::mysql::MySqlArguments;
use taitan_orm_trait::brave_new::result::Result;
use taitan_orm_trait::{CmpOperator, LocationExpr, Optional};
use taitan_orm_trait::brave_new::location::Location;
use time::macros::datetime;
use time::PrimitiveDateTime;
use taitan_orm_trait::brave_new::error::wrap_encode;
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
        let mut sql = String::from("WHERE ");
        if self.name.is_some() {
            sql.push_str(" name = ?");
        }
        if self.created.is_some() {
            sql.push_str(" created = ?");
        }
        Cow::from(sql)
    }


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
    let update_set_sql = user.gen_where_sql();
    let mut args = MySqlArguments::default();
    let update_set_args = user.add_to_args(&mut args).unwrap();
}
