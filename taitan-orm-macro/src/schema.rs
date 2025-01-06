use crate::expands::{generate_entity_impl, generate_location_expr_enum_and_impl, generate_location_struct_and_impl, generate_mutation_struct_and_impl, generate_ordering_struct_and_impl, generate_selected_struct, generate_selection_struct_and_impl, generate_unique_structs_and_impls};
use crate::util::extract_fields;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};
use crate::attrs::{AttrParser, DefaultAttrParser};

pub fn impl_schema_macro(input: TokenStream) -> TokenStream {
    let DeriveInput {
        attrs, ident, data, ..
    } = parse_macro_input!(input);

    let fields = extract_fields(&data).unwrap();

    let serde_list = DefaultAttrParser::extract_serde_names(&attrs);
    // if !serde_list.is_empty() {
    //     panic!("{}", serde_list.join(", "));
    // }

    let mut output = generate_entity_impl(&ident, &attrs, &fields);
    let primary_struct_stream = generate_unique_structs_and_impls(&ident, &attrs, &fields, serde_list.contains(&"unique"));
    let location_struct_stream = generate_location_struct_and_impl(&ident, &attrs, &fields, serde_list.contains(&"location"));
    let location_enum_stream = generate_location_expr_enum_and_impl(&ident, &attrs, &fields, serde_list.contains(&"location"));
    let mutation_struct_stream = generate_mutation_struct_and_impl(&ident, &attrs, &fields, serde_list.contains(&"mutation"));
    let selection_struct_stream = generate_selection_struct_and_impl(&ident, &attrs, &fields);
    let selected_struct_stream = generate_selected_struct(&ident, &attrs, &fields, serde_list.contains(&"selected"));
    let ordering_struct_stream = generate_ordering_struct_and_impl(&ident, &attrs, &fields);

    output.extend(primary_struct_stream);
    output.extend(location_struct_stream);
    output.extend(location_enum_stream);
    output.extend(mutation_struct_stream);
    output.extend(selection_struct_stream);
    output.extend(selected_struct_stream);
    output.extend(ordering_struct_stream);
    // panic!("{}", output);
    output.into()
}
