// use taitan_orm_macro::{Entity, Parameter};
// use taitan_orm_trait::traits::Entity;
//
// #[derive(Debug, Parameter, Entity)]
// struct EntitySpec001 {
//     a: String,
//     b: i64,
// }
//
// #[test]
// fn test_entity_spec_001() {
//     let entity = EntitySpec001{a: "hello".to_owned(), b: 123};
//     let insert_sql =  Entity::<sqlx::MySql>::gen_insert_sql(&entity);
//     assert_eq!(insert_sql, "INSERT INTO entity_spec001 (a,b) VALUES(?,?)".to_string())
// }