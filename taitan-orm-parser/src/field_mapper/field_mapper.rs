use super::base::{
    Connector, KeywordsEscaper, MySqlKeywordEscaper, PostgresKeywordEscaper, SqliteKeywordEscaper,
};
use super::mappers::{ConditionsMapper, MarksMapper, NamesMapper, SetsMapper, UpsertSetsMapper};
use crate::DatabaseType;
use crate::FieldDef;
use proc_macro2::TokenStream;
use std::borrow::Cow;

#[derive(Clone, Debug, Default)]
pub struct FieldMapper {
    names_mapper: NamesMapper,
    marks_mapper: MarksMapper,
    sets_mapper: SetsMapper,
    conditions_mapper: ConditionsMapper,
    upsert_sets_mapper: UpsertSetsMapper,
    mysql_escaper: MySqlKeywordEscaper,
    postgres_escaper: PostgresKeywordEscaper,
    sqlite_escaper: SqliteKeywordEscaper,
}

impl FieldMapper {
    pub fn new() -> Self {
        Self::default()
    }

    fn get_escaper(&self, db_type: &DatabaseType) -> &dyn KeywordsEscaper {
        match db_type {
            DatabaseType::MySql => &self.mysql_escaper,
            DatabaseType::Postgres => &self.postgres_escaper,
            DatabaseType::Sqlite => &self.sqlite_escaper,
        }
    }

    pub fn escape<'a>(&'a self, word: &'a str, db_type: &DatabaseType) -> Cow<'a, str> {
        self.get_escaper(db_type).escape(word)
    }

    pub fn gen_names<'a, T>(&self, fields: T, db_type: &DatabaseType) -> TokenStream
    where
        T: IntoIterator<Item = &'a FieldDef<'a>> + Clone,
    {
        self.names_mapper.connect(fields, self.get_escaper(db_type))
    }

    pub fn gen_upsert_sets<'a, T>(&self, fields: T, db_type: &DatabaseType) -> TokenStream
    where
        T: IntoIterator<Item = &'a FieldDef<'a>> + Clone,
    {
        self.upsert_sets_mapper
            .connect(fields, self.get_escaper(db_type))
    }

    pub fn gen_marks<'a, T>(&self, fields: T, db_type: &DatabaseType) -> TokenStream
    where
        T: IntoIterator<Item = &'a FieldDef<'a>> + Clone,
    {
        match db_type {
            DatabaseType::MySql => self.marks_mapper.connect(fields, self.get_escaper(db_type)),
            _ => self
                .marks_mapper
                .connect_indexed(fields, self.get_escaper(db_type)),
        }
    }

    // pub fn gen_marks_indexed<'a, T>(&self, fields: T, db_type: &DatabaseType) -> TokenStream
    // where
    //     T: IntoIterator<Item = &'a FieldDef<'a>> + Clone,{
    //     self.marks_mapper
    //         .connect_indexed(fields, self.get_escaper(db_type))
    // }

    pub fn gen_sets<'a, T>(&self, fields: T, db_type: &DatabaseType) -> TokenStream
    where
        T: IntoIterator<Item = &'a FieldDef<'a>> + Clone,
    {
        match db_type {
            DatabaseType::MySql => self.sets_mapper.connect(fields, self.get_escaper(db_type)),
            _ => self
                .sets_mapper
                .connect_indexed(fields, self.get_escaper(db_type)),
        }
    }

    // pub fn gen_sets_indexed<'a, T>(&self, fields: T, db_type: &DatabaseType) -> TokenStream
    // where
    //     T: IntoIterator<Item = &'a FieldDef<'a>> + Clone,
    // {
    //     self.sets_mapper
    //         .connect_indexed(fields, self.get_escaper(db_type))
    // }

    pub fn gen_conditions<'a, T>(&self, fields: T, db_type: &DatabaseType) -> TokenStream
    where
        T: IntoIterator<Item = &'a FieldDef<'a>> + Clone,
    {
        match db_type {
            DatabaseType::MySql => self.conditions_mapper.connect_dynamic(fields, self.get_escaper(db_type)),
            _=> self.conditions_mapper.connect_dynamic_indexed(fields, self.get_escaper(db_type)),
        }
    }

    // pub fn gen_conditions_indexed<'a, T>(&self, fields: T, db_type: &DatabaseType) -> TokenStream
    // where
    //     T: IntoIterator<Item = &'a FieldDef<'a>> + Clone,
    // {
    //     self.conditions_mapper
    //         .connect_dynamic_indexed(fields, self.get_escaper(db_type))
    // }
}
