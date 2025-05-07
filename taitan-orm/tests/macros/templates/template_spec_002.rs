use sqlx::Arguments;
use taitan_orm_macro::Template;
#[derive(Template, Debug)]
#[template(
    source = "SELECT * FROM users WHERE a=:{a} {% if b.is_some() %} AND b=:{b} {% endif %} {% if c.is_some() %} AND c=:{c} {% endif %}",
    ext = "txt"
)]
pub struct Query {
    a: String,
    b: Option<i64>,
    c: Option<Option<String>>,
}


#[test]
pub fn test() {
    let query = Query {
        a: "a".to_string(),
        b: Some(1),
        c: None,
    };
    let (sql, args) = taitan_orm::traits::Template::<sqlx::Postgres>::get_sql(&query).unwrap();
    assert_eq!(sql, "SELECT * FROM users WHERE a=$1  AND b=$2  ");
    assert_eq!(args.len(), 2);

    let query = Query {
        a: "a".to_string(),
        b: None,
        c: Some(None),
    };
    let (sql, args) = taitan_orm::traits::Template::<sqlx::Postgres>::get_sql(&query).unwrap();
    assert_eq!(sql, "SELECT * FROM users WHERE a=$1   AND c=$2 ");
    assert_eq!(args.len(), 2);

    let query = Query {
        a: "a".to_string(),
        b: None,
        c: None,
    };
    let (sql, args) = taitan_orm::traits::Template::<sqlx::Postgres>::get_sql(&query).unwrap();
    assert_eq!(sql, "SELECT * FROM users WHERE a=$1  ");
    assert_eq!(args.len(), 1);
}
