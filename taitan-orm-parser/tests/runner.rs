
mod common;
use common::get_inputs;
use common::get_sql_specs;

#[test]
fn run_spec() {
    let input_specs = get_inputs();
    let expected_spec = get_sql_specs();
}