use serde::{Deserialize, Serialize};
use sqlx::error::BoxDynError;
use sqlx::sqlite::SqliteArguments;
use sqlx::{Arguments, ColumnIndex, Decode, Row, Type};
use sqlx::{Database, Sqlite};
use std::borrow::Cow;
use std::error::Error;
use taitan_orm::database::sqlite::SqliteDatabase;
use taitan_orm::SqlExecutor;
use taitan_orm_trait::{validate_order_by, Entity, FieldName, Location, LocationExpr, LocationTrait, Mutation, Optional, OrderBy, SelectedEntity, SelectedEntityNew, Selection, Unique, UpdateCommand};
use time::PrimitiveDateTime;
use uuid::Uuid;

#[derive(Debug)]
pub struct User {
    pub id: i64,
    pub request_id: Uuid,
    pub name: String,
    pub age: Optional<i32>,
    pub birthday: Optional<PrimitiveDateTime>,
}

pub async fn prepare_user_table(db: &mut SqliteDatabase) -> taitan_orm::Result<()> {
    let _result = db.execute_plain("DROP TABLE IF EXISTS `user`").await?;
    let _ = db
        .execute_plain(
            "CREATE TABLE IF NOT EXISTS `user`\
    (`id` BIGINT PRIMARY KEY, \
    `request_id` blob,  \
    `name` VARCHAR(64), \
    `age` INT, \
    `birthday` DATETIME)",
        )
        .await?;
    Ok(())
}

impl Entity for User {
    fn get_table_name(&self) -> &str {
        "user"
    }

    fn get_insert_fields(&self) -> Vec<FieldName> {
        let mut fields = Vec::new();
        fields.push(FieldName::from_str("id", false));
        fields.push(FieldName::from_str("request_id", false));
        fields.push(FieldName::from_str("name", false));

        if let Optional::Some(_) = &self.age {
            fields.push(FieldName::from_str("age", false));
        }
        if let Optional::Some(_) = &self.birthday {
            fields.push(FieldName::from_str("birthday", false));
        }
        fields
    }

    fn get_upsert_set_fields(&self) -> Vec<FieldName> {
        let mut fields = Vec::new();
        fields.push(FieldName::from_str("request_id", false));

        fields.push(FieldName::from_str("name", false));
        if let Optional::Some(_) = &self.age {
            fields.push(FieldName::from_str("age", false));
        }
        if let Optional::Some(_) = &self.birthday {
            fields.push(FieldName::from_str("birthday", false));
        }
        fields
    }

    fn get_auto_increment_field(&self) -> Option<&str> {
        todo!()
    }

    fn set_auto_increment_field(&mut self, value: Option<i64>) -> bool {
        todo!()
    }

    fn gen_insert_arguments_sqlite(&self) -> std::result::Result<SqliteArguments<'_>, BoxDynError> {
        let mut args = SqliteArguments::default();
        args.add(&self.id)?;
        args.add(&self.request_id)?;
        args.add(&self.name)?;
        if let Optional::Some(age) = &self.age {
            args.add(age)?;
        }
        if let Optional::Some(birthday) = &self.birthday {
            args.add(birthday)?;
        }
        Ok(args)
    }

    fn gen_upsert_arguments_sqlite(&self) -> std::result::Result<SqliteArguments<'_>, BoxDynError> {
        let mut args = SqliteArguments::default();
        args.add(&self.id)?;

        args.add(&self.request_id)?;
        args.add(&self.name)?;
        if let Optional::Some(age) = &self.age {
            args.add(age)?;
        }
        if let Optional::Some(birthday) = &self.birthday {
            args.add(birthday)?;
        }

        args.add(&self.request_id)?;
        args.add(&self.name)?;
        if let Optional::Some(age) = &self.age {
            args.add(age)?;
        }
        if let Optional::Some(birthday) = &self.birthday {
            args.add(birthday)?;
        }
        Ok(args)
    }
}

#[derive(Debug)]
pub struct UserPrimary {
    pub id: i64,
}

