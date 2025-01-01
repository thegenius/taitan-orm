use bit_vec::BitVec;
use sqlx::{Error, Sqlite};
use taitan_orm_trait::FieldName;
use time::PrimitiveDateTime;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct UserEntity {
    id: Option<i64>,

    pub request_id: Uuid,

    age: Option<i32>,

    name: String,

    pub birthday: Option<PrimitiveDateTime>,
}
impl taitan_orm::traits::Entity for UserEntity {
    fn get_table_name(&self) -> &'static str {
        "user"
    }
    fn get_insert_fields(&self) -> Vec<FieldName> {
        let mut fields: Vec<FieldName> = Vec::new();
        fields.push(FieldName::from_str("request_id", false));
        if self.age.is_some() {
            fields.push(FieldName::from_str("age", true));
        };

        FieldName::from_str("name", false);
        if self.birthday.is_some() {
            fields.push(FieldName::from_str("birthday", false));
        };
        return fields;
    }
    fn get_upsert_set_fields(&self) -> Vec<FieldName> {
        let mut fields = Vec::new();
        fields.push(FieldName::from_str("request_id", false));
        if self.age.is_some() {
            fields.push(FieldName::from_str("age", false));
        };
        fields.push(FieldName::from_str("name", false));
        if self.birthday.is_some() {
            fields.push(FieldName::from_str("birthday", false));
        };
        fields
    }
    fn get_auto_increment_field(&self) -> Option<&'static str> {
        Some("id")
    }
    fn set_auto_increment_field(&mut self, value: Option<i64>) -> bool {
        self.id = value;
        true
    }
    fn gen_insert_arguments_sqlite(
        &self,
    ) -> Result<sqlx::sqlite::SqliteArguments<'_>, sqlx::error::BoxDynError> {
        let mut args = sqlx::sqlite::SqliteArguments::default();
        sqlx::Arguments::add(&mut args, &self.request_id)?;
        if let Some(age) = &self.age {
            sqlx::Arguments::add(&mut args, age)?;
        }
        sqlx::Arguments::add(&mut args, &self.name)?;
        if let Some(birthday) = &self.birthday {
            sqlx::Arguments::add(&mut args, birthday)?;
        }
        Ok(args)
    }
    fn gen_upsert_arguments_sqlite(
        &self,
    ) -> Result<sqlx::sqlite::SqliteArguments<'_>, sqlx::error::BoxDynError> {
        let mut args = sqlx::sqlite::SqliteArguments::default();
        sqlx::Arguments::add(&mut args, &self.request_id)?;
        if let Some(age) = &self.age {
            sqlx::Arguments::add(&mut args, age)?;
        }
        sqlx::Arguments::add(&mut args, &self.name)?;
        if let Some(birthday) = &self.birthday {
            sqlx::Arguments::add(&mut args, birthday)?;
        }
        sqlx::Arguments::add(&mut args, &self.request_id)?;
        if let Some(age) = &self.age {
            sqlx::Arguments::add(&mut args, age)?;
        }
        sqlx::Arguments::add(&mut args, &self.name)?;
        if let Some(birthday) = &self.birthday {
            sqlx::Arguments::add(&mut args, birthday)?;
        }
        Ok(args)
    }
    fn gen_insert_arguments_mysql(
        &self,
    ) -> Result<sqlx::mysql::MySqlArguments, sqlx::error::BoxDynError> {
        let mut args = sqlx::mysql::MySqlArguments::default();
        sqlx::Arguments::add(&mut args, &self.request_id)?;
        if let Some(age) = &self.age {
            sqlx::Arguments::add(&mut args, age)?;
        }
        sqlx::Arguments::add(&mut args, &self.name)?;
        if let Some(birthday) = &self.birthday {
            sqlx::Arguments::add(&mut args, birthday)?;
        }
        Ok(args)
    }
    fn gen_upsert_arguments_mysql(
        &self,
    ) -> Result<sqlx::mysql::MySqlArguments, sqlx::error::BoxDynError> {
        let mut args = sqlx::mysql::MySqlArguments::default();
        sqlx::Arguments::add(&mut args, &self.request_id)?;
        if let Some(age) = &self.age {
            sqlx::Arguments::add(&mut args, age)?;
        }
        sqlx::Arguments::add(&mut args, &self.name)?;
        if let Some(birthday) = &self.birthday {
            sqlx::Arguments::add(&mut args, birthday)?;
        }
        sqlx::Arguments::add(&mut args, &self.request_id)?;
        if let Some(age) = &self.age {
            sqlx::Arguments::add(&mut args, age)?;
        }
        sqlx::Arguments::add(&mut args, &self.name)?;
        if let Some(birthday) = &self.birthday {
            sqlx::Arguments::add(&mut args, birthday)?;
        }
        Ok(args)
    }
    fn gen_insert_arguments_postgres(
        &self,
    ) -> Result<sqlx::postgres::PgArguments, sqlx::error::BoxDynError> {
        let mut args = sqlx::postgres::PgArguments::default();
        sqlx::Arguments::add(&mut args, &self.request_id)?;
        if let Some(age) = &self.age {
            sqlx::Arguments::add(&mut args, age)?;
        }
        sqlx::Arguments::add(&mut args, &self.name)?;
        if let Some(birthday) = &self.birthday {
            sqlx::Arguments::add(&mut args, birthday)?;
        }
        Ok(args)
    }
    fn gen_upsert_arguments_postgres(
        &self,
    ) -> Result<sqlx::postgres::PgArguments, sqlx::error::BoxDynError> {
        let mut args = sqlx::postgres::PgArguments::default();
        sqlx::Arguments::add(&mut args, &self.request_id)?;
        if let Some(age) = &self.age {
            sqlx::Arguments::add(&mut args, age)?;
        }
        sqlx::Arguments::add(&mut args, &self.name)?;
        if let Some(birthday) = &self.birthday {
            sqlx::Arguments::add(&mut args, birthday)?;
        }
        sqlx::Arguments::add(&mut args, &self.request_id)?;
        if let Some(age) = &self.age {
            sqlx::Arguments::add(&mut args, age)?;
        }
        sqlx::Arguments::add(&mut args, &self.name)?;
        if let Some(birthday) = &self.birthday {
            sqlx::Arguments::add(&mut args, birthday)?;
        }
        Ok(args)
    }
}
#[derive(Default, Debug, Clone)]
pub struct UserPrimary {
    id: i64,
}
impl taitan_orm::traits::Unique for UserPrimary {
    type Mutation = UserMutation;
    fn get_table_name(&self) -> &'static str {
        "user"
    }
    fn get_unique_field_names(&self) -> &'static [&'static str] {
        &["id"]
    }
    fn gen_unique_arguments_sqlite(
        &self,
    ) -> Result<sqlx::sqlite::SqliteArguments<'_>, sqlx::error::BoxDynError> {
        let mut args = sqlx::sqlite::SqliteArguments::default();
        sqlx::Arguments::add(&mut args, &self.id)?;
        Ok(args)
    }
    fn gen_unique_arguments_mysql(
        &self,
    ) -> Result<sqlx::mysql::MySqlArguments, sqlx::error::BoxDynError> {
        let mut args = sqlx::mysql::MySqlArguments::default();
        sqlx::Arguments::add(&mut args, &self.id)?;
        Ok(args)
    }
    fn gen_unique_arguments_postgres(
        &self,
    ) -> Result<sqlx::postgres::PgArguments, sqlx::error::BoxDynError> {
        let mut args = sqlx::postgres::PgArguments::default();
        sqlx::Arguments::add(&mut args, &self.id)?;
        Ok(args)
    }
}
#[derive(Default, Debug, Clone)]
pub struct UserAgeUnique {
    age: i32,
}
impl taitan_orm::traits::Unique for UserAgeUnique {
    type Mutation = UserMutation;
    fn get_table_name(&self) -> &'static str {
        "user"
    }
    fn get_unique_field_names(&self) -> &'static [&'static str] {
        &["age"]
    }
    fn gen_unique_arguments_sqlite(
        &self,
    ) -> Result<sqlx::sqlite::SqliteArguments<'_>, sqlx::error::BoxDynError> {
        let mut args = sqlx::sqlite::SqliteArguments::default();
        sqlx::Arguments::add(&mut args, &self.age)?;
        Ok(args)
    }
    fn gen_unique_arguments_mysql(
        &self,
    ) -> Result<sqlx::mysql::MySqlArguments, sqlx::error::BoxDynError> {
        let mut args = sqlx::mysql::MySqlArguments::default();
        sqlx::Arguments::add(&mut args, &self.age)?;
        Ok(args)
    }
    fn gen_unique_arguments_postgres(
        &self,
    ) -> Result<sqlx::postgres::PgArguments, sqlx::error::BoxDynError> {
        let mut args = sqlx::postgres::PgArguments::default();
        sqlx::Arguments::add(&mut args, &self.age)?;
        Ok(args)
    }
}
#[derive(Debug, Clone)]
pub struct UserNameBirthdayUnique {
    name: String,
    birthday: PrimitiveDateTime,
}
impl taitan_orm::traits::Unique for UserNameBirthdayUnique {
    type Mutation = UserMutation;
    fn get_table_name(&self) -> &'static str {
        "user"
    }
    fn get_unique_field_names(&self) -> &'static [&'static str] {
        &["name", "birthday"]
    }
    fn gen_unique_arguments_sqlite(
        &self,
    ) -> Result<sqlx::sqlite::SqliteArguments<'_>, sqlx::error::BoxDynError> {
        let mut args = sqlx::sqlite::SqliteArguments::default();
        sqlx::Arguments::add(&mut args, &self.name)?;
        sqlx::Arguments::add(&mut args, &self.birthday)?;
        Ok(args)
    }
    fn gen_unique_arguments_mysql(
        &self,
    ) -> Result<sqlx::mysql::MySqlArguments, sqlx::error::BoxDynError> {
        let mut args = sqlx::mysql::MySqlArguments::default();
        sqlx::Arguments::add(&mut args, &self.name)?;
        sqlx::Arguments::add(&mut args, &self.birthday)?;
        Ok(args)
    }
    fn gen_unique_arguments_postgres(
        &self,
    ) -> Result<sqlx::postgres::PgArguments, sqlx::error::BoxDynError> {
        let mut args = sqlx::postgres::PgArguments::default();
        sqlx::Arguments::add(&mut args, &self.name)?;
        sqlx::Arguments::add(&mut args, &self.birthday)?;
        Ok(args)
    }
}
#[derive(Default, Debug, Clone)]
pub struct UserLocation {
    id: Option<taitan_orm::traits::LocationExpr<i64>>,
    request_id: Option<taitan_orm::traits::LocationExpr<Uuid>>,
    age: Option<taitan_orm::traits::LocationExpr<i32>>,
    name: Option<taitan_orm::traits::LocationExpr<String>>,
    birthday: Option<taitan_orm::traits::LocationExpr<PrimitiveDateTime>>,
}
impl taitan_orm::traits::Location for UserLocation {
    fn get_table_name(&self) -> &'static str {
        "user"
    }
    fn get_location_fields_name(&self) -> Vec<FieldName> {
        let mut fields = Vec::new();
        if self.id.is_some() {
            fields.push(FieldName::from_str("id", false));
        };
        fields.push(FieldName::from_str("request_id", false));
        if self.age.is_some() {
            fields.push(FieldName::from_str("age", false));
        };
        fields.push(FieldName::from_str("name", false));
        if self.birthday.is_some() {
            fields.push(FieldName::from_str("birthday", false));
        };
        fields
    }
    fn get_where_clause(&self, wrap_char: char, place_holder: char) -> String {
        let mut sql = String::default();
        if let Some(id) = &self.id {
            sql.push(wrap_char);
            sql.push_str("id");
            sql.push(wrap_char);
            sql.push_str(id.cmp.get_sql());
            sql.push(place_holder);
        }
        if let Some(request_id) = &self.request_id {
            sql.push(wrap_char);
            sql.push_str("request_id");
            sql.push(wrap_char);
            sql.push_str(request_id.cmp.get_sql());
            sql.push(place_holder);
        }
        if let Some(age) = &self.age {
            sql.push(wrap_char);
            sql.push_str("age");
            sql.push(wrap_char);
            sql.push_str(age.cmp.get_sql());
            sql.push(place_holder);
        }
        if let Some(name) = &self.name {
            sql.push(wrap_char);
            sql.push_str("name");
            sql.push(wrap_char);
            sql.push_str(name.cmp.get_sql());
            sql.push(place_holder);
        }
        if let Some(birthday) = &self.birthday {
            sql.push(wrap_char);
            sql.push_str("birthday");
            sql.push(wrap_char);
            sql.push_str(birthday.cmp.get_sql());
            sql.push(place_holder);
        }
        return sql;
    }
    fn gen_location_arguments_sqlite(
        &self,
    ) -> Result<sqlx::sqlite::SqliteArguments<'_>, sqlx::error::BoxDynError> {
        let mut args = sqlx::sqlite::SqliteArguments::default();
        if let Some(id) = &self.id {
            sqlx::Arguments::add(&mut args, &id.val)?;
        }
        if let Some(request_id) = &self.request_id {
            sqlx::Arguments::add(&mut args, &request_id.val)?;
        }
        if let Some(age) = &self.age {
            sqlx::Arguments::add(&mut args, &age.val)?;
        }
        if let Some(name) = &self.name {
            sqlx::Arguments::add(&mut args, &name.val)?;
        }
        if let Some(birthday) = &self.birthday {
            sqlx::Arguments::add(&mut args, &birthday.val)?;
        }
        Ok(args)
    }
    fn gen_location_arguments_mysql(
        &self,
    ) -> Result<sqlx::mysql::MySqlArguments, sqlx::error::BoxDynError> {
        let mut args = sqlx::mysql::MySqlArguments::default();
        if let Some(id) = &self.id {
            sqlx::Arguments::add(&mut args, &id.val)?;
        }
        if let Some(request_id) = &self.request_id {
            sqlx::Arguments::add(&mut args, &request_id.val)?;
        }
        if let Some(age) = &self.age {
            sqlx::Arguments::add(&mut args, &age.val)?;
        }
        if let Some(name) = &self.name {
            sqlx::Arguments::add(&mut args, &name.val)?;
        }
        if let Some(birthday) = &self.birthday {
            sqlx::Arguments::add(&mut args, &birthday.val)?;
        }
        Ok(args)
    }
    fn gen_location_arguments_postgres(
        &self,
    ) -> Result<sqlx::postgres::PgArguments, sqlx::error::BoxDynError> {
        let mut args = sqlx::postgres::PgArguments::default();
        if let Some(id) = &self.id {
            sqlx::Arguments::add(&mut args, &id.val)?;
        }
        if let Some(request_id) = &self.request_id {
            sqlx::Arguments::add(&mut args, &request_id.val)?;
        }
        if let Some(age) = &self.age {
            sqlx::Arguments::add(&mut args, &age.val)?;
        }
        if let Some(name) = &self.name {
            sqlx::Arguments::add(&mut args, &name.val)?;
        }
        if let Some(birthday) = &self.birthday {
            sqlx::Arguments::add(&mut args, &birthday.val)?;
        }
        Ok(args)
    }
}
#[derive(Default, Debug, Clone)]
pub struct UserMutation {
    request_id: Option<Uuid>,
    age: Option<i32>,
    name: Option<String>,
    birthday: Option<PrimitiveDateTime>,
}
impl taitan_orm::traits::Mutation for UserMutation {
    type Location = UserLocation;
    fn get_mutation_fields_name(&self) -> Vec<FieldName> {
        let mut fields = Vec::new();
        if let Some(_) = self.request_id {
            fields.push(FieldName::from_str("request_id", false));
        };
        if let Some(_) = self.age {
            fields.push(FieldName::from_str("age", false));
        };
        if let Some(_) = self.name {
            fields.push(FieldName::from_str("name", false));
        };
        if let Some(_) = self.birthday {
            fields.push(FieldName::from_str("birthday", false));
        };
        fields
    }

