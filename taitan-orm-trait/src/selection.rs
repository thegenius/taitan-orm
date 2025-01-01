use std::fmt::Debug;

pub trait Selection: Sync + Debug {
    fn get_table_name(&self) -> &'static str;

    fn get_selected_fields(&self) -> Vec<String>;

    fn get_selected_bits(&self) -> bit_vec::BitVec {
        todo!()
    }

    fn full_fields() -> Self
    where
        Self: Sized;
}
