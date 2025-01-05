use std::fmt::Debug;
use crate::sql_generator::{SqlGenerator};

pub trait SqlGeneratorContainer {
    type G: SqlGenerator + Sync + Debug;

    fn get_generator(&self) -> &Self::G;
}