    fn gen_change_arguments_sqlite<'a>(
        &'a self,
        location: &'a Self::Location,
    ) -> Result<sqlx::sqlite::SqliteArguments<'a>, sqlx::error::BoxDynError> {
        let mut args = sqlx::sqlite::SqliteArguments::default();
        if let Some(request_id) = &self.request_id {
            sqlx::Arguments::add(&mut args, request_id)?;
        }
        if let Some(age) = &self.age {
            sqlx::Arguments::add(&mut args, age)?;
        }
        if let Some(name) = &self.name {
            sqlx::Arguments::add(&mut args, name)?;
        }
        if let Some(birthday) = &self.birthday {
            sqlx::Arguments::add(&mut args, birthday)?;
        }
        if let Some(id) = &location.id {
            sqlx::Arguments::add(&mut args, &id.val)?;
        }
        if let Some(request_id) = &location.request_id {
            sqlx::Arguments::add(&mut args, &request_id.val)?;
        }
        if let Some(age) = &location.age {
            sqlx::Arguments::add(&mut args, &age.val)?;
        }
        if let Some(name) = &location.name {
            sqlx::Arguments::add(&mut args, &name.val)?;
        }
        if let Some(birthday) = &location.birthday {
            sqlx::Arguments::add(&mut args, &birthday.val)?;
        }
        Ok(args)
    }
    fn gen_change_arguments_mysql<'a>(
        &'a self,
        location: &'a Self::Location,
    ) -> Result<sqlx::mysql::MySqlArguments, sqlx::error::BoxDynError> {
        let mut args = sqlx::mysql::MySqlArguments::default();
        if let Some(request_id) = &self.request_id {
            sqlx::Arguments::add(&mut args, request_id)?;
        }
        if let Some(age) = &self.age {
            sqlx::Arguments::add(&mut args, age)?;
        }
        if let Some(name) = &self.name {
            sqlx::Arguments::add(&mut args, name)?;
        }
        if let Some(birthday) = &self.birthday {
            sqlx::Arguments::add(&mut args, birthday)?;
        }
        if let Some(id) = &location.id {
            sqlx::Arguments::add(&mut args, &id.val)?;
        }
        if let Some(request_id) = &location.request_id {
            sqlx::Arguments::add(&mut args, &request_id.val)?;
        }
        if let Some(age) = &location.age {
            sqlx::Arguments::add(&mut args, &age.val)?;
        }
        if let Some(name) = &location.name {
            sqlx::Arguments::add(&mut args, &name.val)?;
        }
        if let Some(birthday) = &location.birthday {
            sqlx::Arguments::add(&mut args, &birthday.val)?;
        }
        Ok(args)
    }
    fn gen_change_arguments_postgres<'a>(
        &'a self,
        location: &'a Self::Location,
    ) -> Result<sqlx::postgres::PgArguments, sqlx::error::BoxDynError> {
        let mut args = sqlx::postgres::PgArguments::default();
        if let Some(request_id) = &self.request_id {
            sqlx::Arguments::add(&mut args, request_id)?;
        }
        if let Some(age) = &self.age {
            sqlx::Arguments::add(&mut args, age)?;
        }
        if let Some(name) = &self.name {
            sqlx::Arguments::add(&mut args, name)?;
        }
        if let Some(birthday) = &self.birthday {
            sqlx::Arguments::add(&mut args, birthday)?;
        }
        if let Some(id) = &location.id {
            sqlx::Arguments::add(&mut args, &id.val)?;
        }
        if let Some(request_id) = &location.request_id {
            sqlx::Arguments::add(&mut args, &request_id.val)?;
        }
        if let Some(age) = &location.age {
            sqlx::Arguments::add(&mut args, &age.val)?;
        }
        if let Some(name) = &location.name {
            sqlx::Arguments::add(&mut args, &name.val)?;
        }
        if let Some(birthday) = &location.birthday {
            sqlx::Arguments::add(&mut args, &birthday.val)?;
        }
        Ok(args)
    }
}
#[derive(Default, Debug, Clone)]
pub struct UserSelection {
    id: bool,
    request_id: bool,
    age: bool,
    name: bool,
    birthday: bool,
}
impl taitan_orm::traits::Selection for UserSelection {
    fn get_table_name(&self) -> &'static str {
        "user"
    }
    fn get_selected_fields(&self) -> Vec<String> {
        let mut fields = Vec::new();
        if self.id {
            fields.push("id".to_string());
        };
        if self.request_id {
            fields.push("request_id".to_string());
        };
        if self.age {
            fields.push("age".to_string());
        };
        if self.name {
            fields.push("name".to_string());
        };
        if self.birthday {
            fields.push("birthday".to_string());
        };
        return fields;
    }

    fn get_selected_bits(&self) -> BitVec {
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
            age: true,
            name: true,
            birthday: true,
        }
    }
}
#[derive(Default, Debug, Clone)]
pub struct UserSelectedEntity {
    id: Option<i64>,
    request_id: Option<Uuid>,
    age: Option<i32>,
    name: Option<String>,
    birthday: Option<PrimitiveDateTime>,
}
impl taitan_orm::traits::SelectedEntity<sqlx::Sqlite> for UserSelectedEntity {
    type Selection = UserSelection;
    fn from_row(
        selection: &Self::Selection,
        row: <sqlx::Sqlite as sqlx::Database>::Row,
    ) -> Result<Self, sqlx::Error>
    where
        Self: Sized,
    {
        let mut selected = Self::default();
        let mut i = 0;
        if selection.id {
            selected.id = sqlx::Row::try_get(&row, i).ok();
            i += 1;
        };
        if selection.request_id {
            selected.request_id = sqlx::Row::try_get(&row, i).ok();
            i += 1;
        };
        if selection.age {
            selected.age = sqlx::Row::try_get(&row, i).ok();
            i += 1;
        };
        if selection.name {
            selected.name = sqlx::Row::try_get(&row, i).ok();
            i += 1;
        };
        if selection.birthday {
            selected.birthday = sqlx::Row::try_get(&row, i).ok();
            i += 1;
        };
        Ok(selected)
    }

