use crate::field_def::FieldName::Named;
use crate::KeywordsEscaper;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use syn::Field;
// struct $Ident {
//   name: FieldType
// }

// condition
// enum $Ident {
//   A(Expr<Cow<'a, str>>)
//   B(Option<Expr<i64>>)
// }

// index
// enum $Ident {
//    IdxName{ name: Cow<'a, str> }
//    IdxNameAge{ name: String, age: i64 }
// }

//  _____________________________________________________________
// | struct-field-name | inner rust type | is optional | lifetime
//  -------------------------------------------------------------
// | table-column-name | column type     | nullable | default | is generated | is auto-inc | is primary-key part|
//  ------------------------------------------------------------------------------------------------------------
//

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum FieldName<'a> {
    Named(Cow<'a, str>),
    Unnamed { idx: usize, name: Cow<'a, str> },
}

impl<'a> Default for FieldName<'a> {
    fn default() -> Self {
        FieldName::Unnamed {
            idx: 0,
            name: Cow::Borrowed("e0"),
        }
    }
}

impl<'a> FieldName<'a> {
    pub fn named<T: Into<Cow<'a, str>>>(name: T) -> Self {
        FieldName::Named(name.into())
    }
    pub fn unnamed(idx: usize) -> Self {
        FieldName::Unnamed { idx, name: Cow::Owned(format!("e{}", idx)) }
    }
    pub fn get_name(&self) -> &str {
        match self {
            FieldName::Named(name) => name,
            FieldName::Unnamed { name, .. } => name,
        }
    }
}


#[derive(Debug, PartialEq, Clone, Default)]
pub struct StructFieldDef<'a> {
    pub name: FieldName<'a>,
    pub rust_type: Cow<'a, str>,
    pub is_optional: bool,
    pub is_location_expr: bool,
    pub is_enum_variant: bool,
    pub lifetime: Option<Cow<'a, str>>,
    pub field: Option<Field>, // struct字段还原的时候Field最为方便
}

impl<'a> StructFieldDef<'a> {
    pub fn get_name(&self) -> Cow<'a, str> {
        match &self.name {
            FieldName::Named(n) => n.clone(),
            FieldName::Unnamed { idx: _, name } => name.clone(),
        }
    }
}

// #[field(name = r_id, type = BIGINT, nullable = true, auto_inc = true)]
#[derive(Debug, PartialEq, Clone, Default)]
pub struct TableColumnDef<'a> {
    pub name: Option<Cow<'a, str>>,
    pub column_type: Option<Cow<'a, str>>,
    pub default_value: Option<Cow<'a, str>>,
    pub generated: Option<Cow<'a, str>>,
    pub nullable: bool,
    pub auto_inc: bool,
}

#[derive(Debug, PartialEq, Clone, Default)]
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
    pub fn is_optional(&self) -> bool {
        self.struct_field.is_optional
    }

    pub fn is_required(&self) -> bool {
        !self.struct_field.is_optional
    }

    pub fn is_location_expr(&self) -> bool {
        self.struct_field.is_location_expr
    }

    pub fn origin_column_name(&self) -> &Cow<'_, str> {
        match &self.table_column.name {
            Some(column_name) => column_name,
            None => match &self.struct_field.name {
                FieldName::Named(name) => name,
                FieldName::Unnamed { name, .. } => name,
            },
        }
    }
    pub fn column_name(&self, escaper: &dyn KeywordsEscaper) -> Cow<'_, str> {
        let origin = match &self.table_column.name {
            Some(column_name) => column_name,
            None => match &self.struct_field.name {
                FieldName::Named(name) => name,
                FieldName::Unnamed { name, .. } => name,
            },
        };
        escaper.escape(&origin)
    }

    pub fn column_name_upsert(&self, escaper: &dyn KeywordsEscaper) -> String {
        let column_name = self.column_name(escaper);
        escaper.gen_upsert_name(&column_name)
    }
}
