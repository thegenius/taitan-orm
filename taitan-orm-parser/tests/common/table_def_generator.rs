// use crate::common::input_spec::input_spec;
// use crate::common::ExpectSql;
// use syn::DeriveInput;
// use taitan_orm_parser::{DatabaseType, SqlGenerator, SqlType, TableDef};
//
// pub struct TableDefGenerator<'a> {
//     derive_inputs: Vec<DeriveInput>,
//     table_defs: Vec<TableDef<'a>>,
// }
//
// impl<'a> TableDefGenerator<'a> {
//     pub fn new() -> TableDefGenerator<'a> {
//         let mut generator = TableDefGenerator {
//             derive_inputs: input_spec(),
//             table_defs: Vec::new(),
//         };
//         generator
//     }
//
//     pub fn validate(
//         &self,
//         db_type: DatabaseType,
//         sql_type: SqlType,
//         expect_sql: &'static [&'static str],
//     ) {
//         let generator = SqlGenerator::default();
//         let expected_sql = ExpectSql::new(expect_sql);
//         for (index, table_def) in self.iter().enumerate() {
//             let insert_sql = generator
//                 .gen_sql(&db_type, &sql_type, &table_def)
//                 .to_string();
//             expected_sql.expect(&insert_sql, index);
//         }
//     }
//
//     pub fn get_def(&'a self, index: usize) -> TableDef<'a> {
//         TableDef::parse(&self.derive_inputs[index])
//     }
//
//     pub fn iter(&'a self) -> TableDefIterator<'a> {
//         TableDefIterator {
//             generator: self,
//             index: 0,
//         }
//     }
// }
//
// pub struct TableDefIterator<'a> {
//     generator: &'a TableDefGenerator<'a>,
//     index: usize,
// }
//
// // 实现 Iterator trait
// impl<'a> Iterator for TableDefIterator<'a> {
//     type Item = TableDef<'a>;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         if self.index < self.generator.derive_inputs.len() {
//             let table_def = self.generator.get_def(self.index);
//             self.index += 1;
//             Some(table_def)
//         } else {
//             None
//         }
//     }
// }
