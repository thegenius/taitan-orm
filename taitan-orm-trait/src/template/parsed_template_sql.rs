use crate::template::to_sql::SqlTemplateSign;
use crate::template::{TemplateExpr, TemplatePlaceholder, TemplateSqlValue, ToSql};
use nom::error::ErrorKind::{Fail, NonEmpty};
use rinja::filters::format;
use crate::template::parsers::parse_template_sql_values;

/// sql允许是simple字符串，或者合法的
/// hash signs应该被转化为 ?
/// dollar signs应该被转化为 {{ var }}，这样可以被渲染
/// percent expr 应该被转化为
/// {% if val.is_some() %}
/// var = {{val.unwrap()}}
/// {% elif val.is_null %}
/// var is null
/// {% else %}
/// {% endif %}
#[derive(Debug, Clone)]
pub struct ParsedTemplateSql {
    set_sql: String,
    where_sql: String,
    template_signs: Vec<String>,
    argument_signs: Vec<TemplateField>,
}

#[derive(Debug, Clone)]
pub struct TemplateField {
    pub name: String,
    pub is_optional: bool,
}


/// used to generate two struct
/// 1. rinja template struct, if there is template signs: dollar signs or percent signs
/// 2. arguments name list, corresponding to ?

impl ParsedTemplateSql {

    pub fn parse(template_sql: &str) -> Result<Self, nom::Err<nom::error::Error<&str>>> {
        let trimmed_template_sql = template_sql.trim();
        let mut values: Vec<TemplateSqlValue> = Vec::new();
        let (mut remaining, mut parsed) = parse_template_sql_values(trimmed_template_sql)?;
        values.extend(parsed);
        while !remaining.is_empty() {
            let trimmed_remaining = remaining.trim();
            (remaining, parsed) = parse_template_sql_values(trimmed_remaining)?;
            values.extend(parsed);
        }


        // let (remaining, parsed) = parse_template_sql_values(trimmed_template_sql)?;
        // if !remaining.is_empty() {
        //     // panic!("there is remaining remaining tokens: {:?}", remaining);
        //     return Err(nom::Err::Error(nom::error::Error::new(remaining, Fail)));
        // }
        Ok(Self::build(values))
    }
    pub fn build(values: Vec<TemplateSqlValue>) -> Self {
        let has_question_mark: bool = values
            .iter()
            .any(|e| &TemplateSqlValue::Segment("?".to_string()) == e);
        if has_question_mark {
            panic!("sql template should not contains ?");
        }

        let template_signs: Vec<String> = values
            .iter()
            .flat_map(TemplateSqlValue::get_template_signs)
            .collect();
        let argument_signs: Vec<TemplateField> = values
            .iter()
            .flat_map(TemplateSqlValue::get_argument_signs)
            .collect();
        let set_sql: String = values
            .iter()
            .map(TemplateSqlValue::to_set_sql)
            .collect::<Vec<String>>()
            .join(" ").trim().to_string();
        let where_sql: String = values
            .iter()
            .map(TemplateSqlValue::to_where_sql)
            .collect::<Vec<String>>()
            .join(" ").trim().to_string();

        Self {
            set_sql,
            where_sql,
            template_signs,
            argument_signs
        }
    }

    pub fn get_set_sql(&self) -> &str {
        &self.set_sql
    }

    pub fn get_where_sql(&self) -> &str {
        &self.where_sql
    }

    pub fn get_template_signs(&self) -> &Vec<String> {
        &self.template_signs
    }
    pub fn get_argument_signs(&self) -> &Vec<TemplateField> {
        &self.argument_signs
    }

    pub fn need_render(&self) -> bool {
        !self.template_signs.is_empty()
    }
}

