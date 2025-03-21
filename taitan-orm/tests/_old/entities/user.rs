// use serde::{Deserialize, Serialize};
// use sqlx::error::BoxDynError;
// use sqlx::sqlite::SqliteArguments;
// use sqlx::{Arguments, ColumnIndex, Decode, Row, Type};
// use sqlx::{Database, Sqlite};
// use std::borrow::Cow;
// use std::error::Error;
// use taitan_orm::database::sqlite::SqliteDatabase;
// use taitan_orm::prelude::SqlExecutor;
// use taitan_orm::traits::{Entity, Location,  Mutation,  Selected, Unique};
// use taitan_orm::op::{Cmp, Expr};
// use taitan_orm::order::{OrderBy, validate_order_by};
// use time::PrimitiveDateTime;
// use uuid::Uuid;
//
// #[derive(Debug)]
// pub struct User {
//     pub id: i64,
//     pub request_id: Uuid,
//     pub name: String,
//     pub age: Option<i32>,
//     pub birthday: Option<PrimitiveDateTime>,
// }
//
// pub async fn prepare_user_table(db: &mut SqliteDatabase) -> taitan_orm::result::Result<()> {
//     let _result = db.execute_plain("DROP TABLE IF EXISTS `user`").await?;
//     let _ = db
//         .execute_plain(
//             "CREATE TABLE IF NOT EXISTS `user`\
//     (`id` BIGINT PRIMARY KEY, \
//     `request_id` blob,  \
//     `name` VARCHAR(64), \
//     `age` INT, \
//     `birthday` DATETIME)",
//         )
//         .await?;
//     Ok(())
// }
//
// impl Entity for User {
//     fn get_table_name(&self) -> &str {
//         "user"
//     }
//
//     fn get_insert_fields(&self) -> Vec<FieldName> {
//         let mut fields = Vec::new();
//         fields.push(FieldName::from_str("id", false));
//         fields.push(FieldName::from_str("request_id", false));
//         fields.push(FieldName::from_str("name", false));
//
//         if let Option::Some(_) = &self.age {
//             fields.push(FieldName::from_str("age", false));
//         }
//         if let Option::Some(_) = &self.birthday {
//             fields.push(FieldName::from_str("birthday", false));
//         }
//         fields
//     }
//
//     fn get_upsert_set_fields(&self) -> Vec<FieldName> {
//         let mut fields = Vec::new();
//         fields.push(FieldName::from_str("request_id", false));
//
//         fields.push(FieldName::from_str("name", false));
//         if let Option::Some(_) = &self.age {
//             fields.push(FieldName::from_str("age", false));
//         }
//         if let Option::Some(_) = &self.birthday {
//             fields.push(FieldName::from_str("birthday", false));
//         }
//         fields
//     }
//
//     fn get_auto_increment_field(&self) -> Option<&str> {
//         todo!()
//     }
//
//     fn set_auto_increment_field(&mut self, value: Option<i64>) -> bool {
//         todo!()
//     }
//
//     fn gen_insert_arguments_sqlite(&self) -> std::result::Result<SqliteArguments<'_>, BoxDynError> {
//         let mut args = SqliteArguments::default();
//         args.add(&self.id)?;
//         args.add(&self.request_id)?;
//         args.add(&self.name)?;
//         if let Option::Some(age) = &self.age {
//             args.add(age)?;
//         }
//         if let Option::Some(birthday) = &self.birthday {
//             args.add(birthday)?;
//         }
//         Ok(args)
//     }
//
//     fn gen_upsert_arguments_sqlite(&self) -> std::result::Result<SqliteArguments<'_>, BoxDynError> {
//         let mut args = SqliteArguments::default();
//         args.add(&self.id)?;
//
//         args.add(&self.request_id)?;
//         args.add(&self.name)?;
//         if let Option::Some(age) = &self.age {
//             args.add(age)?;
//         }
//         if let Option::Some(birthday) = &self.birthday {
//             args.add(birthday)?;
//         }
//
//         args.add(&self.request_id)?;
//         args.add(&self.name)?;
//         if let Option::Some(age) = &self.age {
//             args.add(age)?;
//         }
//         if let Option::Some(birthday) = &self.birthday {
//             args.add(birthday)?;
//         }
//         Ok(args)
//     }
// }
//
// #[derive(Debug)]
// pub struct UserPrimary {
//     pub id: i64,
// }
//
// impl Unique for UserPrimary {
//     type Mutation = UserMutation;
//     fn get_table_name(&self) -> &'static str {
//         "user"
//     }
//
//     fn get_unique_field_names(&self) -> &'static [&'static str] {
//         &["id"]
//     }
//
//     fn gen_update_arguments_sqlite<'a>(
//         &'a self,
//         mutation: &'a Self::Mutation,
//     ) -> Result<SqliteArguments<'a>, BoxDynError> {
//         let mut args = SqliteArguments::default();
//         if let Option::Some(request_id) = &mutation.request_id {
//             args.add(request_id)?;
//         }
//         if let Option::Some(name) = &mutation.name {
//             args.add(name)?;
//         }
//         if let Option::Some(age) = &mutation.age {
//             args.add(age)?;
//         }
//         if let Option::Some(birthday) = mutation.birthday {
//             args.add(birthday)?;
//         }
//         args.add(&self.id)?;
//         Ok(args)
//     }
//
//     fn gen_unique_arguments_sqlite(&self) -> std::result::Result<SqliteArguments<'_>, BoxDynError> {
//         let mut args = SqliteArguments::default();
//         args.add(&self.id)?;
//         Ok(args)
//     }
// }
//
// #[derive(Debug, Default, Serialize, Deserialize)]
// pub struct UserSelected {
//     pub id: Option<u64>,
//     pub request_id: Option<Uuid>,
//     pub name: Option<String>,
//     pub age: Option<i32>,
//     pub birthday: Option<PrimitiveDateTime>,
//     // money: Option<BigDecimal>,
//     // ipv4addr: Option<Ipv4Addr>,
//     // ipv6addr: Option<Ipv6Addr>,
// }
//
//
// impl Selection for UserSelected {
//     fn get_table_name(&self) -> &'static str {
//         "user"
//     }
//     fn get_selected_fields(&self) -> Vec<String> {
//         let mut fields = Vec::new();
//         if self.id.is_selected() {
//             fields.push("id".to_string());
//         };
//         if self.request_id.is_selected() {
//             fields.push("request_id".to_string());
//         };
//         if self.age.is_selected() {
//             fields.push("age".to_string());
//         };
//         if self.name.is_selected() {
//             fields.push("name".to_string());
//         };
//         if self.birthday.is_selected() {
//             fields.push("birthday".to_string());
//         };
//         return fields;
//     }
//
//     fn full_fields() -> Self
//     where
//         Self: Sized,
//     {
//         Self {
//             id: taitan_orm::result::Option::Null,
//             request_id: taitan_orm::result::Option::Null,
//             age: taitan_orm::result::Option::Null,
//             name: taitan_orm::result::Option::Null,
//             birthday: taitan_orm::result::Option::Null,
//         }
//     }
// }
//
//
// // impl SelectedEntityNew for UserSelected {
// //     type Selection = UserSelection;
// //
// //     fn from_row<DB: Database>(selection: &Self::Selection, row: DB::Row) -> Result<Self, sqlx::Error>
// //     where
// //         Self: Sized,
// //         for <'a> PrimitiveDateTime: Type<DB> + Decode<'a, DB>,
// //         for <'a> i32: Type<DB> + Decode<'a, DB>,
// //         for <'a>String: Type<DB> + Decode<'a, DB>,
// //         for <'a>Uuid: Type<DB> + Decode<'a, DB>,
// //         for <'a>u64: Type<DB> + Decode<'a, DB>,
// //         for <'a> &'a str: ColumnIndex<DB::Row>,
// //         usize: ColumnIndex<DB::Row>
// //     {
// //         let mut selected = Self::default();
// //         let mut i = 0;
// //         if selection.id {
// //             selected.id = row.try_get(i).ok().into();
// //             i += 1;
// //         }
// //         if selection.request_id {
// //             selected.request_id = row.try_get("request_id").ok().into();
// //         }
// //         if selection.name {
// //             selected.name = row.try_get("name").ok().into();
// //         }
// //         if selection.age {
// //             selected.age = row.try_get("age").ok().into();
// //         }
// //         if selection.birthday {
// //             selected.birthday = row.try_get("birthday").ok().into();
// //         }
// //         Ok(selected)
// //     }
// // }
//
// impl SelectedEntity<Sqlite> for UserSelected {
//
//
//     fn from_row(selection: &Self, row: <Sqlite as Database>::Row) -> Result<Self, sqlx::Error>
//     where
//         Self: Sized,
//     {
//         let mut selected = Self::default();
//         let mut i = 0;
//         if selection.id.is_selected() {
//             selected.id = sqlx::Row::try_get(&row, i).ok().into();
//             i += 1;
//         };
//         if selection.request_id.is_selected() {
//             selected.request_id = sqlx::Row::try_get(&row, i).ok().into();
//             i += 1;
//         };
//         if selection.age.is_selected() {
//             selected.age = sqlx::Row::try_get(&row, i).ok().into();
//             i += 1;
//         };
//         if selection.name.is_selected() {
//             selected.name = sqlx::Row::try_get(&row, i).ok().into();
//             i += 1;
//         };
//         if selection.birthday.is_selected() {
//             selected.birthday = sqlx::Row::try_get(&row, i).ok().into();
//             i += 1;
//         };
//         Ok(selected)
//     }
// }
//
// #[derive(Debug, Default)]
// pub struct UserSelection {
//     pub id: bool,
//     pub request_id: bool,
//     pub name: bool,
//     pub age: bool,
//     pub birthday: bool,
//     // money: bool,
//     // ipv4addr: bool,
//     // ipv6addr: bool,
// }
//
// impl Selection for UserSelection {
//     fn get_table_name(&self) -> &'static str {
//         "user"
//     }
//
//     fn get_selected_fields(&self) -> Vec<String> {
//         let mut fields = Vec::new();
//         if self.id {
//             fields.push("id".to_string());
//         }
//         if self.request_id {
//             fields.push("request_id".to_string());
//         }
//         if self.name {
//             fields.push("name".to_string());
//         }
//         if self.age {
//             fields.push("age".to_string());
//         }
//         if self.birthday {
//             fields.push("birthday".to_string());
//         }
//         fields
//     }
//
//     // fn get_selected_bits(&self) -> bit_vec::BitVec {
//     //     let mut fields = bit_vec::BitVec::new();
//     //     fields.push(self.id);
//     //     fields.push(self.request_id);
//     //     fields.push(self.name);
//     //     fields.push(self.age);
//     //     fields.push(self.birthday);
//     //     fields
//     // }
//
//
//     fn full_fields() -> Self
//     where
//         Self: Sized,
//     {
//         Self {
//             id: true,
//             request_id: true,
//             name: true,
//             age: true,
//             birthday: true,
//         }
//     }
// }
//
// #[derive(Debug)]
// pub struct UserMutation {
//     pub request_id: taitan_orm::result::Option<Uuid>,
//     pub name: taitan_orm::result::Option<String>,
//     pub age: taitan_orm::result::Option<i32>,
//     pub birthday: taitan_orm::result::Option<PrimitiveDateTime>,
//     // money: Option<BigDecimal>,
//     // ipv4addr: Option<Ipv4Addr>,
//     // ipv6addr: Option<Ipv6Addr>,
// }
//
// impl Mutation for UserMutation {
//     type Location = UserLocation;
//     fn get_mutation_fields_name(&self) -> Vec<FieldName> {
//         let mut fields = Vec::new();
//         if let Option::Some(_) = &self.request_id {
//             fields.push(FieldName::from_str("request_id", false));
//         }
//         if let Option::Some(_) = &self.name {
//             fields.push(FieldName::from_str("name", false));
//         }
//         if let Option::Some(_) = &self.age {
//             fields.push(FieldName::from_str("age", false));
//         }
//         if let Option::Some(_) = &self.birthday {
//             fields.push(FieldName::from_str("birthday", false));
//         }
//         // if let Some(_) = &self.money {
//         //     fields.push("money".to_string());
//         // }
//         // if let Some(_) = &self.ipv4addr {
//         //     fields.push("ipv4addr".to_string());
//         // }
//         // if let Some(_) = &self.ipv6addr {
//         //     fields.push("ipv6addr".to_string());
//         // }
//         fields
//     }
// }
//
// #[derive(Debug)]
// pub struct UserPrimaryMutationPair<'a>(pub &'a UserMutation, pub &'a UserPrimary);
//
// impl<'a> UpdateCommand for UserPrimaryMutationPair<'a> {
//     fn gen_update_arguments_sqlite(&self) -> Result<SqliteArguments<'_>, BoxDynError> {
//         let mut args = SqliteArguments::default();
//
//         if let Option::Some(request_id) = &self.0.request_id {
//             args.add(request_id)?;
//         }
//         if let Option::Some(name) = &self.0.name {
//             args.add(name)?;
//         }
//         if let Option::Some(age) = &self.0.age {
//             args.add(age)?;
//         }
//         if let Option::Some(birthday) = &self.0.birthday {
//             args.add(birthday)?;
//         }
//
//         args.add(&self.1.id)?;
//
//         Ok(args)
//     }
// }
//
// #[derive(Debug)]
// pub struct UserLocation {
//     pub request_id: Option<LocationExpr<Uuid>>,
//     pub name: Option<LocationExpr<String>>,
//     pub age: Option<LocationExpr<i32>>,
//     pub birthday: Option<LocationExpr<PrimitiveDateTime>>,
//     // money: LocationExpr<BigDecimal>,
//     // ipv4addr: LocationExpr<Ipv4Addr>,
//     // ipv6addr: LocationExpr<Ipv6Addr>,
//
// }
//
// impl UserLocation {
//     #[inline(always)]
//     pub fn concat_where_clause<T>(
//         &self,
//         sql: &mut String,
//         wrap_char: char,
//         place_holder: char,
//         field_name: &str,
//         loc: &LocationExpr<T>,
//     ) {
//         sql.push('`');
//         sql.push_str(field_name);
//         sql.push('`');
//         sql.push_str(loc.get_cmp_sql());
//         sql.push('?');
//     }
// }
//
// impl Location for UserLocation {
//     fn get_table_name(&self) -> &'static str {
//         "user"
//     }
//
//     // fn get_location_fields_name(&self) -> Vec<FieldName> {
//     //     let mut fields = Vec::new();
//     //     if let Option::Some(_) = &self.request_id {
//     //         fields.push(FieldName::from_str("request_id", false));
//     //     }
//     //     if let Option::Some(_) = &self.name {
//     //         fields.push(FieldName::from_str("name", false));
//     //     }
//     //     if let Option::Some(_) = &self.age {
//     //         fields.push(FieldName::from_str("age", false));
//     //     }
//     //     if let Option::Some(_) = &self.birthday {
//     //         fields.push(FieldName::from_str("birthday", false));
//     //     }
//     //     fields
//     // }
//
//     fn get_where_clause(&self) -> String {
//         let mut sql = String::default();
//         if let Option::Some(request_id) = &self.request_id {
//             sql.push('`');
//             sql.push_str("request_id");
//             sql.push('`');
//             sql.push_str(request_id.get_cmp_sql());
//             sql.push('?');
//         }
//         if let Option::Some(name) = &self.name {
//             sql.push('`');
//             sql.push_str("name");
//             sql.push('`');
//             sql.push_str(name.get_cmp_sql());
//             sql.push('?');
//         }
//         if let Option::Some(age) = &self.age {
//             sql.push('`');
//             sql.push_str("age");
//             sql.push('`');
//             sql.push_str(age.get_cmp_sql());
//             sql.push('?');
//         }
//         if let Option::Some(birthday) = &self.birthday {
//             sql.push('`');
//             sql.push_str("birthday");
//             sql.push('`');
//             sql.push_str(birthday.get_cmp_sql());
//             sql.push('?');
//         }
//         sql
//     }
//
//     fn gen_location_arguments_sqlite(&self) -> Result<SqliteArguments<'_>, BoxDynError> {
//         let mut args = SqliteArguments::default();
//
//         if let Option::Some(request_id) = &self.request_id {
//             args.add(&request_id.val)?;
//         }
//         if let Option::Some(name) = &self.name {
//             args.add(&name.val)?;
//         }
//         if let Option::Some(age) = &self.age {
//             args.add(&age.val)?;
//         }
//         if let Option::Some(birthday) = &self.birthday {
//             args.add(&birthday.val)?;
//         }
//
//         Ok(args)
//     }
//     //
//     // fn gen_change_arguments_sqlite<'a>(
//     //     &'a self,
//     //     mutation: &'a Self::Mutation,
//     // ) -> Result<SqliteArguments<'a>, BoxDynError> {
//     //     let mut args = SqliteArguments::default();
//     //
//     //     if let Some(request_id) = &mutation.request_id {
//     //         args.add(request_id)?;
//     //     }
//     //     if let Some(name) = &mutation.name {
//     //         args.add(name)?;
//     //     }
//     //     if let Some(age) = &mutation.age {
//     //         args.add(age)?;
//     //     }
//     //     if let Some(birthday) = &mutation.birthday {
//     //         args.add(birthday)?;
//     //     }
//     //
//     //     if let Some(request_id) = &self.request_id {
//     //         args.add(request_id.val)?;
//     //     }
//     //     if let Some(name) = &self.name {
//     //         args.add(name.clone().val)?;
//     //     }
//     //     if let Some(age) = &self.age {
//     //         args.add(age.val)?;
//     //     }
//     //     if let Some(birthday) = &self.birthday {
//     //         args.add(birthday.val)?;
//     //     }
//     //
//     //     Ok(args)
//     // }
// }
//
// #[derive(Debug)]
// pub struct UserLocationMutationPair(pub UserMutation, pub UserLocation);
//
// impl UpdateCommand for UserLocationMutationPair {
//     fn gen_update_arguments_sqlite(&self) -> Result<SqliteArguments<'_>, BoxDynError> {
//         let mut args = SqliteArguments::default();
//
//         if let Option::Some(request_id) = &self.0.request_id {
//             args.add(request_id)?;
//         }
//         if let Option::Some(name) = &self.0.name {
//             args.add(name)?;
//         }
//         if let Option::Some(age) = &self.0.age {
//             args.add(age)?;
//         }
//         if let Option::Some(birthday) = &self.0.birthday {
//             args.add(birthday)?;
//         }
//
//         if let Option::Some(request_id) = &self.1.request_id {
//             args.add(request_id.val)?;
//         }
//         if let Option::Some(name) = &self.1.name {
//             args.add(name.clone().val)?;
//         }
//         if let Option::Some(age) = &self.1.age {
//             args.add(age.val)?;
//         }
//         if let Option::Some(birthday) = &self.1.birthday {
//             args.add(birthday.val)?;
//         }
//
//         Ok(args)
//     }
// }
//
// #[derive(Debug, Default)]
// pub struct UserOrderBy<'a> {
//     fields: Vec<Cow<'a, str>>,
// }
//
// impl<'a> OrderBy for UserOrderBy<'a> {
//     fn unique_fields(&self) -> &[&[&str]] {
//         &[&["id"]]
//     }
//
//     fn all_fields(&self) -> &[&str] {
//         &["id", "name", "age", "birthday"]
//     }
//     fn get_fields(&self) -> &[Cow<'a, str>] {
//         &self.fields
//     }
// }
//
// impl<'a> UserOrderBy<'a> {
//     pub fn build<I, S>(fields: I) -> Result<Self, Box<dyn Error + 'static>>
//     where
//         I: IntoIterator<Item = S> + Clone,
//         S: AsRef<str> + Into<Cow<'a, str>>, // 确保每个元素可以转换为 Cow<'a, str>
//     {
//         let order_by = Self::default();
//         validate_order_by(
//             fields.clone(),
//             order_by.all_fields(),
//             order_by.unique_fields(),
//         )?;
//
//         Ok(Self {
//             fields: fields.into_iter().map(Into::into).collect(),
//         })
//     }
// }
