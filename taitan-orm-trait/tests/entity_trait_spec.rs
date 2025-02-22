use std::borrow::Cow;
use sqlx::{Arguments, Database, FromRow, Sqlite};
use time::PrimitiveDateTime;
use time::macros::datetime;
use taitan_orm_trait::brave_new::entity::{Entity, SqliteEntity};
use taitan_orm_trait::brave_new::error::wrap_encode;
use taitan_orm_trait::brave_new::param::Parameter;
use taitan_orm_trait::brave_new::result::Result;


fn call_entity(entity: &dyn SqliteEntity) -> Cow<'_, str> {
    entity.gen_create_sql()
}

#[derive(Debug)]
struct User {
    name: String,
    created: PrimitiveDateTime
}

impl Parameter<Sqlite> for User {
    fn add_to_args<'a, 'b>(&'a self, args: &'b mut <Sqlite as Database>::Arguments<'a>) -> Result<()> {
        wrap_encode(args.add(&self.name))?;
        wrap_encode(args.add(&self.created))?;
        Ok(())
    }
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

    // fn add_insert_args<'a>(&'a self, args: &mut <Sqlite as Database>::Arguments<'a>) ->Result<()> {
    //     wrap_encode(args.add(&self.name))?;
    //     wrap_encode(args.add(&self.created))?;
    //     Ok(())
    // }
    //
    // fn add_upsert_args<'a>(&'a self, args: &mut <Sqlite as Database>::Arguments<'a>) ->Result<()> {
    //     wrap_encode(args.add(&self.name))?;
    //     wrap_encode(args.add(&self.created))?;
    //     wrap_encode(args.add(&self.created))?;
    //     Ok(())
    // }
}






#[test]
fn entity_trait_spec() {
    let user = User{name: "Allen".to_string(), created: datetime!(2019-01-01 0:00)};
    let insert_args = <User as Parameter<Sqlite>>::gen_args(&user).unwrap();
    call_entity(&user);
}