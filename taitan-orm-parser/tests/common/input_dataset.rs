use crate::common::named_input::{NamedDeriveInput};
use std::sync::OnceLock;
use crate::common::named_map::NamedMap;

static INPUT_MAP: OnceLock<NamedMap<NamedDeriveInput>> = OnceLock::new();

fn derive_input_sets() -> Vec<NamedDeriveInput> {
    vec![
        {
            NamedDeriveInput {
                name: "001".to_string(),
                input: syn::parse_str(include_str!("../specs/inputs/input_001.spec")).unwrap(),
            }
        },
        {
            NamedDeriveInput {
                name: "002".to_string(),
                input: syn::parse_str(include_str!("../specs/inputs/input_002.spec")).unwrap(),
            }
        },
    ]
}

pub fn get_inputs<'a>() -> &'a NamedMap<NamedDeriveInput> {
    let input_map = INPUT_MAP.get_or_init(|| {
        let mut inputs = NamedMap::new();
        derive_input_sets().into_iter().for_each(|n| {
            inputs.insert(n)
        });
        inputs
    });
    input_map
}