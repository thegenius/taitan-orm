
use crate::common::get_inputs;
use crate::common::get_sql_specs;
use taitan_orm_parser::{SqlGenerator, TableDef};

#[test]
fn run_spec() {
    let input_specs = get_inputs();
    let expected_spec = get_sql_specs();
    let generator = SqlGenerator::default();
    for expected in expected_spec.into_iter() {
        let input = input_specs.get(&expected.input_name);
        if let Some(s) = input {
            let table_def = TableDef::parse(&s.input);
            let actual_sql = generator.gen_sql(&expected.db_type, &expected.sql_type, &table_def);
            if actual_sql.to_string() != expected.expected {
                let err_msg = format!(
                    "{}.{}.{} error:\nactual:\n{}\n\nexpected:\n{}\n",
                    expected.db_type,
                    expected.sql_type,
                    expected.input_name,
                    actual_sql, expected.expected
                );
                panic!("{err_msg}");
            }
        }
    }
}
