use crate::NamedVariantDef;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct ConditionDef<'a> {
    pub struct_name: Cow<'a, str>,
    pub table_name: Cow<'a, str>,
    pub variants: Vec<NamedVariantDef<'a>>,
}