impl Unique for UserPrimary {
    type Mutation = UserMutation;
    fn get_table_name(&self) -> &'static str {
        "user"
    }

    fn get_unique_field_names(&self) -> &'static [&'static str] {
        &["id"]
    }

    fn gen_update_arguments_sqlite<'a>(
        &'a self,
        mutation: &'a Self::Mutation,
    ) -> Result<SqliteArguments<'a>, BoxDynError> {
        let mut args = SqliteArguments::default();
        if let Optional::Some(request_id) = &mutation.request_id {
            args.add(request_id)?;
        }
        if let Optional::Some(name) = &mutation.name {
            args.add(name)?;
        }
        if let Optional::Some(age) = &mutation.age {
            args.add(age)?;
        }
        if let Optional::Some(birthday) = mutation.birthday {
            args.add(birthday)?;
        }
        args.add(&self.id)?;
        Ok(args)
    }

    fn gen_unique_arguments_sqlite(&self) -> std::result::Result<SqliteArguments<'_>, BoxDynError> {
        let mut args = SqliteArguments::default();
        args.add(&self.id)?;
        Ok(args)
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UserSelected {
    pub id: Optional<u64>,
    pub request_id: Optional<Uuid>,
    pub name: Optional<String>,
    pub age: Optional<i32>,
    pub birthday: Optional<PrimitiveDateTime>,
    // money: Option<BigDecimal>,
    // ipv4addr: Option<Ipv4Addr>,
    // ipv6addr: Option<Ipv6Addr>,
}

impl SelectedEntityNew for UserSelected {
    type Selection = UserSelection;

    fn from_row<DB: Database>(selection: &Self::Selection, row: DB::Row) -> Result<Self, sqlx::Error>
    where
        Self: Sized,
        for <'a> PrimitiveDateTime: Type<DB> + Decode<'a, DB>,
        for <'a> i32: Type<DB> + Decode<'a, DB>,
        for <'a>String: Type<DB> + Decode<'a, DB>,
        for <'a>Uuid: Type<DB> + Decode<'a, DB>,
        for <'a>u64: Type<DB> + Decode<'a, DB>,
        for <'a> &'a str: ColumnIndex<DB::Row>,
        usize: ColumnIndex<DB::Row>
    {
        let mut selected = Self::default();
        let mut i = 0;
        if selection.id {
            selected.id = row.try_get(i).ok().into();
            i += 1;
        }
        if selection.request_id {
            selected.request_id = row.try_get("request_id").ok().into();
        }
        if selection.name {
            selected.name = row.try_get("name").ok().into();
        }
        if selection.age {
            selected.age = row.try_get("age").ok().into();
        }
        if selection.birthday {
            selected.birthday = row.try_get("birthday").ok().into();
        }
        Ok(selected)
    }
}

impl SelectedEntity<Sqlite> for UserSelected {
    type Selection = UserSelection;

    fn from_row(
        selection: &Self::Selection,
        row: <Sqlite as Database>::Row,
    ) -> Result<Self, sqlx::Error>
    where
        Self: Sized,
    {
        let mut selected = Self::default();
        if selection.id {
            selected.id = row.try_get("id").ok().into();
        }
        if selection.request_id {
            selected.request_id = row.try_get("request_id").ok().into();
        }
        if selection.name {
            selected.name = row.try_get("name").ok().into();
        }
        if selection.age {
            selected.age = row.try_get("age").ok().into();
        }
        if selection.birthday {
            selected.birthday = row.try_get("birthday").ok().into();
        }
        Ok(selected)
    }
}

#[derive(Debug, Default)]
pub struct UserSelection {
    pub id: bool,
    pub request_id: bool,
    pub name: bool,
    pub age: bool,
    pub birthday: bool,
    // money: bool,
    // ipv4addr: bool,
    // ipv6addr: bool,
}

impl Selection for UserSelection {
    fn get_table_name(&self) -> &'static str {
        "user"
    }

    fn get_selected_fields(&self) -> Vec<String> {
        let mut fields = Vec::new();
        if self.id {
            fields.push("id".to_string());
        }
        if self.request_id {
            fields.push("request_id".to_string());
        }
        if self.name {
            fields.push("name".to_string());
        }
        if self.age {
            fields.push("age".to_string());
        }
        if self.birthday {
            fields.push("birthday".to_string());
        }
        fields
    }

    fn get_selected_bits(&self) -> bit_vec::BitVec {
        let mut fields = bit_vec::BitVec::new();
        fields.push(self.id);
        fields.push(self.request_id);
        fields.push(self.name);
        fields.push(self.age);
        fields.push(self.birthday);
        fields
    }


    fn full_fields() -> Self
    where
        Self: Sized,
    {
        Self {
            id: true,
            request_id: true,
            name: true,
            age: true,
            birthday: true,
        }
    }
}

