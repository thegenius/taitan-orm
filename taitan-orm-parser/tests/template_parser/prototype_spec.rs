use rinja::Template;
use sqlx::mysql::MySqlArguments;
use sqlx::Arguments;
use taitan_orm_trait::brave_new::PlaceholderParser;

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

pub trait TemplateTrait {
    fn get_template_sql(&self) -> String;

    fn get_rendered_sql(&self) -> DynamicRenderedSql {
        let rendered = self.render().unwrap();
        let (sql, vars) = PlaceholderParser::parse(&rendered);
        DynamicRenderedSql {
            sql,
            variables: vars,
        }
    }
    fn add_to_args<T>(&self, name: &str, args: &mut T) where T: Arguments;

    fn get_rendered<T>(&self) -> (String, T) where T: Arguments {
        let DynamicRenderedSql { sql, variables } = self.get_rendered_sql();
        let mut args = T::default();
        for variable in &variables {
            self.add_to_args(variable, &mut args);
        }
        (sql, args)
    }
}

impl<'a> Query<'a> {
    pub fn get_template_sql(&self) -> String {
        "SELECT * FROM users WHERE a=#{a} AND b=#{b} AND c=#{c}".to_string()
    }

    pub fn get_rendered_sql(&self) -> DynamicRenderedSql {
        let rendered = self.render().unwrap();
        let (sql, vars) = PlaceholderParser::parse(&rendered);
        DynamicRenderedSql {
            sql,
            variables: vars,
        }
    }

    pub fn add_to_args(&self, name: &str, args: &mut MySqlArguments) {
        match name {
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

    pub fn get_rendered(&self) -> (String, MySqlArguments) {
        let DynamicRenderedSql { sql, variables } = self.get_rendered_sql();
        let mut args = MySqlArguments::default();
        for variable in &variables {
            self.add_to_args(variable, &mut args);
        }
        (sql, args)
    }
}

#[test]
pub fn test() {
    let query = Query {a: "a", b: Some(1), c: None};
    let (sql, args) = query.get_rendered();
    assert_eq!(sql, "SELECT * FROM users WHERE a=? AND b=? AND c=?");
    assert_eq!(args.len(), 2);
}
