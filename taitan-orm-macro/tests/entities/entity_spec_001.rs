use taitan_orm_macro::{EntityNew, Parameter};
use taitan_orm_trait::brave_new::Entity;

#[derive(Debug, Parameter, EntityNew)]
struct EntitySpec001 {
    a: String,
    b: i64,
}

#[test]
fn test_entity_spec_001() {
    let entity = EntitySpec001{a: "hello".to_owned(), b: 123};
    let insert_sql =  Entity::<sqlx::MySql>::gen_insert_sql(&entity);
    assert_eq!(insert_sql, "INSERT INTO entity_spec001 (a,b) VALUES(?,?)".to_string())
}