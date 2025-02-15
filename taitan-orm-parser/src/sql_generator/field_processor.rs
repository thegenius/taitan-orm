use crate::sql_generator::keywords_escaper::{
    KeywordsEscaper, MySqlKeywordEscaper, PostgresKeywordEscaper, SqliteKeywordEscaper,
};
use crate::FieldDef;
use std::borrow::Cow;

pub trait FieldProcessor {
    type Escaper: KeywordsEscaper;
    fn get_escaper(&self) -> &Self::Escaper;


    fn gen_list_string(&self, fields: &[FieldDef]) -> String {
        fields
            .iter()
            .map(|f| self.get_escaper().escape(f.column_name()))
            .collect::<Vec<Cow<'_, str>>>()
            .join(",")
    }
    fn gen_plain_marks(&self, fields: &[FieldDef]) -> String {
        fields.iter().map(|f| "?").collect::<Vec<&str>>().join(",")
    }
    fn gen_indexed_marks(&self, fields: &[FieldDef]) -> String {
        fields
            .iter()
            .enumerate()
            .map(|(index, _)| format!("${}", index + 1))
            .collect()
    }
    fn gen_marks(&self, fields: &[FieldDef]) -> String;
}

#[derive(Default)]
pub struct MySqlFieldProcessor {
    escaper: MySqlKeywordEscaper,
}
impl FieldProcessor for MySqlFieldProcessor {
    type Escaper = MySqlKeywordEscaper;
    fn get_escaper(&self) -> &Self::Escaper {
        &self.escaper
    }
    fn gen_marks(&self, fields: &[FieldDef]) -> String {
        self.gen_plain_marks(fields)
    }
}

#[derive(Default)]
pub struct PostgresFieldProcessor {
    escaper: PostgresKeywordEscaper,
}
impl FieldProcessor for PostgresFieldProcessor {
    type Escaper = PostgresKeywordEscaper;
    fn get_escaper(&self) -> &Self::Escaper {
        &self.escaper
    }
    fn gen_marks(&self, fields: &[FieldDef]) -> String {
        self.gen_indexed_marks(fields)
    }
}

#[derive(Default)]
pub struct SqliteFieldProcessor {
    escaper: SqliteKeywordEscaper,
}
impl FieldProcessor for SqliteFieldProcessor {
    type Escaper = SqliteKeywordEscaper;
    fn get_escaper(&self) -> &Self::Escaper {
        &self.escaper
    }
    fn gen_marks(&self, fields: &[FieldDef]) -> String {
        self.gen_plain_marks(fields)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use syn::{parse_quote, DeriveInput};
    use crate::TableDef;

    #[test]
    pub fn test_mysql() {
        let input: DeriveInput = parse_quote! {
            #[primary(a, b)]
            struct Foo<'a, 'b> {
                a: &'a str,
                b: Cow<'b, str>,
                c: String,
                d: Option<Cow<'b, str>>,
                e: Optional<Cow<'b, str>>
            }
        };

        let table_def = TableDef::parse(&input);
        let processor = MySqlFieldProcessor::default();
        let field_list = processor.gen_list_string(&table_def.fields);
        assert_eq!(field_list, "a,b,c,d,e");

    }
}
