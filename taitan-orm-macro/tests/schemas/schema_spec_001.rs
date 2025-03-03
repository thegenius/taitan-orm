use serde::Deserialize;
use taitan_orm_macro::MutationNew;
use taitan_orm_macro::Parameter;
use taitan_orm_macro::SchemaNew;
use taitan_orm_trait::brave_new::Unique;
#[derive(SchemaNew)]
#[primary(a)]
#[unique(uk_01 = (a, b))]
#[index(
   idx_01 = (a, b, c),
   idx_02 = (a, b, c,d),
)]
#[derive(Debug)]
struct SchemaSpec001 {
    a: String,
    b: i64,
    c: bool,
    d: i64,
}
//
// impl taitan_orm_trait::brave_new::param::Parameter<sqlx::Sqlite> for SchemaSpec001 {
//     fn add_to_args<'a, 'b>(
//         &'a self,
//         args: &'b mut <sqlx::Sqlite as sqlx::Database>::Arguments<'a>,
//     ) -> taitan_orm_trait::brave_new::result::Result<()> {
//         sqlx::Arguments::add(args, &self.a)?;
//         sqlx::Arguments::add(args, &self.b)?;
//         sqlx::Arguments::add(args, &self.c)?;
//         sqlx::Arguments::add(args, &self.d)?;
//         Ok(())
//     }
// }
// impl taitan_orm_trait::brave_new::param::Parameter<sqlx::MySql> for SchemaSpec001 {
//     fn add_to_args<'a, 'b>(
//         &'a self,
//         args: &'b mut <sqlx::MySql as sqlx::Database>::Arguments<'a>,
//     ) -> taitan_orm_trait::brave_new::result::Result<()> {
//         sqlx::Arguments::add(args, &self.a)?;
//         sqlx::Arguments::add(args, &self.b)?;
//         sqlx::Arguments::add(args, &self.c)?;
//         sqlx::Arguments::add(args, &self.d)?;
//         Ok(())
//     }
// }
// impl taitan_orm_trait::brave_new::param::Parameter<sqlx::Postgres> for SchemaSpec001 {
//     fn add_to_args<'a, 'b>(
//         &'a self,
//         args: &'b mut <sqlx::Postgres as sqlx::Database>::Arguments<'a>,
//     ) -> taitan_orm_trait::brave_new::result::Result<()> {
//         sqlx::Arguments::add(args, &self.a)?;
//         sqlx::Arguments::add(args, &self.b)?;
//         sqlx::Arguments::add(args, &self.c)?;
//         sqlx::Arguments::add(args, &self.d)?;
//         Ok(())
//     }
// }
// impl taitan_orm_trait::brave_new::Entity<sqlx::Sqlite> for SchemaSpec001 {
//     fn gen_insert_sql<'a>(&self) -> std::borrow::Cow<'a, str> {
//         let fields = {
//             let mut s = String::default();
//             let mut has_prev = false;
//             s.push_str("a,b,c,d");
//             has_prev = true;
//             has_prev = true;
//             s
//         };
//         let marks = {
//             let mut s = String::default();
//             let mut has_prev = false;
//             s.push_str("?,?,?,?");
//             has_prev = true;
//             has_prev = true;
//             s
//         };
//         std::borrow::Cow::Owned(format!(
//             "INSERT INTO schema_spec001 ({}) VALUES({})",
//             fields, marks
//         ))
//     }
//     fn gen_upsert_sql<'a>(&self) -> std::borrow::Cow<'a, str> {
//         let fields = {
//             let mut s = String::default();
//             let mut has_prev = false;
//             s.push_str("a,b,c,d");
//             has_prev = true;
//             has_prev = true;
//             s
//         };
//         let marks = {
//             let mut s = String::default();
//             let mut has_prev = false;
//             s.push_str("?,?,?,?");
//             has_prev = true;
//             has_prev = true;
//             s
//         };
//         let primarys = {
//             let mut s = String::default();
//             let mut has_prev = false;
//             s.push_str("a");
//             has_prev = true;
//             has_prev = true;
//             s
//         };
//         let upsert_sets = {
//             let mut s = String::default();
//             let mut has_prev = false;
//             s.push_str("b=EXCLUDED.b,c=EXCLUDED.c,d=EXCLUDED.d");
//             has_prev = true;
//             has_prev = true;
//             s
//         };
//         std::borrow::Cow::Owned(format!(
//             "INSERT INTO schema_spec001 ({}) VALUES({}) ON CONFLICT ({}) DO UPDATE SET {}",
//             fields, marks, primarys, upsert_sets
//         ))
//     }
//     fn gen_create_sql<'a>(&self) -> std::borrow::Cow<'a, str> {
//         todo!()
//     }
// }
// impl taitan_orm_trait::brave_new::Entity<sqlx::MySql> for SchemaSpec001 {
//     fn gen_insert_sql<'a>(&self) -> std::borrow::Cow<'a, str> {
//         let fields = {
//             let mut s = String::default();
//             let mut has_prev = false;
//             s.push_str("a,b,c,d");
//             has_prev = true;
//             has_prev = true;
//             s
//         };
//         let marks = {
//             let mut s = String::default();
//             let mut has_prev = false;
//             s.push_str("?,?,?,?");
//             has_prev = true;
//             has_prev = true;
//             s
//         };
//         std::borrow::Cow::Owned(format!(
//             "INSERT INTO schema_spec001 ({}) VALUES({})",
//             fields, marks
//         ))
//     }
//     fn gen_upsert_sql<'a>(&self) -> std::borrow::Cow<'a, str> {
//         let fields = {
//             let mut s = String::default();
//             let mut has_prev = false;
//             s.push_str("a,b,c,d");
//             has_prev = true;
//             has_prev = true;
//             s
//         };
//         let marks = {
//             let mut s = String::default();
//             let mut has_prev = false;
//             s.push_str("?,?,?,?");
//             has_prev = true;
//             has_prev = true;
//             s
//         };
//         let upsert_sets = {
//             let mut s = String::default();
//             let mut has_prev = false;
//             s.push_str("b=VALUES(b),c=VALUES(c),d=VALUES(d)");
//             has_prev = true;
//             has_prev = true;
//             s
//         };
//         std::borrow::Cow::Owned(format!(
//             "INSERT INTO schema_spec001 ({}) VALUES({}) ON DUPLICATE KEY UPDATE {}",
//             fields, marks, upsert_sets
//         ))
//     }
//     fn gen_create_sql<'a>(&self) -> std::borrow::Cow<'a, str> {
//         todo!()
//     }
// }
// impl taitan_orm_trait::brave_new::Entity<sqlx::Postgres> for SchemaSpec001 {
//     fn gen_insert_sql<'a>(&self) -> std::borrow::Cow<'a, str> {
//         let fields = {
//             let mut s = String::default();
//             let mut has_prev = false;
//             s.push_str("a,b,c,d");
//             has_prev = true;
//             has_prev = true;
//             s
//         };
//         let marks = {
//             let mut s = String::default();
//             let mut has_prev = false;
//             let mut index = 0;
//             s.push_str("$1,$2,$3,$4");
//             has_prev = true;
//             has_prev = true;
//             index += 4usize;
//             s
//         };
//         std::borrow::Cow::Owned(format!(
//             "INSERT INTO schema_spec001 ({}) VALUES({})",
//             fields, marks
//         ))
//     }
//     fn gen_upsert_sql<'a>(&self) -> std::borrow::Cow<'a, str> {
//         let fields = {
//             let mut s = String::default();
//             let mut has_prev = false;
//             s.push_str("a,b,c,d");
//             has_prev = true;
//             has_prev = true;
//             s
//         };
//         let marks = {
//             let mut s = String::default();
//             let mut has_prev = false;
//             let mut index = 0;
//             s.push_str("$1,$2,$3,$4");
//             has_prev = true;
//             has_prev = true;
//             index += 4usize;
//             s
//         };
//         let primarys = {
//             let mut s = String::default();
//             let mut has_prev = false;
//             s.push_str("a");
//             has_prev = true;
//             has_prev = true;
//             s
//         };
//         let upsert_sets = {
//             let mut s = String::default();
//             let mut has_prev = false;
//             s.push_str("b=EXCLUDED.b,c=EXCLUDED.c,d=EXCLUDED.d");
//             has_prev = true;
//             has_prev = true;
//             s
//         };
//         std::borrow::Cow::Owned(format!(
//             "INSERT INTO schema_spec001 ({}) VALUES({}) ON CONFLICT ({}) DO UPDATE SET {}",
//             fields, marks, primarys, upsert_sets
//         ))
//     }
//     fn gen_create_sql<'a>(&self) -> std::borrow::Cow<'a, str> {
//         todo!()
//     }
// }
// #[derive(
//     Debug,
//     taitan_orm_macro :: Parameter,
//     taitan_orm_macro ::
// LocationNew,
// )]
// pub struct SchemaSpec001Primary {
//     a: String,
// }
// impl Unique<sqlx::Sqlite> for SchemaSpec001Primary {
//     type Mutation = SchemaSpec001Mutation;
// }
// impl Unique<sqlx::MySql> for SchemaSpec001Primary {
//     type Mutation = SchemaSpec001Mutation;
// }
// impl Unique<sqlx::Postgres> for SchemaSpec001Primary {
//     type Mutation = SchemaSpec001Mutation;
// }
// #[derive(
//     Debug,
//     taitan_orm_macro :: Parameter,
//     taitan_orm_macro ::
// LocationNew,
// )]
// pub struct SchemaSpec001UniqueUk01 {
//     a: String,
//     b: i64,
// }
// impl Unique<sqlx::Sqlite> for SchemaSpec001UniqueUk01 {
//     type Mutation = SchemaSpec001Mutation;
// }
// impl Unique<sqlx::MySql> for SchemaSpec001UniqueUk01 {
//     type Mutation = SchemaSpec001Mutation;
// }
// impl Unique<sqlx::Postgres> for SchemaSpec001UniqueUk01 {
//     type Mutation = SchemaSpec001Mutation;
// }
// #[derive(
//     Debug,
//     taitan_orm_macro :: Parameter,
//     taitan_orm_macro ::
// LocationNew,
// )]
// pub enum SchemaSpec001IndexIdx01 {
//     A { a: String },
//     AB { a: String, b: i64 },
//     ABC { a: String, b: i64, c: bool },
// }
// #[derive(
//     Debug,
//     taitan_orm_macro :: Parameter,
//     taitan_orm_macro ::
// LocationNew,
// )]
// pub enum SchemaSpec001IndexIdx02 {
//     A { a: String },
//     AB { a: String, b: i64 },
//     ABC { a: String, b: i64, c: bool },
//     ABCD { a: String, b: i64, c: bool, d: i64 },
// }
// #[derive(
//     Clone,
//     Debug,
//     taitan_orm_macro :: Parameter,
//     taitan_orm_macro ::
// MutationNew,
// )]
// pub struct SchemaSpec001Mutation {
//     b: std::option::Option<std::option::Option<i64>>,
//     c: std::option::Option<std::option::Option<bool>>,
//     d: std::option::Option<std::option::Option<i64>>,
// }
// #[derive(
//     Clone,
//     Debug,
//     taitan_orm_macro :: Parameter,
//     taitan_orm_macro ::
// LocationNew,
// )]
// pub enum SchemaSpec001Location {
//     B(taitan_orm_trait::LocationExpr<i64>),
//     C(taitan_orm_trait::LocationExpr<bool>),
//     D(taitan_orm_trait::LocationExpr<i64>),
// }

#[test]
fn schema_spec_001() {
    let entity = SchemaSpec001 {
        a: "hello".to_string(),
        b: 23,
        c: false,
        d: 1,
    };
    let insert_sql = taitan_orm_trait::brave_new::Entity::<sqlx::Sqlite>::gen_insert_sql(&entity);
    assert_eq!(
        insert_sql,
        "INSERT INTO schema_spec001 (a,b,c,d) VALUES(?,?,?,?)"
    );
}