#[derive(Debug)]
pub struct UserMutation {
    pub request_id: taitan_orm::Optional<Uuid>,
    pub name: taitan_orm::Optional<String>,
    pub age: taitan_orm::Optional<i32>,
    pub birthday: taitan_orm::Optional<PrimitiveDateTime>,
    // money: Option<BigDecimal>,
    // ipv4addr: Option<Ipv4Addr>,
    // ipv6addr: Option<Ipv6Addr>,
}

impl Mutation for UserMutation {
    type Location = UserLocation;
    fn get_mutation_fields_name(&self) -> Vec<FieldName> {
        let mut fields = Vec::new();
        if let Optional::Some(_) = &self.request_id {
            fields.push(FieldName::from_str("request_id", false));
        }
        if let Optional::Some(_) = &self.name {
            fields.push(FieldName::from_str("name", false));
        }
        if let Optional::Some(_) = &self.age {
            fields.push(FieldName::from_str("age", false));
        }
        if let Optional::Some(_) = &self.birthday {
            fields.push(FieldName::from_str("birthday", false));
        }
        // if let Some(_) = &self.money {
        //     fields.push("money".to_string());
        // }
        // if let Some(_) = &self.ipv4addr {
        //     fields.push("ipv4addr".to_string());
        // }
        // if let Some(_) = &self.ipv6addr {
        //     fields.push("ipv6addr".to_string());
        // }
        fields
    }
}

#[derive(Debug)]
pub struct UserPrimaryMutationPair<'a>(pub &'a UserMutation, pub &'a UserPrimary);

impl<'a> UpdateCommand for UserPrimaryMutationPair<'a> {
    fn gen_update_arguments_sqlite(&self) -> Result<SqliteArguments<'_>, BoxDynError> {
        let mut args = SqliteArguments::default();

        if let Optional::Some(request_id) = &self.0.request_id {
            args.add(request_id)?;
        }
        if let Optional::Some(name) = &self.0.name {
            args.add(name)?;
        }
        if let Optional::Some(age) = &self.0.age {
            args.add(age)?;
        }
        if let Optional::Some(birthday) = &self.0.birthday {
            args.add(birthday)?;
        }

        args.add(&self.1.id)?;

        Ok(args)
    }
}

#[derive(Debug)]
pub struct UserLocation {
    pub request_id: Optional<LocationExpr<Uuid>>,
    pub name: Optional<LocationExpr<String>>,
    pub age: Optional<LocationExpr<i32>>,
    pub birthday: Optional<LocationExpr<PrimitiveDateTime>>,
    // money: LocationExpr<BigDecimal>,
    // ipv4addr: LocationExpr<Ipv4Addr>,
    // ipv6addr: LocationExpr<Ipv6Addr>,
}

impl UserLocation {
    #[inline(always)]
    pub fn concat_where_clause<T>(
        &self,
        sql: &mut String,
        wrap_char: char,
        place_holder: char,
        field_name: &str,
        loc: &dyn LocationTrait,
    ) {
        sql.push(wrap_char);
        sql.push_str(field_name);
        sql.push(wrap_char);
        sql.push_str(loc.get_cmp_sql());
        sql.push(place_holder);
    }
}

