use crate::template_parser::sql_part::SqlPart;

#[derive(Debug, PartialEq, Clone)]
pub struct SqlTemplate {
    pub parts: Vec<SqlPart>,
}

impl SqlTemplate {
    pub fn parse(input: &str) -> SqlTemplate {
        let mut parts = Vec::new();
        let mut remainder = input;
        loop {
            let parse_result = SqlPart::parse(remainder);
            if let Ok((remaining, part)) = parse_result {
                parts.push(part);
                remainder = remaining;
            } else {
                panic!("Failed to parse sql template {}", remainder);
            }

            if remainder.is_empty() {
                break;
            }
        }
        SqlTemplate { parts }
    }
}

#[cfg(test)]
mod tests {
    use crate::template_parser::expr::Expr;
    use crate::template_parser::segment::Segment;
    use crate::template_parser::segment::Segment::{Text, Unknown};
    use super::*;

}
