use sqlx::Arguments;
use taitan_orm_macro::Parameter;
use taitan_orm_trait::brave_new::param::Parameter;
use taitan_orm_trait::Optional;

#[derive(taitan_orm_macro::Parameter)]
struct ParamSpec002 {
    a: Option<String>,
    b: Option<i64>,
    c: Option<Option<i64>>,
}

#[test]
fn test_param_spec_002() {
    let param = ParamSpec002 {
        a: Some("a".to_string()),
        b: None,
        c: None,
    };
    let args = Parameter::<sqlx::Sqlite>::gen_args(&param).unwrap();
    assert_eq!(args.len(), 1);

    let param = ParamSpec002 {
        a: Some("a".to_string()),
        b: Some(2i64),
        c: None,
    };
    let args = Parameter::<sqlx::Sqlite>::gen_args(&param).unwrap();
    assert_eq!(args.len(), 2);

    let param = ParamSpec002 {
        a: Some("a".to_string()),
        b: Some(2i64),
        c: Some(None),
    };
    let args = Parameter::<sqlx::Sqlite>::gen_args(&param).unwrap();
    assert_eq!(args.len(), 3);
}
