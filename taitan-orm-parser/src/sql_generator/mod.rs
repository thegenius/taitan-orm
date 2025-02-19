mod field_processor;
mod keywords_escaper;
mod field_group_list;

use crate::TableDef;

pub use field_group_list::FieldGroup;
pub use field_group_list::FieldGroupList;

pub use keywords_escaper::KeywordsEscaper;
pub use keywords_escaper::MySqlKeywordEscaper;
pub use keywords_escaper::SqliteKeywordEscaper;
pub use keywords_escaper::PostgresKeywordEscaper;

pub trait SqlGenerator {



    // if all fields are not optional
    fn gen_insert_sql(&self, table_def: TableDef) -> String {
        // let table_name = table_def.table_name;
        // let fields = self.get_field_processor().gen_list_string(&table_def.fields);
        // let marks = self.get_field_processor().gen_marks(&table_def.fields);
        // format!("INSERT INTO {}({}) VALUES ({})", table_name, fields, marks)
        todo!()
    }



}

