use std::borrow::Cow;
use serde::{Deserialize, Serialize};
use crate::KeywordsEscaper;
//  _____________________________________________________________
// | struct-field-name | inner rust type | is optional | lifetime
//  -------------------------------------------------------------
// | table-column-name | column type     | nullable | default | is generated | is auto-inc | is primary-key part|
//  ------------------------------------------------------------------------------------------------------------
//

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct StructFieldDef<'a> {
    pub name: Cow<'a, str>,
    pub rust_type: Cow<'a, str>,
    pub is_optional: bool,
    pub is_location_expr: bool,
    pub lifetime: Option<Cow<'a, str>>,
}

// #[field(name = r_id, type = BIGINT, nullable = true, auto_inc = true)]
#[derive(Debug, PartialEq, Clone, Default, Deserialize, Serialize)]
pub struct TableColumnDef<'a> {
    pub name: Option<Cow<'a, str>>,
    pub column_type: Option<Cow<'a, str>>,
    pub default_value: Option<Cow<'a, str>>,
    pub generated: Option<Cow<'a, str>>,
    pub nullable: bool,
    pub auto_inc: bool,
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct FieldDef<'a> {
    pub struct_field: StructFieldDef<'a>,
    pub table_column: TableColumnDef<'a>,
}

impl<'a> AsRef<FieldDef<'a>> for FieldDef<'a> {
    fn as_ref(&self) -> &FieldDef<'a> {
        self
    }
}

impl FieldDef<'_> {

    pub fn origin_column_name(&self) -> &Cow<'_, str> {
        match &self.table_column.name {
            Some(column_name) => column_name,
            None => &self.struct_field.name,
        }
    }
    pub fn column_name(&self, escaper: &dyn KeywordsEscaper) -> Cow<'_, str> {
        let origin = match &self.table_column.name {
            Some(column_name) => column_name,
            None => &self.struct_field.name,
        };
        escaper.escape(origin)
    }

    pub fn column_name_upsert(&self, escaper: &dyn KeywordsEscaper) -> String {
        let column_name = self.column_name(escaper);
        escaper.gen_upsert_name(&column_name)
    }
}