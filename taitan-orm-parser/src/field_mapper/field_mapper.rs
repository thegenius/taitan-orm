use std::borrow::Cow;
use super::base::{
    Connector, KeywordsEscaper, MySqlKeywordEscaper, PostgresKeywordEscaper, SqliteKeywordEscaper,
};
use super::mappers::{ConditionsMapper, MarksMapper, NamesMapper, SetsMapper};
use crate::DatabaseType;
use crate::FieldDef;
use proc_macro2::TokenStream;

#[derive(Clone, Debug, Default)]
pub struct FieldMapper {
    names_mapper: NamesMapper,
    marks_mapper: MarksMapper,
    sets_mapper: SetsMapper,
    conditions_mapper: ConditionsMapper,
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

    pub fn gen_names(&self, fields: &[FieldDef], db_type: &DatabaseType) -> TokenStream {
        self.names_mapper.connect(fields, self.get_escaper(db_type))
    }

    pub fn gen_marks(&self, fields: &[FieldDef], db_type: &DatabaseType) -> TokenStream {
        self.marks_mapper.connect(fields, self.get_escaper(db_type))
    }

    pub fn gen_marks_indexed(&self, fields: &[FieldDef], db_type: &DatabaseType) -> TokenStream {
        self.marks_mapper
            .connect_indexed(fields, self.get_escaper(db_type))
    }

    pub fn gen_sets(&self, fields: &[FieldDef], db_type: &DatabaseType) -> TokenStream {
        self.sets_mapper.connect(fields, self.get_escaper(db_type))
    }

    pub fn gen_sets_indexed(&self, fields: &[FieldDef], db_type: &DatabaseType) -> TokenStream {
        self.sets_mapper
            .connect_indexed(fields, self.get_escaper(db_type))
    }

    pub fn gen_conditions(&self, fields: &[FieldDef], db_type: &DatabaseType) -> TokenStream {
        self.conditions_mapper
            .connect_dynamic(fields, self.get_escaper(db_type))
    }

    pub fn gen_conditions_indexed(
        &self,
        fields: &[FieldDef],
        db_type: &DatabaseType,
    ) -> TokenStream {
        self.conditions_mapper
            .connect_dynamic_indexed(fields, self.get_escaper(db_type))
    }
}
