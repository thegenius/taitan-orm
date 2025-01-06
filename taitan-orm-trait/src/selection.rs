use std::fmt::Debug;


// TODO: Sync + Debug maybe removed
pub trait Selection: Sync + Debug {
    fn get_table_name(&self) -> &'static str {
        ""
    }

    // TODO: maybe we need FieldName, not String
    fn get_selected_fields(&self) -> Vec<String> {
        Vec::new()
    }


    // we need return Self, so Self must be Sized
    // we need default value for non-optional field, so Self must support Default
    fn full_fields() -> Self
    where
        Self: Sized + Default {
        Self::default()
    }
}
