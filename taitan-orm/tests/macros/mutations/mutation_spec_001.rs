use taitan_orm_macro::{Mutation, Parameter};
use taitan_orm_trait::traits::Mutation;
// use taitan_orm_trait::Optional;
#[derive(Debug, Parameter, Mutation)]
struct ParamSpec001 {
    a: String,
    b: i64,
}

#[test]
fn test_mutation_spec_001() {
    let entity = ParamSpec001 {
        a: "hello".to_owned(),
        b: 123,
    };
    let insert_sql = Mutation::<sqlx::Sqlite>::gen_update_set_sql(&entity);
    assert_eq!(insert_sql, "a=?,b=?".to_string())
}
