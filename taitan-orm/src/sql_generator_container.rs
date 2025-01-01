use std::fmt::Debug;
use crate::{SqlGenerator};

pub trait SqlGeneratorContainer {
    type G: SqlGenerator + Sync + Debug;

    fn get_generator(&mut self) -> &Self::G;
}