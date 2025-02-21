use crate::common::named_input::NamedDeriveInput;

pub fn derive_input_sets() -> Vec<NamedDeriveInput> {
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