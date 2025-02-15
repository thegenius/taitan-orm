mod field_processor;
mod keywords_escaper;

use crate::sql_generator::field_processor::FieldProcessor;
use crate::TableDef;

pub trait SqlGenerator {
    type FieldProcessor: FieldProcessor;

    fn get_field_processor(&self) -> &Self::FieldProcessor;

    // if all fields are not optional
    fn gen_insert_sql(&self, table_def: TableDef) -> String {
        let table_name = table_def.table_name;
        let fields = self.get_field_processor().gen_list_string(&table_def.fields);
        let marks = self.get_field_processor().gen_marks(&table_def.fields);
        format!("INSERT INTO {}({}) VALUES ({})", table_name, fields, marks)
    }



}

