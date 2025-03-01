use std::borrow::Cow;
use taitan_orm_macro::{MutationNew, Parameter};
use taitan_orm_trait::brave_new::{Entity, Mutation};
use taitan_orm_trait::Optional;

#[derive(Debug, Parameter, MutationNew)]
struct MutationSpec002 {
    f1: String,
    f2: Option<i64>,
    f3: Option<Option<String>>,
}

#[test]
fn test_mutation_spec_002() {
    let entity = MutationSpec002{f1: "hello".to_owned(), f2: Some(123), f3: None};
    let set_sql = Mutation::<sqlx::Sqlite>::gen_update_set_sql(&entity);
    assert_eq!(set_sql, "f1=?,f2=?".to_string());

    let entity = MutationSpec002{f1: "hello".to_owned(), f2: Some(123), f3: Some(None)};
    let set_sql = Mutation::<sqlx::Sqlite>::gen_update_set_sql(&entity);
    assert_eq!(set_sql, "f1=?,f2=?,f3=?".to_string());
}