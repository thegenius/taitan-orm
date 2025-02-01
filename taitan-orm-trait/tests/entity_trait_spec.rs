use std::borrow::Cow;
use sqlx::{Arguments, Database, Sqlite};
use time::PrimitiveDateTime;
use time::macros::datetime;
use taitan_orm_trait::brave_new::entity::Entity;
use taitan_orm_trait::brave_new::result::Result;

struct User {
    name: String,
    created: PrimitiveDateTime
}

impl Entity<Sqlite> for User {
    fn gen_insert_sql<'a>(&self) -> Cow<'a, str> {
        let sql = "insert into users (name, created) values (?, ?)";
        Cow::from(sql)
    }

    fn gen_upsert_sql<'a>(&self) -> Cow<'a, str> {
        let sql = "insert into users (name, created) values (?, ?) on conflict (name) do update set created = ?";
        Cow::from(sql)
    }

    fn gen_create_sql<'a>(&self) -> Cow<'a, str> {
        let sql = "insert into users (name, created) values (?, ?)";
        Cow::from(sql)
    }

    fn add_insert_args<'a>(&'a self, args: &mut <Sqlite as Database>::Arguments<'a>) ->Result<()> {
        args.add(&self.name)?;
        args.add(&self.created)?;
        Ok(())
    }

    fn add_upsert_args<'a>(&'a self, args: &mut <Sqlite as Database>::Arguments<'a>) ->Result<()> {
        args.add(&self.name)?;
        args.add(&self.created)?;
        args.add(&self.created)?;
        Ok(())
    }
}

#[test]
fn entity_trait_spec() {
    let user = User{name: "Allen".to_string(), created: datetime!(2019-01-01 0:00)};
    let insert_args = user.gen_insert_args().unwrap();
    let insert_args = user.gen_upsert_args().unwrap();
}