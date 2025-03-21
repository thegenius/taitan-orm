use taitan_orm_macro::{Entity, Parameter};


#[derive(Debug, Parameter, Entity)]
struct EntitySpec001 {
    a: String,
    b: i64,
}

#[test]
fn test_entity_spec_001() {
    let entity = EntitySpec001{a: "hello".to_owned(), b: 123};
    let insert_sql =  taitan_orm::traits::Entity::<sqlx::MySql>::gen_insert_sql(&entity);
    assert_eq!(insert_sql, "INSERT INTO entity_spec001 (a,b) VALUES(?,?)".to_string())
}