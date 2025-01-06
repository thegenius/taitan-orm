use taitan_orm_trait::{Optional, Selection};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct SelectedTest {
    age: i32,
    name: Optional<String>,
}

