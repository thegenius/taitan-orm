use taitan_orm_trait::brave_new::PlaceholderParser;
#[test]
fn test_dynamic_placeholder() {
    let template = "SELECT * FROM users WHERE a=:{a} AND b=:{b} AND c=:{c}";
    let (sql, vars) = PlaceholderParser::parse(template);

    assert_eq!(sql, "SELECT * FROM users WHERE a=? AND b=? AND c=?");
    assert_eq!(vars, vec!["a".to_string(), "b".to_string(), "c".to_string()]);

    let (sql, vars) = PlaceholderParser::parse_indexed(template);

    assert_eq!(sql, "SELECT * FROM users WHERE a=$1 AND b=$2 AND c=$3");
    assert_eq!(vars, vec!["a".to_string(), "b".to_string(), "c".to_string()]);


    let template = "SELECT * FROM users WHERE a=:{a} AND b=:{b} AND c=:{c} OR d='sdf'";
    let (sql, vars) = PlaceholderParser::parse_indexed(template);
    assert_eq!(sql, "SELECT * FROM users WHERE a=$1 AND b=$2 AND c=$3 OR d='sdf'");
    assert_eq!(vars, vec!["a".to_string(), "b".to_string(), "c".to_string()]);

    let template = "SELECT * FROM users WHERE a=:{a} AND b=:{b} AND c=:{c} OR d=':{name}'";
    let (sql, vars) = PlaceholderParser::parse_indexed(template);
    assert_eq!(sql, "SELECT * FROM users WHERE a=$1 AND b=$2 AND c=$3 OR d=':{name}'");
    assert_eq!(vars, vec!["a".to_string(), "b".to_string(), "c".to_string()]);
}