    fn from_row_bits(bits: &bit_vec::BitVec, row: <sqlx::Sqlite as sqlx::Database>::Row) -> Result<Self, sqlx::Error>
    where
        Self: Sized {
        let mut selected = Self::default();
        let mut i = 0;
        if bits.get(0).unwrap_or(false) {
            selected.id = sqlx::Row::try_get(&row, i).ok();
            i += 1;
        };
        if bits.get(1).unwrap_or(false) {
            selected.request_id = sqlx::Row::try_get(&row, i).ok();
            i += 1;
        };
        if bits.get(2).unwrap_or(false)  {
            selected.age = sqlx::Row::try_get(&row, i).ok();
            i += 1;
        };
        if bits.get(3).unwrap_or(false)  {
            selected.name = sqlx::Row::try_get(&row, i).ok();
            i += 1;
        };
        if bits.get(4).unwrap_or(false)  {
            selected.birthday = sqlx::Row::try_get(&row, i).ok();
            i += 1;
        };
        Ok(selected)
    }

    fn from_row_full(row: <sqlx::Sqlite as sqlx::Database>::Row) -> Result<Self, sqlx::Error>
    where
        Self: Sized,
    {
        let mut selected = Self::default();
        let mut i = 0;
        selected.id = sqlx::Row::try_get(&row, i).ok();
        i += 1;
        selected.request_id = sqlx::Row::try_get(&row, i).ok();
        i += 1;
        selected.age = sqlx::Row::try_get(&row, i).ok();
        i += 1;
        selected.name = sqlx::Row::try_get(&row, i).ok();
        i += 1;
        selected.birthday = sqlx::Row::try_get(&row, i).ok();
        i += 1;
        Ok(selected)
    }
}
impl taitan_orm::traits::SelectedEntity<sqlx::MySql> for UserSelectedEntity {
    type Selection = UserSelection;
    fn from_row(
        selection: &Self::Selection,
        row: <sqlx::MySql as sqlx::Database>::Row,
    ) -> Result<Self, sqlx::Error>
    where
        Self: Sized,
    {
        let mut selected = Self::default();
        let mut i = 0;
        if selection.id {
            selected.id = sqlx::Row::try_get(&row, i).ok();
            i += 1;
        };
        if selection.request_id {
            selected.request_id = sqlx::Row::try_get(&row, i).ok();
            i += 1;
        };
        if selection.age {
            selected.age = sqlx::Row::try_get(&row, i).ok();
            i += 1;
        };
        if selection.name {
            selected.name = sqlx::Row::try_get(&row, i).ok();
            i += 1;
        };
        if selection.birthday {
            selected.birthday = sqlx::Row::try_get(&row, i).ok();
            i += 1;
        };
        Ok(selected)
    }
    fn from_row_full(row: <sqlx::MySql as sqlx::Database>::Row) -> Result<Self, sqlx::Error>
    where
        Self: Sized,
    {
        let mut selected = Self::default();
        let mut i = 0;
        selected.id = sqlx::Row::try_get(&row, i).ok();
        i += 1;
        selected.request_id = sqlx::Row::try_get(&row, i).ok();
        i += 1;
        selected.age = sqlx::Row::try_get(&row, i).ok();
        i += 1;
        selected.name = sqlx::Row::try_get(&row, i).ok();
        i += 1;
        selected.birthday = sqlx::Row::try_get(&row, i).ok();
        i += 1;
        Ok(selected)
    }
}
impl taitan_orm::traits::SelectedEntity<sqlx::Postgres> for UserSelectedEntity {
    type Selection = UserSelection;
    fn from_row(
        selection: &Self::Selection,
        row: <sqlx::Postgres as sqlx::Database>::Row,
    ) -> Result<Self, sqlx::Error>
    where
        Self: Sized,
    {
        let mut selected = Self::default();
        let mut i = 0;
        if selection.id {
            selected.id = sqlx::Row::try_get(&row, i).ok();
            i += 1;
        };
        if selection.request_id {
            selected.request_id = sqlx::Row::try_get(&row, i).ok();
            i += 1;
        };
        if selection.age {
            selected.age = sqlx::Row::try_get(&row, i).ok();
            i += 1;
        };
        if selection.name {
            selected.name = sqlx::Row::try_get(&row, i).ok();
            i += 1;
        };
        if selection.birthday {
            selected.birthday = sqlx::Row::try_get(&row, i).ok();
            i += 1;
        };
        Ok(selected)
    }
    fn from_row_full(row: <sqlx::Postgres as sqlx::Database>::Row) -> Result<Self, sqlx::Error>
    where
        Self: Sized,
    {
        let mut selected = Self::default();
        let mut i = 0;
        selected.id = sqlx::Row::try_get(&row, i).ok();
        i += 1;
        selected.request_id = sqlx::Row::try_get(&row, i).ok();
        i += 1;
        selected.age = sqlx::Row::try_get(&row, i).ok();
        i += 1;
        selected.name = sqlx::Row::try_get(&row, i).ok();
        i += 1;
        selected.birthday = sqlx::Row::try_get(&row, i).ok();
        i += 1;
        Ok(selected)
    }
}
#[derive(Debug, Default)]
pub struct UserOrdering<'a> {
    fields: Vec<std::borrow::Cow<'a, str>>,
}
impl<'a> taitan_orm::traits::OrderBy for UserOrdering<'a> {
    fn unique_fields(&self) -> &[&[&str]] {
        &[&["age"], &["name", "birthday"], &["id"]]
    }
    fn all_fields(&self) -> &[&str] {
        &["id", "request_id", "age", "name", "birthday"]
    }
    fn get_fields(&self) -> &[std::borrow::Cow<'a, str>] {
        &self.fields
    }
}
impl<'a> UserOrdering<'a> {
    pub fn build<I, S>(fields: I) -> Result<Self, Box<dyn std::error::Error + 'static>>
    where
        I: IntoIterator<Item = S> + Clone,
        S: AsRef<str> + Into<std::borrow::Cow<'a, str>>,
    {
        let order_by = Self::default();
        taitan_orm::traits::validate_order_by(
            fields.clone(),
            taitan_orm::traits::OrderBy::all_fields(&order_by),
            taitan_orm::traits::OrderBy::unique_fields(&order_by),
        )?;
        Ok(Self {
            fields: fields.into_iter().map(Into::into).collect(),
        })
    }
}
