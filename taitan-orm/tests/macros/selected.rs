use serde::Deserialize;
use taitan_orm_macro::Selected;
use taitan_orm_trait::Optional;

#[derive(Selected, Debug, Default)]
pub struct SelectedTest {
    age: i32,
    name: Optional<String>
}