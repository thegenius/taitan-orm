use crate::template_parser::sql_part::SqlSegment;
use rinja::filters::format;
use tracing::debug;

#[derive(Debug, PartialEq, Clone)]
pub struct SqlTemplate {
    pub parts: Vec<SqlSegment>,
}

impl SqlTemplate {
    pub fn new<I>(parts: I) -> Self
    where
        I: IntoIterator<Item = SqlSegment>,
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
            let parse_result = SqlSegment::parse(remainder);
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

#[cfg(test)]
mod sql_template_test {}
