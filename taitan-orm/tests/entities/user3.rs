use taitan_orm_trait::Optional;
use time::PrimitiveDateTime;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct UserEntity {
    pub id: Optional<i64>,

    pub request_id: Uuid,

    pub age: Optional<i32>,

    pub name: String,

    pub birthday: Optional<PrimitiveDateTime>,
}

impl taitan_orm::traits::Entity for UserEntity {
    fn get_table_name(&self) -> &'static str {
        "user"
    }
    fn get_insert_fields(&self) -> Vec<taitan_orm::prelude::FieldName> {
        let mut fields = Vec::new();
        fields.push(taitan_orm::prelude::FieldName::from_str("r_id", false));
        match &self.age {
            taitan_orm::result::Optional::Some(age) => {
                fields.push(taitan_orm::prelude::FieldName::from_str("age", false));
            }
            taitan_orm::result::Optional::Null => {
                fields.push(taitan_orm::prelude::FieldName::from_str("age", true));
            }
            _ => {}
        };
        fields.push(taitan_orm::prelude::FieldName::from_str("name", false));
        match &self.birthday {
            taitan_orm::result::Optional::Some(birthday) => {
                fields.push(taitan_orm::prelude::FieldName::from_str("birthday", false));
            }
            taitan_orm::result::Optional::Null => {
                fields.push(taitan_orm::prelude::FieldName::from_str("birthday", true));
            }
            _ => {}
        };
        return fields;
    }
    fn get_upsert_set_fields(&self) -> Vec<taitan_orm::prelude::FieldName> {
        let mut fields = Vec::new();
        fields.push(taitan_orm::prelude::FieldName::from_str("r_id", false));
        match &self.age {
            taitan_orm::result::Optional::Some(age) => {
                fields.push(taitan_orm::prelude::FieldName::from_str("age", false));
            }
            taitan_orm::result::Optional::Null => {
                fields.push(taitan_orm::prelude::FieldName::from_str("age", true));
            }
            _ => {}
        };
        fields.push(taitan_orm::prelude::FieldName::from_str("name", false));
        match &self.birthday {
            taitan_orm::result::Optional::Some(birthday) => {
                fields.push(taitan_orm::prelude::FieldName::from_str("birthday", false));
            }
            taitan_orm::result::Optional::Null => {
                fields.push(taitan_orm::prelude::FieldName::from_str("birthday", true));
            }
            _ => {}
        };
        return fields;
    }
    fn get_auto_increment_field(&self) -> Option<&'static str> {
        Some("id")
    }
    fn set_auto_increment_field(&mut self, value: Option<i64>) -> bool {
        true
    }
    fn gen_insert_arguments_sqlite(
        &self,
    ) -> Result<sqlx::sqlite::SqliteArguments<'_>, sqlx::error::BoxDynError> {
        let mut args = sqlx::sqlite::SqliteArguments::default();
        sqlx::Arguments::add(&mut args, &self.request_id)?;
        if let taitan_orm::result::Optional::Some(age) = &self.age {
            sqlx::Arguments::add(&mut args, age)?;
        }
        sqlx::Arguments::add(&mut args, &self.name)?;
        if let taitan_orm::result::Optional::Some(birthday) = &self.birthday {
            sqlx::Arguments::add(&mut args, birthday)?;
        }
        Ok(args)
    }
    fn gen_upsert_arguments_sqlite(
        &self,
    ) -> Result<sqlx::sqlite::SqliteArguments<'_>, sqlx::error::BoxDynError> {
        let mut args = sqlx::sqlite::SqliteArguments::default();
        sqlx::Arguments::add(&mut args, &self.request_id)?;
        if let taitan_orm::result::Optional::Some(age) = &self.age {
            sqlx::Arguments::add(&mut args, age)?;
        }
        sqlx::Arguments::add(&mut args, &self.name)?;
        if let taitan_orm::result::Optional::Some(birthday) = &self.birthday {
            sqlx::Arguments::add(&mut args, birthday)?;
        }
        sqlx::Arguments::add(&mut args, &self.request_id)?;
        if let taitan_orm::result::Optional::Some(age) = &self.age {
            sqlx::Arguments::add(&mut args, age)?;
        }
        sqlx::Arguments::add(&mut args, &self.name)?;
        if let taitan_orm::result::Optional::Some(birthday) = &self.birthday {
            sqlx::Arguments::add(&mut args, birthday)?;
        }
        Ok(args)
    }
    fn gen_insert_arguments_mysql(
        &self,
    ) -> Result<sqlx::mysql::MySqlArguments, sqlx::error::BoxDynError> {
        let mut args = sqlx::mysql::MySqlArguments::default();
        sqlx::Arguments::add(&mut args, &self.request_id)?;
        if let taitan_orm::result::Optional::Some(age) = &self.age {
            sqlx::Arguments::add(&mut args, age)?;
        }
        sqlx::Arguments::add(&mut args, &self.name)?;
        if let taitan_orm::result::Optional::Some(birthday) = &self.birthday {
            sqlx::Arguments::add(&mut args, birthday)?;
        }
        Ok(args)
    }
    fn gen_upsert_arguments_mysql(
        &self,
    ) -> Result<sqlx::mysql::MySqlArguments, sqlx::error::BoxDynError> {
        let mut args = sqlx::mysql::MySqlArguments::default();
        sqlx::Arguments::add(&mut args, &self.request_id)?;
        if let taitan_orm::result::Optional::Some(age) = &self.age {
            sqlx::Arguments::add(&mut args, age)?;
        }
        sqlx::Arguments::add(&mut args, &self.name)?;
        if let taitan_orm::result::Optional::Some(birthday) = &self.birthday {
            sqlx::Arguments::add(&mut args, birthday)?;
        }
        sqlx::Arguments::add(&mut args, &self.request_id)?;
        if let taitan_orm::result::Optional::Some(age) = &self.age {
            sqlx::Arguments::add(&mut args, age)?;
        }
        sqlx::Arguments::add(&mut args, &self.name)?;
        if let taitan_orm::result::Optional::Some(birthday) = &self.birthday {
            sqlx::Arguments::add(&mut args, birthday)?;
        }
        Ok(args)
    }
    fn gen_insert_arguments_postgres(
        &self,
    ) -> Result<sqlx::postgres::PgArguments, sqlx::error::BoxDynError> {
        let mut args = sqlx::postgres::PgArguments::default();
        sqlx::Arguments::add(&mut args, &self.request_id)?;
        if let taitan_orm::result::Optional::Some(age) = &self.age {
            sqlx::Arguments::add(&mut args, age)?;
        }
        sqlx::Arguments::add(&mut args, &self.name)?;
        if let taitan_orm::result::Optional::Some(birthday) = &self.birthday {
            sqlx::Arguments::add(&mut args, birthday)?;
        }
        Ok(args)
    }
    fn gen_upsert_arguments_postgres(
        &self,
    ) -> Result<sqlx::postgres::PgArguments, sqlx::error::BoxDynError> {
        let mut args = sqlx::postgres::PgArguments::default();
        sqlx::Arguments::add(&mut args, &self.request_id)?;
        if let taitan_orm::result::Optional::Some(age) = &self.age {
            sqlx::Arguments::add(&mut args, age)?;
        }
        sqlx::Arguments::add(&mut args, &self.name)?;
        if let taitan_orm::result::Optional::Some(birthday) = &self.birthday {
            sqlx::Arguments::add(&mut args, birthday)?;
        }
        sqlx::Arguments::add(&mut args, &self.request_id)?;
        if let taitan_orm::result::Optional::Some(age) = &self.age {
            sqlx::Arguments::add(&mut args, age)?;
        }
        sqlx::Arguments::add(&mut args, &self.name)?;
        if let taitan_orm::result::Optional::Some(birthday) = &self.birthday {
            sqlx::Arguments::add(&mut args, birthday)?;
        }
        Ok(args)
    }
}
#[derive(Debug, Clone, serde :: Deserialize, serde :: Serialize)]
pub struct UserPrimary {
    pub id: i64,
}
impl taitan_orm::traits::Unique for UserPrimary {
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
    ) -> Result<sqlx::sqlite::SqliteArguments<'a>, sqlx::error::BoxDynError> {
        let mut args = sqlx::sqlite::SqliteArguments::default();
        if let taitan_orm::result::Optional::Some(request_id) = &mutation.request_id {
            sqlx::Arguments::add(&mut args, request_id)?;
        }
        if let taitan_orm::result::Optional::Some(age) = &mutation.age {
            sqlx::Arguments::add(&mut args, age)?;
        }
        if let taitan_orm::result::Optional::Some(name) = &mutation.name {
            sqlx::Arguments::add(&mut args, name)?;
        }
        if let taitan_orm::result::Optional::Some(birthday) = &mutation.birthday {
            sqlx::Arguments::add(&mut args, birthday)?;
        }
        sqlx::Arguments::add(&mut args, &self.id)?;
        Ok(args)
    }
    fn gen_update_arguments_mysql<'a>(
        &'a self,
        mutation: &'a Self::Mutation,
    ) -> Result<sqlx::mysql::MySqlArguments, sqlx::error::BoxDynError> {
        let mut args = sqlx::mysql::MySqlArguments::default();
        if let taitan_orm::result::Optional::Some(request_id) = &mutation.request_id {
            sqlx::Arguments::add(&mut args, request_id)?;
        }
        if let taitan_orm::result::Optional::Some(age) = &mutation.age {
            sqlx::Arguments::add(&mut args, age)?;
        }
        if let taitan_orm::result::Optional::Some(name) = &mutation.name {
            sqlx::Arguments::add(&mut args, name)?;
        }
        if let taitan_orm::result::Optional::Some(birthday) = &mutation.birthday {
            sqlx::Arguments::add(&mut args, birthday)?;
        }
        sqlx::Arguments::add(&mut args, &self.id)?;
        Ok(args)
    }
    fn gen_update_arguments_postgres<'a>(
        &'a self,
        mutation: &'a Self::Mutation,
    ) -> Result<sqlx::postgres::PgArguments, sqlx::error::BoxDynError> {
        let mut args = sqlx::postgres::PgArguments::default();
        if let taitan_orm::result::Optional::Some(request_id) = &mutation.request_id {
            sqlx::Arguments::add(&mut args, request_id)?;
        }
        if let taitan_orm::result::Optional::Some(age) = &mutation.age {
            sqlx::Arguments::add(&mut args, age)?;
        }
        if let taitan_orm::result::Optional::Some(name) = &mutation.name {
            sqlx::Arguments::add(&mut args, name)?;
        }
        if let taitan_orm::result::Optional::Some(birthday) = &mutation.birthday {
            sqlx::Arguments::add(&mut args, birthday)?;
        }
        sqlx::Arguments::add(&mut args, &self.id)?;
        Ok(args)
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
#[derive(Debug, Clone, serde :: Deserialize, serde :: Serialize)]
pub struct UserAgeUnique {
    pub age: i32,
}
impl taitan_orm::traits::Unique for UserAgeUnique {
    type Mutation = UserMutation;
    fn get_table_name(&self) -> &'static str {
        "user"
    }
    fn get_unique_field_names(&self) -> &'static [&'static str] {
        &["age"]
    }
    fn gen_update_arguments_sqlite<'a>(
        &'a self,
        mutation: &'a Self::Mutation,
    ) -> Result<sqlx::sqlite::SqliteArguments<'a>, sqlx::error::BoxDynError> {
        let mut args = sqlx::sqlite::SqliteArguments::default();
        if let taitan_orm::result::Optional::Some(request_id) = &mutation.request_id {
            sqlx::Arguments::add(&mut args, request_id)?;
        }
        if let taitan_orm::result::Optional::Some(age) = &mutation.age {
            sqlx::Arguments::add(&mut args, age)?;
        }
        if let taitan_orm::result::Optional::Some(name) = &mutation.name {
            sqlx::Arguments::add(&mut args, name)?;
        }
        if let taitan_orm::result::Optional::Some(birthday) = &mutation.birthday {
            sqlx::Arguments::add(&mut args, birthday)?;
        }
        sqlx::Arguments::add(&mut args, &self.age)?;
        Ok(args)
    }
    fn gen_update_arguments_mysql<'a>(
        &'a self,
        mutation: &'a Self::Mutation,
    ) -> Result<sqlx::mysql::MySqlArguments, sqlx::error::BoxDynError> {
        let mut args = sqlx::mysql::MySqlArguments::default();
        if let taitan_orm::result::Optional::Some(request_id) = &mutation.request_id {
            sqlx::Arguments::add(&mut args, request_id)?;
        }
        if let taitan_orm::result::Optional::Some(age) = &mutation.age {
            sqlx::Arguments::add(&mut args, age)?;
        }
        if let taitan_orm::result::Optional::Some(name) = &mutation.name {
            sqlx::Arguments::add(&mut args, name)?;
        }
        if let taitan_orm::result::Optional::Some(birthday) = &mutation.birthday {
            sqlx::Arguments::add(&mut args, birthday)?;
        }
        sqlx::Arguments::add(&mut args, &self.age)?;
        Ok(args)
    }
    fn gen_update_arguments_postgres<'a>(
        &'a self,
        mutation: &'a Self::Mutation,
    ) -> Result<sqlx::postgres::PgArguments, sqlx::error::BoxDynError> {
        let mut args = sqlx::postgres::PgArguments::default();
        if let taitan_orm::result::Optional::Some(request_id) = &mutation.request_id {
            sqlx::Arguments::add(&mut args, request_id)?;
        }
        if let taitan_orm::result::Optional::Some(age) = &mutation.age {
            sqlx::Arguments::add(&mut args, age)?;
        }
        if let taitan_orm::result::Optional::Some(name) = &mutation.name {
            sqlx::Arguments::add(&mut args, name)?;
        }
        if let taitan_orm::result::Optional::Some(birthday) = &mutation.birthday {
            sqlx::Arguments::add(&mut args, birthday)?;
        }
        sqlx::Arguments::add(&mut args, &self.age)?;
        Ok(args)
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
#[derive(Debug, Clone, serde :: Deserialize, serde :: Serialize)]
pub struct UserNameBirthdayUnique {
    pub name: String,
    pub birthday: PrimitiveDateTime,
}
impl taitan_orm::traits::Unique for UserNameBirthdayUnique {
    type Mutation = UserMutation;
    fn get_table_name(&self) -> &'static str {
        "user"
    }
    fn get_unique_field_names(&self) -> &'static [&'static str] {
        &["name", "birthday"]
    }
    fn gen_update_arguments_sqlite<'a>(
        &'a self,
        mutation: &'a Self::Mutation,
    ) -> Result<sqlx::sqlite::SqliteArguments<'a>, sqlx::error::BoxDynError> {
        let mut args = sqlx::sqlite::SqliteArguments::default();
        if let taitan_orm::result::Optional::Some(request_id) = &mutation.request_id {
            sqlx::Arguments::add(&mut args, request_id)?;
        }
        if let taitan_orm::result::Optional::Some(age) = &mutation.age {
            sqlx::Arguments::add(&mut args, age)?;
        }
        if let taitan_orm::result::Optional::Some(name) = &mutation.name {
            sqlx::Arguments::add(&mut args, name)?;
        }
        if let taitan_orm::result::Optional::Some(birthday) = &mutation.birthday {
            sqlx::Arguments::add(&mut args, birthday)?;
        }
        sqlx::Arguments::add(&mut args, &self.name)?;
        sqlx::Arguments::add(&mut args, &self.birthday)?;
        Ok(args)
    }
    fn gen_update_arguments_mysql<'a>(
        &'a self,
        mutation: &'a Self::Mutation,
    ) -> Result<sqlx::mysql::MySqlArguments, sqlx::error::BoxDynError> {
        let mut args = sqlx::mysql::MySqlArguments::default();
        if let taitan_orm::result::Optional::Some(request_id) = &mutation.request_id {
            sqlx::Arguments::add(&mut args, request_id)?;
        }
        if let taitan_orm::result::Optional::Some(age) = &mutation.age {
            sqlx::Arguments::add(&mut args, age)?;
        }
        if let taitan_orm::result::Optional::Some(name) = &mutation.name {
            sqlx::Arguments::add(&mut args, name)?;
        }
        if let taitan_orm::result::Optional::Some(birthday) = &mutation.birthday {
            sqlx::Arguments::add(&mut args, birthday)?;
        }
        sqlx::Arguments::add(&mut args, &self.name)?;
        sqlx::Arguments::add(&mut args, &self.birthday)?;
        Ok(args)
    }
    fn gen_update_arguments_postgres<'a>(
        &'a self,
        mutation: &'a Self::Mutation,
    ) -> Result<sqlx::postgres::PgArguments, sqlx::error::BoxDynError> {
        let mut args = sqlx::postgres::PgArguments::default();
        if let taitan_orm::result::Optional::Some(request_id) = &mutation.request_id {
            sqlx::Arguments::add(&mut args, request_id)?;
        }
        if let taitan_orm::result::Optional::Some(age) = &mutation.age {
            sqlx::Arguments::add(&mut args, age)?;
        }
        if let taitan_orm::result::Optional::Some(name) = &mutation.name {
            sqlx::Arguments::add(&mut args, name)?;
        }
        if let taitan_orm::result::Optional::Some(birthday) = &mutation.birthday {
            sqlx::Arguments::add(&mut args, birthday)?;
        }
        sqlx::Arguments::add(&mut args, &self.name)?;
        sqlx::Arguments::add(&mut args, &self.birthday)?;
        Ok(args)
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
    mode: taitan_orm::prelude::LocationMode,
    pub id: taitan_orm::result::Optional<taitan_orm::traits::LocationExpr<i64>>,
    pub request_id: taitan_orm::result::Optional<taitan_orm::traits::LocationExpr<Uuid>>,
    pub age: taitan_orm::result::Optional<taitan_orm::traits::LocationExpr<i32>>,
    pub name: taitan_orm::result::Optional<taitan_orm::traits::LocationExpr<String>>,
    pub birthday: taitan_orm::result::Optional<taitan_orm::traits::LocationExpr<PrimitiveDateTime>>,
}
impl taitan_orm::traits::Location for UserLocation {
    fn get_table_name(&self) -> &'static str {
        "user"
    }
    fn get_location_fields_name(&self) -> Vec<taitan_orm::prelude::FieldName> {
        let mut fields = Vec::new();
        match &self.id {
            taitan_orm::result::Optional::Some(id) => {
                fields.push(taitan_orm::prelude::FieldName::from_str("id", false));
            }
            taitan_orm::result::Optional::Null => {
                fields.push(taitan_orm::prelude::FieldName::from_str("id", true));
            }
            _ => {}
        };
        match &self.request_id {
            taitan_orm::result::Optional::Some(request_id) => {
                fields.push(taitan_orm::prelude::FieldName::from_str("r_id", false));
            }
            taitan_orm::result::Optional::Null => {
                fields.push(taitan_orm::prelude::FieldName::from_str("r_id", true));
            }
            _ => {}
        };
        match &self.age {
            taitan_orm::result::Optional::Some(age) => {
                fields.push(taitan_orm::prelude::FieldName::from_str("age", false));
            }
            taitan_orm::result::Optional::Null => {
                fields.push(taitan_orm::prelude::FieldName::from_str("age", true));
            }
            _ => {}
        };
        match &self.name {
            taitan_orm::result::Optional::Some(name) => {
                fields.push(taitan_orm::prelude::FieldName::from_str("name", false));
            }
            taitan_orm::result::Optional::Null => {
                fields.push(taitan_orm::prelude::FieldName::from_str("name", true));
            }
            _ => {}
        };
        match &self.birthday {
            taitan_orm::result::Optional::Some(birthday) => {
                fields.push(taitan_orm::prelude::FieldName::from_str("birthday", false));
            }
            taitan_orm::result::Optional::Null => {
                fields.push(taitan_orm::prelude::FieldName::from_str("birthday", true));
            }
            _ => {}
        };
        return fields;
    }
    fn get_where_clause(&self, wrap_char: char, place_holder: char) -> String {
        let mut sql = String::default();
        let connectives = self.mode.as_connective();
        match &self.id {
            Optional::Some(id) => {
                sql.push(wrap_char);
                sql.push_str("id");
                sql.push(wrap_char);
                sql.push_str(id.cmp.get_sql());
                sql.push(place_holder);
                sql.push_str(connectives);
            }
            Optional::Null => {
                sql.push(wrap_char);
                sql.push_str("id");
                sql.push(wrap_char);
                sql.push_str(" IS NULL ");
            }
            _ => {}
        }
        match &self.request_id {
            Optional::Some(request_id) => {
                sql.push(wrap_char);
                sql.push_str("request_id");
                sql.push(wrap_char);
                sql.push_str(request_id.cmp.get_sql());
                sql.push(place_holder);
                sql.push_str(connectives);
            }
            Optional::Null => {
                sql.push(wrap_char);
                sql.push_str("request_id");
                sql.push(wrap_char);
                sql.push_str(" IS NULL ");
            }
            _ => {}
        }
        match &self.age {
            Optional::Some(age) => {
                sql.push(wrap_char);
                sql.push_str("age");
                sql.push(wrap_char);
                sql.push_str(age.cmp.get_sql());
                sql.push(place_holder);
                sql.push_str(connectives);
            }
            Optional::Null => {
                sql.push(wrap_char);
                sql.push_str("age");
                sql.push(wrap_char);
                sql.push_str(" IS NULL ");
            }
            _ => {}
        }
        match &self.name {
            Optional::Some(name) => {
                sql.push(wrap_char);
                sql.push_str("name");
                sql.push(wrap_char);
                sql.push_str(name.cmp.get_sql());
                sql.push(place_holder);
                sql.push_str(connectives);
            }
            Optional::Null => {
                sql.push(wrap_char);
                sql.push_str("name");
                sql.push(wrap_char);
                sql.push_str(" IS NULL ");
            }
            _ => {}
        }
        match &self.birthday {
            Optional::Some(birthday) => {
                sql.push(wrap_char);
                sql.push_str("birthday");
                sql.push(wrap_char);
                sql.push_str(birthday.cmp.get_sql());
                sql.push(place_holder);
                sql.push_str(connectives);
            }
            Optional::Null => {
                sql.push(wrap_char);
                sql.push_str("birthday");
                sql.push(wrap_char);
                sql.push_str(" IS NULL ");
            }
            _ => {}
        }
        return sql
            .strip_suffix(connectives)
            .unwrap_or(sql.as_str())
            .to_string();
    }
    fn gen_location_arguments_sqlite(
        &self,
    ) -> Result<sqlx::sqlite::SqliteArguments<'_>, sqlx::error::BoxDynError> {
        let mut args = sqlx::sqlite::SqliteArguments::default();
        if let taitan_orm::result::Optional::Some(id) = &self.id {
            sqlx::Arguments::add(&mut args, &id.val)?;
        }
        if let taitan_orm::result::Optional::Some(request_id) = &self.request_id {
            sqlx::Arguments::add(&mut args, &request_id.val)?;
        }
        if let taitan_orm::result::Optional::Some(age) = &self.age {
            sqlx::Arguments::add(&mut args, &age.val)?;
        }
        if let taitan_orm::result::Optional::Some(name) = &self.name {
            sqlx::Arguments::add(&mut args, &name.val)?;
        }
        if let taitan_orm::result::Optional::Some(birthday) = &self.birthday {
            sqlx::Arguments::add(&mut args, &birthday.val)?;
        }
        Ok(args)
    }
    fn gen_location_arguments_mysql(
        &self,
    ) -> Result<sqlx::mysql::MySqlArguments, sqlx::error::BoxDynError> {
        let mut args = sqlx::mysql::MySqlArguments::default();
        if let taitan_orm::result::Optional::Some(id) = &self.id {
            sqlx::Arguments::add(&mut args, &id.val)?;
        }
        if let taitan_orm::result::Optional::Some(request_id) = &self.request_id {
            sqlx::Arguments::add(&mut args, &request_id.val)?;
        }
        if let taitan_orm::result::Optional::Some(age) = &self.age {
            sqlx::Arguments::add(&mut args, &age.val)?;
        }
        if let taitan_orm::result::Optional::Some(name) = &self.name {
            sqlx::Arguments::add(&mut args, &name.val)?;
        }
        if let taitan_orm::result::Optional::Some(birthday) = &self.birthday {
            sqlx::Arguments::add(&mut args, &birthday.val)?;
        }
        Ok(args)
    }
    fn gen_location_arguments_postgres(
        &self,
    ) -> Result<sqlx::postgres::PgArguments, sqlx::error::BoxDynError> {
        let mut args = sqlx::postgres::PgArguments::default();
        if let taitan_orm::result::Optional::Some(id) = &self.id {
            sqlx::Arguments::add(&mut args, &id.val)?;
        }
        if let taitan_orm::result::Optional::Some(request_id) = &self.request_id {
            sqlx::Arguments::add(&mut args, &request_id.val)?;
        }
        if let taitan_orm::result::Optional::Some(age) = &self.age {
            sqlx::Arguments::add(&mut args, &age.val)?;
        }
        if let taitan_orm::result::Optional::Some(name) = &self.name {
            sqlx::Arguments::add(&mut args, &name.val)?;
        }
        if let taitan_orm::result::Optional::Some(birthday) = &self.birthday {
            sqlx::Arguments::add(&mut args, &birthday.val)?;
        }
        Ok(args)
    }
}
#[derive(Debug, Clone)]
pub enum UserLocationExpr {
    Id(taitan_orm::traits::LocationExpr<i64>),
    RequestId(taitan_orm::traits::LocationExpr<Uuid>),
    Age(taitan_orm::traits::LocationExpr<i32>),
    Name(taitan_orm::traits::LocationExpr<String>),
    Birthday(taitan_orm::traits::LocationExpr<PrimitiveDateTime>),
}
impl UserLocationExpr {
    pub fn id(cmp: &str, val: i64) -> Result<Self, taitan_orm::error::TaitanOrmError> {
        Ok(Self::Id(taitan_orm::traits::LocationExpr::from(cmp, val)?))
    }
    pub fn request_id(cmp: &str, val: Uuid) -> Result<Self, taitan_orm::error::TaitanOrmError> {
        Ok(Self::RequestId(taitan_orm::traits::LocationExpr::from(
            cmp, val,
        )?))
    }
    pub fn age(cmp: &str, val: i32) -> Result<Self, taitan_orm::error::TaitanOrmError> {
        Ok(Self::Age(taitan_orm::traits::LocationExpr::from(cmp, val)?))
    }
    pub fn name(cmp: &str, val: String) -> Result<Self, taitan_orm::error::TaitanOrmError> {
        Ok(Self::Name(taitan_orm::traits::LocationExpr::from(
            cmp, val,
        )?))
    }
    pub fn birthday(
        cmp: &str,
        val: PrimitiveDateTime,
    ) -> Result<Self, taitan_orm::error::TaitanOrmError> {
        Ok(Self::Birthday(taitan_orm::traits::LocationExpr::from(
            cmp, val,
        )?))
    }
}
impl taitan_orm::traits::Location for UserLocationExpr {
    fn get_table_name(&self) -> &'static str {
        "user"
    }
    fn get_location_fields_name(&self) -> Vec<taitan_orm::prelude::FieldName> {
        let mut fields = Vec::new();
        match self {
            Self::Id(_) => fields.push(taitan_orm::prelude::FieldName::from_str("id", false)),
            Self::RequestId(_) => {
                fields.push(taitan_orm::prelude::FieldName::from_str("r_id", false))
            }
            Self::Age(_) => fields.push(taitan_orm::prelude::FieldName::from_str("age", false)),
            Self::Name(_) => fields.push(taitan_orm::prelude::FieldName::from_str("name", false)),
            Self::Birthday(_) => {
                fields.push(taitan_orm::prelude::FieldName::from_str("birthday", false))
            }
        }
        return fields;
    }
    fn get_where_clause(&self, wrap_char: char, place_holder: char) -> String {
        let mut sql = String::default();
        match self {
            Self::Id(id) => {
                sql.push(wrap_char);
                sql.push_str("id");
                sql.push(wrap_char);
                sql.push_str(id.cmp.get_sql());
                sql.push(place_holder);
            }
            Self::RequestId(request_id) => {
                sql.push(wrap_char);
                sql.push_str("request_id");
                sql.push(wrap_char);
                sql.push_str(request_id.cmp.get_sql());
                sql.push(place_holder);
            }
            Self::Age(age) => {
                sql.push(wrap_char);
                sql.push_str("age");
                sql.push(wrap_char);
                sql.push_str(age.cmp.get_sql());
                sql.push(place_holder);
            }
            Self::Name(name) => {
                sql.push(wrap_char);
                sql.push_str("name");
                sql.push(wrap_char);
                sql.push_str(name.cmp.get_sql());
                sql.push(place_holder);
            }
            Self::Birthday(birthday) => {
                sql.push(wrap_char);
                sql.push_str("birthday");
                sql.push(wrap_char);
                sql.push_str(birthday.cmp.get_sql());
                sql.push(place_holder);
            }
        }
        return sql;
    }
    fn gen_location_arguments_sqlite(
        &self,
    ) -> Result<sqlx::sqlite::SqliteArguments<'_>, sqlx::error::BoxDynError> {
        let mut args = sqlx::sqlite::SqliteArguments::default();
        match self {
            Self::Id(id) => {
                sqlx::Arguments::add(&mut args, &id.val)?;
            }
            Self::RequestId(request_id) => {
                sqlx::Arguments::add(&mut args, &request_id.val)?;
            }
            Self::Age(age) => {
                sqlx::Arguments::add(&mut args, &age.val)?;
            }
            Self::Name(name) => {
                sqlx::Arguments::add(&mut args, &name.val)?;
            }
            Self::Birthday(birthday) => {
                sqlx::Arguments::add(&mut args, &birthday.val)?;
            }
        }
        Ok(args)
    }
    fn gen_location_arguments_mysql(
        &self,
    ) -> Result<sqlx::mysql::MySqlArguments, sqlx::error::BoxDynError> {
        let mut args = sqlx::mysql::MySqlArguments::default();
        match self {
            Self::Id(id) => {
                sqlx::Arguments::add(&mut args, &id.val)?;
            }
            Self::RequestId(request_id) => {
                sqlx::Arguments::add(&mut args, &request_id.val)?;
            }
            Self::Age(age) => {
                sqlx::Arguments::add(&mut args, &age.val)?;
            }
            Self::Name(name) => {
                sqlx::Arguments::add(&mut args, &name.val)?;
            }
            Self::Birthday(birthday) => {
                sqlx::Arguments::add(&mut args, &birthday.val)?;
            }
        }
        Ok(args)
    }
    fn gen_location_arguments_postgres(
        &self,
    ) -> Result<sqlx::postgres::PgArguments, sqlx::error::BoxDynError> {
        let mut args = sqlx::postgres::PgArguments::default();
        match self {
            Self::Id(id) => {
                sqlx::Arguments::add(&mut args, &id.val)?;
            }
            Self::RequestId(request_id) => {
                sqlx::Arguments::add(&mut args, &request_id.val)?;
            }
            Self::Age(age) => {
                sqlx::Arguments::add(&mut args, &age.val)?;
            }
            Self::Name(name) => {
                sqlx::Arguments::add(&mut args, &name.val)?;
            }
            Self::Birthday(birthday) => {
                sqlx::Arguments::add(&mut args, &birthday.val)?;
            }
        }
        Ok(args)
    }
}
#[derive(Default, Debug, Clone)]
pub struct UserMutation {
    pub request_id: taitan_orm::result::Optional<Uuid>,
    pub age: taitan_orm::result::Optional<i32>,
    pub name: taitan_orm::result::Optional<String>,
    pub birthday: taitan_orm::result::Optional<PrimitiveDateTime>,
}
impl taitan_orm::traits::Mutation for UserMutation {
    type Location = UserLocation;
    fn get_mutation_fields_name(&self) -> Vec<taitan_orm::prelude::FieldName> {
        let mut fields = Vec::new();
        match &self.request_id {
            taitan_orm::result::Optional::Some(request_id) => {
                fields.push(taitan_orm::prelude::FieldName::from_str("r_id", false));
            }
            taitan_orm::result::Optional::Null => {
                fields.push(taitan_orm::prelude::FieldName::from_str("r_id", true));
            }
            _ => {}
        };
        match &self.age {
            taitan_orm::result::Optional::Some(age) => {
                fields.push(taitan_orm::prelude::FieldName::from_str("age", false));
            }
            taitan_orm::result::Optional::Null => {
                fields.push(taitan_orm::prelude::FieldName::from_str("age", true));
            }
            _ => {}
        };
        match &self.name {
            taitan_orm::result::Optional::Some(name) => {
                fields.push(taitan_orm::prelude::FieldName::from_str("name", false));
            }
            taitan_orm::result::Optional::Null => {
                fields.push(taitan_orm::prelude::FieldName::from_str("name", true));
            }
            _ => {}
        };
        match &self.birthday {
            taitan_orm::result::Optional::Some(birthday) => {
                fields.push(taitan_orm::prelude::FieldName::from_str("birthday", false));
            }
            taitan_orm::result::Optional::Null => {
                fields.push(taitan_orm::prelude::FieldName::from_str("birthday", true));
            }
            _ => {}
        };
        return fields;
    }
    fn gen_change_arguments_sqlite<'a>(
        &'a self,
        location: &'a Self::Location,
    ) -> Result<sqlx::sqlite::SqliteArguments<'a>, sqlx::error::BoxDynError> {
        let mut args = sqlx::sqlite::SqliteArguments::default();
        if let taitan_orm::result::Optional::Some(request_id) = &self.request_id {
            sqlx::Arguments::add(&mut args, request_id)?;
        }
        if let taitan_orm::result::Optional::Some(age) = &self.age {
            sqlx::Arguments::add(&mut args, age)?;
        }
        if let taitan_orm::result::Optional::Some(name) = &self.name {
            sqlx::Arguments::add(&mut args, name)?;
        }
        if let taitan_orm::result::Optional::Some(birthday) = &self.birthday {
            sqlx::Arguments::add(&mut args, birthday)?;
        }
        if let taitan_orm::result::Optional::Some(id) = &location.id {
            sqlx::Arguments::add(&mut args, &id.val)?;
        }
        if let taitan_orm::result::Optional::Some(request_id) = &location.request_id {
            sqlx::Arguments::add(&mut args, &request_id.val)?;
        }
        if let taitan_orm::result::Optional::Some(age) = &location.age {
            sqlx::Arguments::add(&mut args, &age.val)?;
        }
        if let taitan_orm::result::Optional::Some(name) = &location.name {
            sqlx::Arguments::add(&mut args, &name.val)?;
        }
        if let taitan_orm::result::Optional::Some(birthday) = &location.birthday {
            sqlx::Arguments::add(&mut args, &birthday.val)?;
        }
        Ok(args)
    }
    fn gen_change_arguments_mysql<'a>(
        &'a self,
        location: &'a Self::Location,
    ) -> Result<sqlx::mysql::MySqlArguments, sqlx::error::BoxDynError> {
        let mut args = sqlx::mysql::MySqlArguments::default();
        if let taitan_orm::result::Optional::Some(request_id) = &self.request_id {
            sqlx::Arguments::add(&mut args, request_id)?;
        }
        if let taitan_orm::result::Optional::Some(age) = &self.age {
            sqlx::Arguments::add(&mut args, age)?;
        }
        if let taitan_orm::result::Optional::Some(name) = &self.name {
            sqlx::Arguments::add(&mut args, name)?;
        }
        if let taitan_orm::result::Optional::Some(birthday) = &self.birthday {
            sqlx::Arguments::add(&mut args, birthday)?;
        }
        if let taitan_orm::result::Optional::Some(id) = &location.id {
            sqlx::Arguments::add(&mut args, &id.val)?;
        }
        if let taitan_orm::result::Optional::Some(request_id) = &location.request_id {
            sqlx::Arguments::add(&mut args, &request_id.val)?;
        }
        if let taitan_orm::result::Optional::Some(age) = &location.age {
            sqlx::Arguments::add(&mut args, &age.val)?;
        }
        if let taitan_orm::result::Optional::Some(name) = &location.name {
            sqlx::Arguments::add(&mut args, &name.val)?;
        }
        if let taitan_orm::result::Optional::Some(birthday) = &location.birthday {
            sqlx::Arguments::add(&mut args, &birthday.val)?;
        }
        Ok(args)
    }
    fn gen_change_arguments_postgres<'a>(
        &'a self,
        location: &'a Self::Location,
    ) -> Result<sqlx::postgres::PgArguments, sqlx::error::BoxDynError> {
        let mut args = sqlx::postgres::PgArguments::default();
        if let taitan_orm::result::Optional::Some(request_id) = &self.request_id {
            sqlx::Arguments::add(&mut args, request_id)?;
        }
        if let taitan_orm::result::Optional::Some(age) = &self.age {
            sqlx::Arguments::add(&mut args, age)?;
        }
        if let taitan_orm::result::Optional::Some(name) = &self.name {
            sqlx::Arguments::add(&mut args, name)?;
        }
        if let taitan_orm::result::Optional::Some(birthday) = &self.birthday {
            sqlx::Arguments::add(&mut args, birthday)?;
        }
        if let taitan_orm::result::Optional::Some(id) = &location.id {
            sqlx::Arguments::add(&mut args, &id.val)?;
        }
        if let taitan_orm::result::Optional::Some(request_id) = &location.request_id {
            sqlx::Arguments::add(&mut args, &request_id.val)?;
        }
        if let taitan_orm::result::Optional::Some(age) = &location.age {
            sqlx::Arguments::add(&mut args, &age.val)?;
        }
        if let taitan_orm::result::Optional::Some(name) = &location.name {
            sqlx::Arguments::add(&mut args, &name.val)?;
        }
        if let taitan_orm::result::Optional::Some(birthday) = &location.birthday {
            sqlx::Arguments::add(&mut args, &birthday.val)?;
        }
        Ok(args)
    }
}
#[derive(Default, Debug, Clone)]
pub struct UserSelection {
    pub id: bool,
    pub request_id: bool,
    pub age: bool,
    pub name: bool,
    pub birthday: bool,
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
            fields.push("r_id".to_string());
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
#[derive(taitan_orm::prelude::Selected, Default, Debug, Clone)]
#[table_name = "user"]
pub struct UserSelectedEntity {
    pub id: taitan_orm::result::Optional<i64>,
    pub request_id: taitan_orm::result::Optional<Uuid>,
    pub age: taitan_orm::result::Optional<i32>,
    pub name: taitan_orm::result::Optional<String>,
    pub birthday: taitan_orm::result::Optional<PrimitiveDateTime>,
}

//
// impl taitan_orm::traits::Selection for UserSelectedEntity {
//     fn get_table_name(&self) -> &'static str {
//         "user"
//     }
//     fn get_selected_fields(&self) -> Vec<String> {
//         let mut fields = Vec::new();
//         fields.push("id".to_string());
//         fields.push("request_id".to_string());
//         fields.push("age".to_string());
//         fields.push("name".to_string());
//         fields.push("birthday".to_string());
//         return fields;
//     }
//     fn full_fields() -> Self
//     where
//         Self: Sized + Default,
//     {
//         Self {
//             ..Default::default()
//         }
//     }
// }

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
