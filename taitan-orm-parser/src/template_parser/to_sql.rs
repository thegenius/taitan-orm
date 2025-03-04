
use crate::template_parser::structs::TemplateField;

pub trait ToSql {
    fn to_set_sql(&self) -> String;
    fn to_where_sql(&self) -> String;
}

pub trait SqlTemplateSign {
    fn get_template_signs(&self) -> Vec<String> {
        Vec::new()
    }
    fn get_argument_signs(&self) -> Vec<TemplateField> {
        Vec::new()
    }
}