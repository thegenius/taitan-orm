//
// use time::PrimitiveDateTime;
// use uuid::Uuid;
//
// #[derive(Default, Debug, Clone)]
// // #[table_name = "user"]
// pub struct UserLocation {
//     mode: taitan_orm::prelude::LocationMode,
//     pub id: taitan_orm::result::Optional<taitan_orm::traits::LocationExpr<i64>>,
//     pub request_id: taitan_orm::result::Optional<taitan_orm::traits::LocationExpr<Uuid>>,
//     pub age: taitan_orm::result::Optional<taitan_orm::traits::LocationExpr<i32>>,
//     pub name: taitan_orm::result::Optional<taitan_orm::traits::LocationExpr<String>>,
//     pub birthday: taitan_orm::result::Optional<taitan_orm::traits::LocationExpr<PrimitiveDateTime>>,
// }
// impl taitan_orm::traits::Location for UserLocation {
//     fn get_table_name(&self) -> &'static str {
//         "user"
//     }
//     fn get_where_clause(&self) -> String {
//         let mut sql = String::default();
//         let connectives = self.mode.as_connective();
//         match &self.id {
//             taitan_orm::result::Optional::Some(id) => {
//                 sql.push('`');
//                 sql.push_str("id");
//                 sql.push('`');
//                 sql.push_str(id.cmp.get_sql());
//                 sql.push('?');
//                 sql.push_str(connectives);
//             }
//             taitan_orm::result::Optional::Null => {
//                 sql.push('`');
//                 sql.push_str("id");
//                 sql.push('`');
//                 sql.push_str(" IS NULL ");
//             }
//             _ => {}
//         }
//         match &self.request_id {
//             taitan_orm::result::Optional::Some(request_id) => {
//                 sql.push('`');
//                 sql.push_str("request_id");
//                 sql.push('`');
//                 sql.push_str(request_id.cmp.get_sql());
//                 sql.push('?');
//                 sql.push_str(connectives);
//             }
//             taitan_orm::result::Optional::Null => {
//                 sql.push('`');
//                 sql.push_str("request_id");
//                 sql.push('`');
//                 sql.push_str(" IS NULL ");
//             }
//             _ => {}
//         }
//         match &self.age {
//             taitan_orm::result::Optional::Some(age) => {
//                 sql.push('`');
//                 sql.push_str("age");
//                 sql.push('`');
//                 sql.push_str(age.cmp.get_sql());
//                 sql.push('?');
//                 sql.push_str(connectives);
//             }
//             taitan_orm::result::Optional::Null => {
//                 sql.push('`');
//                 sql.push_str("age");
//                 sql.push('`');
//                 sql.push_str(" IS NULL ");
//             }
//             _ => {}
//         }
//         match &self.name {
//             taitan_orm::result::Optional::Some(name) => {
//                 sql.push('`');
//                 sql.push_str("name");
//                 sql.push('`');
//                 sql.push_str(name.cmp.get_sql());
//                 sql.push('?');
//                 sql.push_str(connectives);
//             }
//             taitan_orm::result::Optional::Null => {
//                 sql.push('`');
//                 sql.push_str("name");
//                 sql.push('`');
//                 sql.push_str(" IS NULL ");
//             }
//             _ => {}
//         }
//         match &self.birthday {
//             taitan_orm::result::Optional::Some(birthday) => {
//                 sql.push('`');
//                 sql.push_str("birthday");
//                 sql.push('`');
//                 sql.push_str(birthday.cmp.get_sql());
//                 sql.push('?');
//                 sql.push_str(connectives);
//             }
//             taitan_orm::result::Optional::Null => {
//                 sql.push('`');
//                 sql.push_str("birthday");
//                 sql.push('`');
//                 sql.push_str(" IS NULL ");
//             }
//             _ => {}
//         }
//         return sql
//             .strip_suffix(connectives)
//             .unwrap_or(sql.as_str())
//             .to_string();
//     }
//     fn gen_location_arguments_sqlite(
//         &self,
//     ) -> Result<sqlx::sqlite::SqliteArguments<'_>, sqlx::error::BoxDynError> {
//         let mut args = sqlx::sqlite::SqliteArguments::default();
//         if let taitan_orm::result::Optional::Some(id) = &self.id {
//             sqlx::Arguments::add(&mut args, &id.val)?;
//         }
//         if let taitan_orm::result::Optional::Some(request_id) = &self.request_id {
//             sqlx::Arguments::add(&mut args, &request_id.val)?;
//         }
//         if let taitan_orm::result::Optional::Some(age) = &self.age {
//             sqlx::Arguments::add(&mut args, &age.val)?;
//         }
//         if let taitan_orm::result::Optional::Some(name) = &self.name {
//             sqlx::Arguments::add(&mut args, &name.val)?;
//         }
//         if let taitan_orm::result::Optional::Some(birthday) = &self.birthday {
//             sqlx::Arguments::add(&mut args, &birthday.val)?;
//         }
//         Ok(args)
//     }
//     fn gen_location_arguments_mysql(
//         &self,
//     ) -> Result<sqlx::mysql::MySqlArguments, sqlx::error::BoxDynError> {
//         let mut args = sqlx::mysql::MySqlArguments::default();
//         if let taitan_orm::result::Optional::Some(id) = &self.id {
//             sqlx::Arguments::add(&mut args, &id.val)?;
//         }
//         if let taitan_orm::result::Optional::Some(request_id) = &self.request_id {
//             sqlx::Arguments::add(&mut args, &request_id.val)?;
//         }
//         if let taitan_orm::result::Optional::Some(age) = &self.age {
//             sqlx::Arguments::add(&mut args, &age.val)?;
//         }
//         if let taitan_orm::result::Optional::Some(name) = &self.name {
//             sqlx::Arguments::add(&mut args, &name.val)?;
//         }
//         if let taitan_orm::result::Optional::Some(birthday) = &self.birthday {
//             sqlx::Arguments::add(&mut args, &birthday.val)?;
//         }
//         Ok(args)
//     }
//     fn gen_location_arguments_postgres(
//         &self,
//     ) -> Result<sqlx::postgres::PgArguments, sqlx::error::BoxDynError> {
//         let mut args = sqlx::postgres::PgArguments::default();
//         if let taitan_orm::result::Optional::Some(id) = &self.id {
//             sqlx::Arguments::add(&mut args, &id.val)?;
//         }
//         if let taitan_orm::result::Optional::Some(request_id) = &self.request_id {
//             sqlx::Arguments::add(&mut args, &request_id.val)?;
//         }
//         if let taitan_orm::result::Optional::Some(age) = &self.age {
//             sqlx::Arguments::add(&mut args, &age.val)?;
//         }
//         if let taitan_orm::result::Optional::Some(name) = &self.name {
//             sqlx::Arguments::add(&mut args, &name.val)?;
//         }
//         if let taitan_orm::result::Optional::Some(birthday) = &self.birthday {
//             sqlx::Arguments::add(&mut args, &birthday.val)?;
//         }
//         Ok(args)
//     }
// }
