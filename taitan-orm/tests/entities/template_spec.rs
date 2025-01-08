use std::borrow::Cow;
use taitan_orm_macro::TemplateRecord;
use taitan_orm_trait::Optional;

#[derive(Clone, Debug)]
pub struct TestTemplate6 {
    name: Optional<String>,
    age: i32,
}

