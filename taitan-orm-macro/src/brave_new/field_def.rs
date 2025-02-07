use std::borrow::Cow;

//  _____________________________________________________________
// | struct-field-name | inner rust type | is optional | default |
//  -------------------------------------------------------------
// | table-column-name | column type     | nullable | default | is generated | is auto-inc | is primary-key part|
//  ------------------------------------------------------------------------------------------------------------
//

#[derive(Debug, PartialEq, Clone)]
pub struct StructFieldDef {
    name: Cow<'static, str>,
    rust_type: Cow<'static, str>,
    default_value: Option<Cow<'static, str>>,
    is_optional: bool,
}


#[derive(Debug, PartialEq, Clone)]
pub struct TableColumnDef {
    name: Option<Cow<'static, str>>,
    column_type: Option<Cow<'static, str>>,
    default_value: Option<Cow<'static, str>>,
    is_nullable: bool,
    is_generated: bool,
    is_auto_inc: bool,
    is_primary_key_part: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FieldDef {
    struct_field: StructFieldDef,
    table_column: TableColumnDef,
}


impl FieldDef {
    pub fn database_field_name(&self) -> &str {
        match &self.table_column.name {
            Some(column_name) => column_name,
            None => &self.struct_field.name,
        }
    }
}