use std::borrow::Cow;
use syn::DeriveInput;
use crate::common::named_map::Named;

#[derive(Debug, Clone)]
pub struct NamedDeriveInput {
    pub name: String,
    pub input: DeriveInput,
}

unsafe impl Sync for NamedDeriveInput {}
unsafe impl Send for NamedDeriveInput {}
impl NamedDeriveInput {
    pub fn new(name: String, input: DeriveInput) -> Self {
        Self { name,  input }
    }
}

impl Named for NamedDeriveInput {
    fn name(&self) -> Cow<str> {
        Cow::Borrowed(&self.name)
    }
}



