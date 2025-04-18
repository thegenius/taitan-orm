use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};
use crate::expands::{generate_location_impl};
use crate::util::{extract_fields};


pub fn impl_condition_macro(input: TokenStream) -> TokenStream {
    let DeriveInput {
        attrs, ident, data,  ..
    } = parse_macro_input!(input);

    let fields = extract_fields(&data).unwrap();

    let output = generate_location_impl(&ident, &attrs, &fields);

    // panic!("{}", output);
    output.into()
}