// impl ParsedTemplateSql {
//     pub fn build(template_sql: &str) -> Result<Self, nom::Err<nom::error::Error<&str>>> {
//         let trimmed_template_sql = template_sql.trim();
//         if trimmed_template_sql.is_empty() {
//             return Err(nom::Err::Error(nom::error::Error::new(template_sql, NonEmpty)));
//         }
//         let (_, parsed) = parse_template_sql(trimmed_template_sql)?;
//         let parsed_template = ParsedTemplateSql::new(parsed);
//         Ok(parsed_template)
//     }
//
//     pub fn need_render(&self) -> bool {
//         !self.dollar_signs.is_empty()
//     }
//
//     fn merge_segments(values: Vec<TemplateValue>) -> Vec<TemplateValue> {
//         let mut merged_values: Vec<TemplateValue> = Vec::new();
//
//         let mut i = 0;
//         while i < values.len() {
//             let value = values.get(i).unwrap();
//             if let TemplateValue::Operator(segment) = value {
//                 if i + 1 < values.len() {
//                     let next = values.get(i + 1).unwrap();
//                     if let TemplateValue::Operator(next) = next {
//                         let merged_segment = TemplateValue::Operator(format!("{}{}", segment, next));
//                         merged_values.push(merged_segment);
//                         i += 2;
//                         continue;
//                     }
//                 }
//             }
//             merged_values.push(value.clone());
//             i += 1;
//         }
//         merged_values
//     }
//     pub fn new(mut values: Vec<TemplateValue>) -> Self {
//
//         let values = Self::merge_segments(values);
//
//         let has_question_mark: bool = values
//             .iter()
//             .any(|e| &TemplateValue::Segment("?".to_string()) == e);
//         if has_question_mark {
//             panic!("sql template should not contains ?");
//         }
//
//         let variables: Vec<String> = values
//             .iter()
//             .filter(|e| matches!(e, TemplateValue::HashVariable(_)))
//             .map(|e| e.inner_string())
//             .collect();
//
//         let dollar_signs: Vec<String> = values
//             .iter()
//             .filter(|e| matches!(e, TemplateValue::DollarVariable(_)))
//             .map(|e| e.inner_string())
//             .collect();
//
//
//         let result: Vec<TemplateValue> = values
//             .into_iter()
//             .map(|e| {
//                 let t: TemplateValue = match e {
//                     TemplateValue::HashVariable(_) => TemplateValue::Segment("?".to_string()),
//                     TemplateValue::DollarVariable(e) => TemplateValue::Segment(format!("{{{{ {} }}}}", e)),
//                     _ => e
//                 };
//                 t
//             })
//             .collect();
//
//
//         let marked_sql = result
//             .iter()
//             .map(|e| e.to_string())
//             .collect::<Vec<String>>()
//             .join(" ");
//
//         Self {
//             sql: marked_sql,
//             variables,
//             dollar_signs,
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Optional;
    use rinja::Template;

    #[derive(Template)]
    #[template(source = "Hello {{ name }}", ext = "txt")]
    struct HelloTemplate<'a> {
        name: &'a str,
    }

    #[derive(Template)]
    #[template(
        source = "{% if name.is_some() %}hello {{ name.unwrap() }}{% elif name.is_null() %}hello is null{% else %}{% endif %}",
        ext = "txt"
    )]
    struct IfBlockTemplate<'a> {
        name: Optional<&'a str>,
    }

    #[test]
    fn test_rinja_dollar_placeholder() {
        let hello_template = HelloTemplate { name: "Allen" };
        let rendered = hello_template.render().unwrap();
        assert_eq!(rendered, "Hello Allen");
    }

    #[test]
    fn test_rinja_if_block() {
        let if_template = IfBlockTemplate {
            name: Optional::Some("Allen"),
        };
        let rendered = if_template.render().unwrap();
        assert_eq!(rendered, "hello Allen");

        let if_template = IfBlockTemplate {
            name: Optional::Null,
        };
        let rendered = if_template.render().unwrap();
        assert_eq!(rendered, "hello is null");

        let if_template = IfBlockTemplate {
            name: Optional::None,
        };
        let rendered = if_template.render().unwrap();
        assert_eq!(rendered, "");
    }


    // #[derive(Template, Clone, Debug)]
    // #[template(source= "select * from `user` WHERE name = %{name} AND age = #{age} ", ext = "txt")]
    // pub struct TestTemplate6 {
    //     name: Optional<String>,
    //     age: i32,
    // }

    #[test]
    fn test_template_sql() {
        let parsed_template = ParsedTemplateSql::parse("select * from `user` WHERE name = %{name} AND age = #{age} ").unwrap();
        assert_eq!(parsed_template.get_where_sql(), "select * from `user` WHERE {% if name.is_some() %}name = ? AND{% elif name.is_null() %}name IS NULL AND{% else %}{% endif %} age = ?");
        // assert_eq!(parsed_template.variables, vec!["v1.v2.v3"]);
    }

    #[test]
    fn test_parse_sql_1() {
        let parsed_template = ParsedTemplateSql::parse("select count(*) from ${name} #{age} \"hello ${name}\"").unwrap();
        assert_eq!(parsed_template.get_where_sql(), "select count ( * ) from {{name}} ? \"hello ${name}\"");

    }

    // #[test]
    // fn test_template_sql() {
    //     let parsed_template = ParsedTemplateSql::build("SELECT * `test` user #{v1. v2. v3} where id = 23").unwrap();
    //     assert_eq!(parsed_template.sql, "SELECT * `test` user ? where id = 23");
    //     assert_eq!(parsed_template.variables, vec!["v1.v2.v3"]);
    // }
    //
    // #[test]
    // fn test_template_sql2() {
    //     let parsed_template = ParsedTemplateSql::build("SELECT * `test` user ${v1. v2. v3} where id = 23").unwrap();
    //     assert_eq!(parsed_template.sql, "SELECT * `test` user {{ v1.v2.v3 }} where id = 23");
    //     assert_eq!(parsed_template.dollar_signs, vec!["v1.v2.v3"]);
    // }
    //
    // #[test]
    // fn test_template_sql3() {
    //     let parsed_template = ParsedTemplateSql::build("select * from #{name}").unwrap();
    //     assert_eq!(parsed_template.sql, "select * from ?");
    //     assert_eq!(parsed_template.variables, vec!["name"]);
    // }
}
