use sqlx::{Arguments, Database, MySql};
use std::borrow::Cow;
use sqlx::mysql::MySqlArguments;
use taitan_orm_trait::result::Result;
use taitan_orm_trait::op::{Cmp, Expr};
use taitan_orm_trait::traits::Location;
use time::macros::datetime;
use time::PrimitiveDateTime;
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
        name: Some(Expr::new(Cmp::Eq, "Allen".to_string() ) ),
        created: Some(Expr::new(Cmp::Eq,datetime!(2019-01-01 0:00)))
    };
    let update_set_sql = user.gen_where_sql();
    let mut args = MySqlArguments::default();
    let update_set_args = user.add_to_args(&mut args).unwrap();
}
