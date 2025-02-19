mod field_processor;

use crate::TableDef;

// pub use field_group_list::FieldGroup;
// pub use field_group_list::FieldGroupList;



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

