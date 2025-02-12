use std::borrow::Cow;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::field_def::{FieldDef};

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct TableDef {
    pub table_name: Cow<'static, str>,
    pub serde_structs: Vec<String>,
    pub columns: Vec<FieldDef>,
    pub primary_key_fields: Vec<FieldDef>,
    pub non_primary_key_fields: Vec<FieldDef>,
    pub unique_keys: HashMap<String, Vec<FieldDef>>,
    pub index_key: HashMap<String, Vec<FieldDef>>,
}