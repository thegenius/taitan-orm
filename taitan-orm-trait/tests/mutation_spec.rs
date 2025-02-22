use sqlx::{Arguments, Database, Postgres};
use std::borrow::Cow;
use sqlx::postgres::{PgArguments};
use taitan_orm_trait::brave_new::mutation::Mutation;
use taitan_orm_trait::brave_new::result::Result;
use taitan_orm_trait::Optional;
use time::macros::datetime;
use time::PrimitiveDateTime;
use taitan_orm_trait::brave_new::error::wrap_encode;
use taitan_orm_trait::brave_new::param::Parameter;

#[derive(Debug)]
struct UserMutation {
    name: Optional<String>,
    created: Optional<PrimitiveDateTime>,
}

impl Parameter<Postgres> for UserMutation {
    fn add_to_args<'a, 'b>(&'a self, args: &'b mut <Postgres as Database>::Arguments<'a>) -> Result<()> {
        if let Optional::Some(name) =  &self.name {
            wrap_encode(args.add(name))?;
        }
        if let Optional::Some(created) = &self.created {
            wrap_encode(args.add(created))?;
        }
        Ok(())
    }
}

impl Mutation<Postgres> for UserMutation {
    fn gen_update_set_sql<'a>(&self) -> Cow<'a, str> {
        let mut sql = String::from("UPDATE `user`");
        if self.name.is_some() {
            sql.push_str(" name = ?");
        }
        if self.created.is_some() {
            sql.push_str(" created = ?");
        }
        Cow::from(sql)
    }

    // fn add_update_set_args<'a>(&'a self, args: &mut <Postgres as Database>::Arguments<'a>) -> Result<()> {
    //     if let Optional::Some(name) =  &self.name {
    //         wrap_encode(args.add(name))?;
    //     }
    //     if let Optional::Some(created) = &self.created {
    //         wrap_encode(args.add(created))?;
    //     }
    //     Ok(())
    // }

    fn all_none(&self) -> bool {
        self.name.is_none() && self.created.is_none()
    }
}

#[test]
fn mutation_trait_spec() {
    let user = UserMutation {
        name: Optional::Some("Allen".to_string()),
        created: Optional::Some(datetime!(2019-01-01 0:00)),
    };
    let update_set_sql = user.gen_update_set_sql();
    let mut args = PgArguments::default();
    let update_set_args = user.add_to_args(&mut args).unwrap();
}
