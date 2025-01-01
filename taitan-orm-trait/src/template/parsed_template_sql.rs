use nom::error::ErrorKind::NonEmpty;
use crate::template::parser::parse_template_sql;
use crate::template::template_value::{InnerString, TemplateValue};

#[derive(Debug, Clone)]
pub struct ParsedTemplateSql {
    pub sql: String,
    pub variables: Vec<String>,
    pub dollar_signs: Vec<String>,
}

impl ParsedTemplateSql {
    pub fn build(template_sql: &str) -> Result<Self, nom::Err<nom::error::Error<&str>>> {
        let trimmed_template_sql = template_sql.trim();
        if trimmed_template_sql.is_empty() {
            return Err(nom::Err::Error(nom::error::Error::new(template_sql, NonEmpty)));
        }
        let (_, parsed) = parse_template_sql(trimmed_template_sql)?;
        let parsed_template = ParsedTemplateSql::new(parsed);
        Ok(parsed_template)
    }

    pub fn need_render(&self) -> bool {
        !self.dollar_signs.is_empty()
    }

    pub fn new(values: Vec<TemplateValue>) -> Self {
        let has_question_mark: bool = values
            .iter()
            .any(|e| &TemplateValue::Segment("?".to_string()) == e);
        if has_question_mark {
            panic!("sql template should not contains ?");
        }

        let variables: Vec<String> = values
            .iter()
            .filter(|e| matches!(e, TemplateValue::HashVariable(_)))
            .map(|e| e.inner_string())
            .collect();

        let dollar_signs: Vec<String> = values
            .iter()
            .filter(|e| matches!(e, TemplateValue::DollarVariable(_)))
            .map(|e| e.inner_string())
            .collect();


        let result: Vec<TemplateValue> = values
            .into_iter()
            .map(|e| {
                let t: TemplateValue = match e {
                    TemplateValue::HashVariable(_) => TemplateValue::Segment("?".to_string()),
                    TemplateValue::DollarVariable(e) => TemplateValue::Segment(format!("{{{{ {} }}}}", e)),
                    _ => e
                };
                t
            })
            .collect();


        let marked_sql = result
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<String>>()
            .join(" ");

        Self {
            sql: marked_sql,
            variables,
            dollar_signs,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_template_sql() {
        let parsed_template = ParsedTemplateSql::build("SELECT * `test` user #{v1. v2. v3} where id = 23").unwrap();
        assert_eq!(parsed_template.sql, "SELECT * `test` user ? where id = 23");
        assert_eq!(parsed_template.variables, vec!["v1.v2.v3"]);
    }

    #[test]
    fn test_template_sql2() {
        let parsed_template = ParsedTemplateSql::build("SELECT * `test` user ${v1. v2. v3} where id = 23").unwrap();
        assert_eq!(parsed_template.sql, "SELECT * `test` user {{ v1.v2.v3 }} where id = 23");
        assert_eq!(parsed_template.dollar_signs, vec!["v1.v2.v3"]);
    }

    #[test]
    fn test_template_sql3() {
        let parsed_template = ParsedTemplateSql::build("select * from #{name}").unwrap();
        assert_eq!(parsed_template.sql, "select * from ?");
        assert_eq!(parsed_template.variables, vec!["name"]);
    }


}
