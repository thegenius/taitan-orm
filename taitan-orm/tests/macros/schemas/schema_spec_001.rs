// use serde::Deserialize;
//
// use taitan_orm_macro::Schema;
//
// #[derive(Schema)]
// #[primary(a)]
// #[unique(uk_01 = (a, b))]
// #[index(
//    idx_01 = (a, b, c),
//    idx_02 = (a, b, c,d),
// )]
// #[derive(Debug)]
// struct SchemaSpec001 {
//     a: String,
//     b: i64,
//     c: bool,
//     d: i64,
// }
//
// #[test]
// pub fn test() {
//     let spec_001 = SchemaSpec001 {
//         a: "a".to_string(),
//         b: 10,
//         c: true,
//         d: 20,
//     };
//     let spec_primary = SchemaSpec001Primary { a: "a".to_string() };
//
//     let selected = SchemaSpec001Selected {
//         a: Some(None),
//         b: Some(None),
//         c: Some(None),
//         d: Some(None),
//     };
//     let default_selected = SchemaSpec001Selected::default();
// }
//
