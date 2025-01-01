use crate::{Location, Selection};

#[typetag::serde(tag = "table")]
pub trait LocatedQuery {
    fn get_selection(&self) -> &dyn Selection;
    fn get_location(&self) -> &dyn Location;
}
