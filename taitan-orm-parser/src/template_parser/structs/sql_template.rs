use crate::template_parser::structs::sql_part::SqlPart;
use crate::template_parser::to_sql::{SqlSegment, ToSqlSegment};
use taitan_orm_tracing::debug;

#[derive(Debug, PartialEq, Clone)]
pub struct SqlTemplate {
    pub parts: Vec<SqlPart>,
}

impl SqlTemplate {
    pub fn new<I>(parts: I) -> Self
    where
        I: IntoIterator<Item = SqlPart>,
    {
        Self {
            parts: parts.into_iter().collect(),
        }
    }
    pub fn parse(input: &str) -> Result<SqlTemplate, String> {
        debug!("SqlTemplate::parse({})", input);
        let mut parts = Vec::new();
        let mut remainder = input;
        loop {
            let parse_result = SqlPart::parse(remainder);
            if let Ok((remaining, part)) = parse_result {
                parts.push(part);
                remainder = remaining;
            } else {
                let err_msg = format!("failed to parse sql template: {}", input);
                return Err(err_msg);
            }

            if remainder.is_empty() {
                break;
            }
        }
        Ok(SqlTemplate { parts })
    }
}

impl ToSqlSegment for SqlTemplate {
    fn gen_sql_segments(&self) -> Vec<SqlSegment> {
        self.parts
            .iter()
            .map(|p| p.gen_sql_segments())
            .flatten()
            .collect()
    }
}
