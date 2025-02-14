use std::borrow::Cow;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::field_def::{FieldDef};


// #[table(user)]
// #[serde_struct(entity, location)]
// #[primary(id)]
// #[unique(name, age)]
// #[index(name = xxx, fields = (f1, f2, f3))]
#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct TableDef<'a> {
    pub table_name: Cow<'a, str>,
    pub serde_structs: Vec<String>,
    pub columns: Vec<FieldDef<'a>>,
    pub primary_key_fields: Vec<FieldDef<'a>>,
    pub non_primary_key_fields: Vec<FieldDef<'a>>,
    pub unique_keys: HashMap<String, Vec<FieldDef<'a>>>,
    pub index_key: HashMap<String, Vec<FieldDef<'a>>>,
}