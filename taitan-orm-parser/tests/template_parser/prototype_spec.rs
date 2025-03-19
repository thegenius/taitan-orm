use rinja::Template;
use sqlx::mysql::MySqlArguments;
use sqlx::Arguments;
pub struct StaticRenderedSql {
    sql: &'static str,
}

pub struct DynamicRenderedSql {
    sql: String,
    variables: Vec<String>,
}

#[derive(Template)]
#[template(source = "{{ get_template_sql() }}", ext = "txt")]
pub struct Query<'a> {
    a: &'a str,
    b: Option<i64>,
    c: Option<Option<&'a str>>,
}

impl<'a> Query<'a> {
    pub fn get_template_sql(&self) -> String {
        "SELECT * FROM users WHERE a=#{a} AND b=#{b} AND c=#{c}".to_string()
    }

    pub fn get_rendered_sql(&self) -> DynamicRenderedSql {
        let rendered = self.render().unwrap();
        let variables = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        DynamicRenderedSql {
            sql: rendered,
            variables,
        }
    }

    pub fn get_rendered(&self) -> (String, MySqlArguments) {
        let DynamicRenderedSql { sql, variables } = self.get_rendered_sql();
        let mut args = MySqlArguments::default();
        for variable in variables {
            match variable.as_ref() {
                "a" => args.add(&self.a).unwrap(),
                "b" => args.add(self.b).unwrap(),
                "c" => {
                    if let Some(c) = &self.c {
                        args.add(c).unwrap()
                    }
                }
                _ => {}
            }
        }
        (sql, args)
    }
}

#[test]
pub fn test() {
    let query = Query {a: "a", b: Some(1), c: None};
    let (sql, args) = query.get_rendered();
    assert_eq!(sql, "SELECT * FROM users WHERE a=#{a} AND b=#{b} AND c=#{c}");
}
