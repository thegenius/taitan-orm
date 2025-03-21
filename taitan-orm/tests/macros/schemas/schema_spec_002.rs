//
// use taitan_orm_macro::Schema;
// use time::macros::datetime;
// use time::PrimitiveDateTime;
//
// #[derive(Debug)]
// // #[derive(Debug, Schema, Clone)]
// // #[table = "user"]
// // #[unique = "name"]
// // #[index(name = "idx_hello", fields("age", "birthday"))]
// // #[primary(id)]
// pub struct Spec002 {
//     id: i32,
//     name: String,
//     age: Option<i32>,
//     birthday: Option<PrimitiveDateTime>,
// }
//
// impl taitan_orm::traits::Parameter<sqlx::Sqlite> for Spec002 {
//     fn add_to_args<'a, 'b>(
//         &'a self,
//         args: &'b mut <sqlx::Sqlite as sqlx::Database>::Arguments<'a>,
//     ) -> taitan_orm::result::Result<()> {
//         sqlx::Arguments::add(args, &self.id)?;
//         sqlx::Arguments::add(args, &self.name)?;
//         if let Some(f) = &self.age {
//             sqlx::Arguments::add(args, f)?
//         };
//         if let Some(f) = &self.birthday {
//             sqlx::Arguments::add(args, f)?
//         };
//         Ok(())
//     }
// }
// impl taitan_orm::traits::Parameter<sqlx::MySql> for Spec002 {
//     fn add_to_args<'a, 'b>(
//         &'a self,
//         args: &'b mut <sqlx::MySql as sqlx::Database>::Arguments<'a>,
//     ) -> taitan_orm::result::Result<()> {
//         sqlx::Arguments::add(args, &self.id)?;
//         sqlx::Arguments::add(args, &self.name)?;
//         if let Some(f) = &self.age {
//             sqlx::Arguments::add(args, f)?
//         };
//         if let Some(f) = &self.birthday {
//             sqlx::Arguments::add(args, f)?
//         };
//         Ok(())
//     }
// }
// impl taitan_orm::traits::Parameter<sqlx::Postgres> for Spec002 {
//     fn add_to_args<'a, 'b>(
//         &'a self,
//         args: &'b mut <sqlx::Postgres as sqlx::Database>::Arguments<'a>,
//     ) -> taitan_orm::result::Result<()> {
//         sqlx::Arguments::add(args, &self.id)?;
//         sqlx::Arguments::add(args, &self.name)?;
//         if let Some(f) = &self.age {
//             sqlx::Arguments::add(args, f)?
//         };
//         if let Some(f) = &self.birthday {
//             sqlx::Arguments::add(args, f)?
//         };
//         Ok(())
//     }
// }
// impl taitan_orm::traits::Entity<sqlx::Sqlite> for Spec002 {
//     fn gen_insert_sql<'a>(&self) -> std::borrow::Cow<'a, str> {
//         let fields = {
//             let mut s = String::default();
//             let mut has_prev = false;
//             s.push_str("id,name");
//             has_prev = true;
//             has_prev = true;
//             if !self.age.is_none() {
//                 s.push_str(",age");
//             }
//             if !self.birthday.is_none() {
//                 s.push_str(",birthday");
//             }
//             s
//         };
//         let marks = {
//             let mut s = String::default();
//             let mut has_prev = false;
//             s.push_str("?,?");
//             has_prev = true;
//             has_prev = true;
//             if !self.age.is_none() {
//                 s.push_str(",?");
//             }
//             if !self.birthday.is_none() {
//                 s.push_str(",?");
//             }
//             s
//         };
//         std::borrow::Cow::Owned(format!("INSERT INTO user ({}) VALUES({})", fields, marks))
//     }
//     fn gen_upsert_sql<'a>(&self) -> std::borrow::Cow<'a, str> {
//         let fields = {
//             let mut s = String::default();
//             let mut has_prev = false;
//             s.push_str("id,name");
//             has_prev = true;
//             has_prev = true;
//             if !self.age.is_none() {
//                 s.push_str(",age");
//             }
//             if !self.birthday.is_none() {
//                 s.push_str(",birthday");
//             }
//             s
//         };
//         let marks = {
//             let mut s = String::default();
//             let mut has_prev = false;
//             s.push_str("?,?");
//             has_prev = true;
//             has_prev = true;
//             if !self.age.is_none() {
//                 s.push_str(",?");
//             }
//             if !self.birthday.is_none() {
//                 s.push_str(",?");
//             }
//             s
//         };
//         let primarys = {
//             let mut s = String::default();
//             let mut has_prev = false;
//             s.push_str("id");
//             has_prev = true;
//             has_prev = true;
//             s
//         };
//         let upsert_sets = {
//             let mut s = String::default();
//             let mut has_prev = false;
//             s.push_str("name=EXCLUDED.name");
//             has_prev = true;
//             has_prev = true;
//             if !self.age.is_none() {
//                 s.push_str(",age=EXCLUDED.age");
//             }
//             if !self.birthday.is_none() {
//                 s.push_str(",birthday=EXCLUDED.birthday");
//             }
//             s
//         };
//         std::borrow::Cow::Owned(format!(
//             "INSERT INTO user ({}) VALUES({}) ON CONFLICT ({}) DO UPDATE SET {}",
//             fields, marks, primarys, upsert_sets
//         ))
//     }
//     fn gen_create_sql<'a>(&self) -> std::borrow::Cow<'a, str> {
//         todo!()
//     }
// }
// impl taitan_orm::traits::Entity<sqlx::MySql> for Spec002 {
//     fn gen_insert_sql<'a>(&self) -> std::borrow::Cow<'a, str> {
//         let fields = {
//             let mut s = String::default();
//             let mut has_prev = false;
//             s.push_str("id,name");
//             has_prev = true;
//             has_prev = true;
//             if !self.age.is_none() {
//                 s.push_str(",age");
//             }
//             if !self.birthday.is_none() {
//                 s.push_str(",birthday");
//             }
//             s
//         };
//         let marks = {
//             let mut s = String::default();
//             let mut has_prev = false;
//             s.push_str("?,?");
//             has_prev = true;
//             has_prev = true;
//             if !self.age.is_none() {
//                 s.push_str(",?");
//             }
//             if !self.birthday.is_none() {
//                 s.push_str(",?");
//             }
//             s
//         };
//         std::borrow::Cow::Owned(format!("INSERT INTO user ({}) VALUES({})", fields, marks))
//     }
//     fn gen_upsert_sql<'a>(&self) -> std::borrow::Cow<'a, str> {
//         let fields = {
//             let mut s = String::default();
//             let mut has_prev = false;
//             s.push_str("id,name");
//             has_prev = true;
//             has_prev = true;
//             if !self.age.is_none() {
//                 s.push_str(",age");
//             }
//             if !self.birthday.is_none() {
//                 s.push_str(",birthday");
//             }
//             s
//         };
//         let marks = {
//             let mut s = String::default();
//             let mut has_prev = false;
//             s.push_str("?,?");
//             has_prev = true;
//             has_prev = true;
//             if !self.age.is_none() {
//                 s.push_str(",?");
//             }
//             if !self.birthday.is_none() {
//                 s.push_str(",?");
//             }
//             s
//         };
//         let upsert_sets = {
//             let mut s = String::default();
//             let mut has_prev = false;
//             s.push_str("name=VALUES(name)");
//             has_prev = true;
//             has_prev = true;
//             if !self.age.is_none() {
//                 s.push_str(",age=VALUES(age)");
//             }
//             if !self.birthday.is_none() {
//                 s.push_str(",birthday=VALUES(birthday)");
//             }
//             s
//         };
//         std::borrow::Cow::Owned(format!(
//             "INSERT INTO user ({}) VALUES({}) ON DUPLICATE KEY UPDATE {}",
//             fields, marks, upsert_sets
//         ))
//     }
//     fn gen_create_sql<'a>(&self) -> std::borrow::Cow<'a, str> {
//         todo!()
//     }
// }
// impl taitan_orm::traits::Entity<sqlx::Postgres> for Spec002 {
//     fn gen_insert_sql<'a>(&self) -> std::borrow::Cow<'a, str> {
//         let fields = {
//             let mut s = String::default();
//             let mut has_prev = false;
//             s.push_str("id,name");
//             has_prev = true;
//             has_prev = true;
//             if !self.age.is_none() {
//                 s.push_str(",age");
//             }
//             if !self.birthday.is_none() {
//                 s.push_str(",birthday");
//             }
//             s
//         };
//         let marks = {
//             let mut s = String::default();
//             let mut has_prev = false;
//             let mut index = 0;
//             s.push_str("$1,$2");
//             has_prev = true;
//             has_prev = true;
//             index += 2usize;
//             if !self.age.is_none() {
//                 {
//                     index += 1;
//                     s.push_str(format!(",${}", index).as_ref())
//                 }
//             }
//             if !self.birthday.is_none() {
//                 {
//                     index += 1;
//                     s.push_str(format!(",${}", index).as_ref())
//                 }
//             }
//             s
//         };
//         std::borrow::Cow::Owned(format!(
//             "INSERT INTO \"user\" ({}) VALUES({})",
//             fields, marks
//         ))
//     }
//     fn gen_upsert_sql<'a>(&self) -> std::borrow::Cow<'a, str> {
//         let fields = {
//             let mut s = String::default();
//             let mut has_prev = false;
//             s.push_str("id,name");
//             has_prev = true;
//             has_prev = true;
//             if !self.age.is_none() {
//                 s.push_str(",age");
//             }
//             if !self.birthday.is_none() {
//                 s.push_str(",birthday");
//             }
//             s
//         };
//         let marks = {
//             let mut s = String::default();
//             let mut has_prev = false;
//             let mut index = 0;
//             s.push_str("$1,$2");
//             has_prev = true;
//             has_prev = true;
//             index += 2usize;
//             if !self.age.is_none() {
//                 {
//                     index += 1;
//                     s.push_str(format!(",${}", index).as_ref())
//                 }
//             }
//             if !self.birthday.is_none() {
//                 {
//                     index += 1;
//                     s.push_str(format!(",${}", index).as_ref())
//                 }
//             }
//             s
//         };
//         let primarys = {
//             let mut s = String::default();
//             let mut has_prev = false;
//             s.push_str("id");
//             has_prev = true;
//             has_prev = true;
//             s
//         };
//         let upsert_sets = {
//             let mut s = String::default();
//             let mut has_prev = false;
//             s.push_str("name=EXCLUDED.name");
//             has_prev = true;
//             has_prev = true;
//             if !self.age.is_none() {
//                 s.push_str(",age=EXCLUDED.age");
//             }
//             if !self.birthday.is_none() {
//                 s.push_str(",birthday=EXCLUDED.birthday");
//             }
//             s
//         };
//         std::borrow::Cow::Owned(format!(
//             "INSERT INTO \"user\" ({}) VALUES({}) ON CONFLICT ({}) DO UPDATE SET {}",
//             fields, marks, primarys, upsert_sets
//         ))
//     }
//     fn gen_create_sql<'a>(&self) -> std::borrow::Cow<'a, str> {
//         todo!()
//     }
// }
// #[derive(
//     Debug,
//     Clone,
//     taitan_orm :: macros :: Parameter,
//     taitan_orm :: macros
// :: Location,
//     serde :: Serialize,
//     serde :: Deserialize,
// )]
// pub struct Spec002Primary {
//     id: i32,
// }
// impl taitan_orm::traits::Unique<sqlx::Sqlite> for Spec002Primary {
//     type Mutation = Spec002Mutation;
// }
// impl taitan_orm::traits::Unique<sqlx::MySql> for Spec002Primary {
//     type Mutation = Spec002Mutation;
// }
// impl taitan_orm::traits::Unique<sqlx::Postgres> for Spec002Primary {
//     type Mutation = Spec002Mutation;
// }
// #[derive(
//     Debug,
//     Clone,
//     taitan_orm :: macros :: Parameter,
//     taitan_orm :: macros
// :: Location,
//     serde :: Serialize,
//     serde :: Deserialize,
// )]
// pub struct Spec002UniqueUnique {
//     name: String,
// }
// impl taitan_orm::traits::Unique<sqlx::Sqlite> for Spec002UniqueUnique {
//     type Mutation = Spec002Mutation;
// }
// impl taitan_orm::traits::Unique<sqlx::MySql> for Spec002UniqueUnique {
//     type Mutation = Spec002Mutation;
// }
// impl taitan_orm::traits::Unique<sqlx::Postgres> for Spec002UniqueUnique {
//     type Mutation = Spec002Mutation;
// }
// #[derive(
//     Debug,
//     Clone,
//     taitan_orm :: macros :: Parameter,
//     taitan_orm :: macros :: Location,
//     serde :: Serialize,
//     serde :: Deserialize,
// )]
// pub enum Spec002IndexName {}
// #[derive(
//     Debug,
//     Clone,
//     taitan_orm :: macros :: Parameter,
//     taitan_orm :: macros
// :: Location,
//     serde :: Serialize,
//     serde :: Deserialize,
// )]
// pub enum Spec002IndexFields {}
// #[derive(
//     Clone,
//     Debug,
//     taitan_orm :: macros :: Parameter,
//     taitan_orm :: macros
// :: Mutation,
//     serde :: Serialize,
//     serde :: Deserialize,
// )]
// pub struct Spec002Mutation {
//     name: std::option::Option<std::option::Option<String>>,
//     age: std::option::Option<std::option::Option<i32>>,
//     birthday: std::option::Option<std::option::Option<PrimitiveDateTime>>,
// }
// #[derive(
//     Clone,
//     Debug,
//     taitan_orm :: macros :: Parameter,
//     taitan_orm :: macros
// :: Location,
//     serde :: Serialize,
//     serde :: Deserialize,
// )]
// pub enum Spec002Location {
//     Name(taitan_orm::op::Expr<String>),
//     Age(taitan_orm::op::Expr<i32>),
//     Birthday(taitan_orm::op::Expr<PrimitiveDateTime>),
// }
// #[derive(
//     Clone, Debug, taitan_orm :: macros :: Selected, serde :: Serialize, serde :: Deserialize,
// )]
// pub struct Spec002Selected {
//     id: std::option::Option<std::option::Option<i32>>,
//     name: std::option::Option<std::option::Option<String>>,
//     age: std::option::Option<std::option::Option<i32>>,
//     birthday: std::option::Option<std::option::Option<PrimitiveDateTime>>,
// }
//
// #[test]
// pub fn test() {
//     let spec_001 = Spec002 {
//         id: 32,
//         name: "hello".to_string(),
//         age: Some(23),
//         birthday: Some(datetime!(2019-01-01 0:00)),
//     };
//     let spec_primary = Spec002Primary { id: 32 };
//
//     let selected = Spec002Selected {
//         id: Some(None),
//         name: Some(None),
//         age: Some(None),
//         birthday: Some(None),
//     };
//     let default_selected = Spec002Selected::default();
// }
