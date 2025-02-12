use std::borrow::Cow;
use serde::{Deserialize, Serialize};
//  _____________________________________________________________
// | struct-field-name | inner rust type | is optional | lifetime
//  -------------------------------------------------------------
// | table-column-name | column type     | nullable | default | is generated | is auto-inc | is primary-key part|
//  ------------------------------------------------------------------------------------------------------------
//

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct StructFieldDef {
    pub name: Cow<'static, str>,
    pub rust_type: Cow<'static, str>,
    pub is_optional: bool,
    pub lifetime: Option<Cow<'static, str>>,
}


#[derive(Debug, PartialEq, Clone, Default, Deserialize, Serialize)]
pub struct TableColumnDef {
    pub name: Option<Cow<'static, str>>,
    pub column_type: Option<Cow<'static, str>>,
    pub default_value: Option<Cow<'static, str>>,
    pub is_nullable: bool,
    pub is_generated: bool,
    pub is_auto_inc: bool,
    pub is_primary_key_part: bool,
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct FieldDef {
    pub struct_field: StructFieldDef,
    pub table_column: TableColumnDef,
}


impl FieldDef {
    pub fn database_field_name(&self) -> &str {
        match &self.table_column.name {
            Some(column_name) => column_name,
            None => &self.struct_field.name,
        }
    }
}