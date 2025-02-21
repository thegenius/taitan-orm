use crate::common::named_input::{NamedDeriveInput};
use std::sync::OnceLock;
use crate::common::named_map::NamedMap;
use crate::register::inputs::derive_input_sets;

static INPUT_MAP: OnceLock<NamedMap<NamedDeriveInput>> = OnceLock::new();
pub fn get_inputs<'a>() -> NamedMap<NamedDeriveInput> {
    let input_map = INPUT_MAP.get_or_init(|| {
        let mut inputs = NamedMap::new();
        derive_input_sets().into_iter().for_each(|n| {
            inputs.insert(n)
        });
        inputs
    });
    input_map.clone()
}


