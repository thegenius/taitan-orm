// use serde::Deserialize;
// use taitan_orm_macro::MutationNew;
// use taitan_orm_macro::Parameter;
// use taitan_orm_macro::SchemaNew;
// use taitan_orm_trait::brave_new::Unique;
// // #[derive(Debug, SchemaNew)]
// // #[primary(a)]
// // #[unique(uk_01 = (a, b))]
// // #[index(
// //    idx_01 = (a, b, c),
// //    idx_02 = (a, b, c,d),
// // )]
// #[derive(Debug)]
// struct SchemaSpec001 {
//     a: String,
//     b: i64,
//     c: bool,
//     d: i64,
// }
// // impl Unique<sqlx::Sqlite> for SchemaSpec001UniqueUk01 {
// //     type Mutation = SchemaSpec001Mutation;
// // }
//
//
//
// #[test]
// fn schema_spec_001() {
//     let entity = SchemaSpec001 {
//         a: "hello".to_string(),
//         b: 23,
//         c: false,
//         d: 1,
//     };
//     let insert_sql = taitan_orm_trait::brave_new::Entity::<sqlx::Sqlite>::gen_insert_sql(&entity);
//     assert_eq!(
//         insert_sql,
//         "INSERT INTO schema_spec001 (a,b,c,d) VALUES(?,?,?,?)"
//     );
// }
