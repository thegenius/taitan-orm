use rinja::Template;

#[derive(Template)]
#[template( source = "{{ gen_template() }}", ext="txt")]
struct RinjaSpec {
    query: String,
    name: String,
    age: Option<i64>
}

impl RinjaSpec {
    pub fn gen_template(&self) -> String {
        let mut s= String::new();
        s.push_str("SELECT");
        s.push(' ');
        s.push_str("*");
        s.push(' ');
        s.push_str("FROM");
        s.push(' ');
        s.push_str("users");
        s.push(' ');
        s.push_str("WHERE");
        s.push(' ');
        s.push_str("query");
        s.push('=');
        s.push('?');
        let expr_0_1 = self.age.is_some();
        if expr_0_1 {
            s.push_str(" AND ");
        }
        if let Some(age) = self.age {
            s.push_str("age");
            s.push_str(">");
            s.push('?');
        };
        s
    }
}

#[test]
pub fn rinja_spec() {
    let template = RinjaSpec {query: "Allen".to_string(), name: "User".to_string(), age: None};
    let s = template.render().unwrap();
    assert_eq!(s, "SELECT * FROM users WHERE query=?");
}