impl Location for UserLocation {
    fn get_table_name(&self) -> &'static str {
        "user"
    }

    fn get_location_fields_name(&self) -> Vec<FieldName> {
        let mut fields = Vec::new();
        if let Optional::Some(_) = &self.request_id {
            fields.push(FieldName::from_str("request_id", false));
        }
        if let Optional::Some(_) = &self.name {
            fields.push(FieldName::from_str("name", false));
        }
        if let Optional::Some(_) = &self.age {
            fields.push(FieldName::from_str("age", false));
        }
        if let Optional::Some(_) = &self.birthday {
            fields.push(FieldName::from_str("birthday", false));
        }
        fields
    }

    fn get_where_clause(&self, wrap_char: char, place_holder: char) -> String {
        let mut sql = String::default();
        if let Optional::Some(request_id) = &self.request_id {
            sql.push(wrap_char);
            sql.push_str("request_id");
            sql.push(wrap_char);
            sql.push_str(request_id.get_cmp_sql());
            sql.push(place_holder);
        }
        if let Optional::Some(name) = &self.name {
            sql.push(wrap_char);
            sql.push_str("name");
            sql.push(wrap_char);
            sql.push_str(name.get_cmp_sql());
            sql.push(place_holder);
        }
        if let Optional::Some(age) = &self.age {
            sql.push(wrap_char);
            sql.push_str("age");
            sql.push(wrap_char);
            sql.push_str(age.get_cmp_sql());
            sql.push(place_holder);
        }
        if let Optional::Some(birthday) = &self.birthday {
            sql.push(wrap_char);
            sql.push_str("birthday");
            sql.push(wrap_char);
            sql.push_str(birthday.get_cmp_sql());
            sql.push(place_holder);
        }
        sql
    }

    fn gen_location_arguments_sqlite(&self) -> Result<SqliteArguments<'_>, BoxDynError> {
        let mut args = SqliteArguments::default();

        if let Optional::Some(request_id) = &self.request_id {
            args.add(&request_id.val)?;
        }
        if let Optional::Some(name) = &self.name {
            args.add(&name.val)?;
        }
        if let Optional::Some(age) = &self.age {
            args.add(&age.val)?;
        }
        if let Optional::Some(birthday) = &self.birthday {
            args.add(&birthday.val)?;
        }

        Ok(args)
    }
    //
    // fn gen_change_arguments_sqlite<'a>(
    //     &'a self,
    //     mutation: &'a Self::Mutation,
    // ) -> Result<SqliteArguments<'a>, BoxDynError> {
    //     let mut args = SqliteArguments::default();
    //
    //     if let Some(request_id) = &mutation.request_id {
    //         args.add(request_id)?;
    //     }
    //     if let Some(name) = &mutation.name {
    //         args.add(name)?;
    //     }
    //     if let Some(age) = &mutation.age {
    //         args.add(age)?;
    //     }
    //     if let Some(birthday) = &mutation.birthday {
    //         args.add(birthday)?;
    //     }
    //
    //     if let Some(request_id) = &self.request_id {
    //         args.add(request_id.val)?;
    //     }
    //     if let Some(name) = &self.name {
    //         args.add(name.clone().val)?;
    //     }
    //     if let Some(age) = &self.age {
    //         args.add(age.val)?;
    //     }
    //     if let Some(birthday) = &self.birthday {
    //         args.add(birthday.val)?;
    //     }
    //
    //     Ok(args)
    // }
}

#[derive(Debug)]
pub struct UserLocationMutationPair(pub UserMutation, pub UserLocation);

impl UpdateCommand for UserLocationMutationPair {
    fn gen_update_arguments_sqlite(&self) -> Result<SqliteArguments<'_>, BoxDynError> {
        let mut args = SqliteArguments::default();

        if let Optional::Some(request_id) = &self.0.request_id {
            args.add(request_id)?;
        }
        if let Optional::Some(name) = &self.0.name {
            args.add(name)?;
        }
        if let Optional::Some(age) = &self.0.age {
            args.add(age)?;
        }
        if let Optional::Some(birthday) = &self.0.birthday {
            args.add(birthday)?;
        }

        if let Optional::Some(request_id) = &self.1.request_id {
            args.add(request_id.val)?;
        }
        if let Optional::Some(name) = &self.1.name {
            args.add(name.clone().val)?;
        }
        if let Optional::Some(age) = &self.1.age {
            args.add(age.val)?;
        }
        if let Optional::Some(birthday) = &self.1.birthday {
            args.add(birthday.val)?;
        }

        Ok(args)
    }
}

#[derive(Debug, Default)]
pub struct UserOrderBy<'a> {
    fields: Vec<Cow<'a, str>>,
}

impl<'a> OrderBy for UserOrderBy<'a> {
    fn unique_fields(&self) -> &[&[&str]] {
        &[&["id"]]
    }

    fn all_fields(&self) -> &[&str] {
        &["id", "name", "age", "birthday"]
    }
    fn get_fields(&self) -> &[Cow<'a, str>] {
        &self.fields
    }
}

impl<'a> UserOrderBy<'a> {
    pub fn build<I, S>(fields: I) -> Result<Self, Box<dyn Error + 'static>>
    where
        I: IntoIterator<Item = S> + Clone,
        S: AsRef<str> + Into<Cow<'a, str>>, // 确保每个元素可以转换为 Cow<'a, str>
    {
        let order_by = Self::default();
        validate_order_by(
            fields.clone(),
            order_by.all_fields(),
            order_by.unique_fields(),
        )?;

        Ok(Self {
            fields: fields.into_iter().map(Into::into).collect(),
        })
    }
}
