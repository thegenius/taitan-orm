use crate::template_parser::structs::atomic::Atomic;
use crate::template_parser::structs::expr::Expr;
use crate::template_parser::structs::template_part::TemplatePart;
use crate::template_parser::to_sql::{SqlSegment, ToSqlSegment};
use nom::{branch::alt, character::complete::multispace0, combinator::map, IResult};
use taitan_orm_tracing::debug;

#[derive(Debug, PartialEq, Clone)]
pub enum SqlPart {
    Template(TemplatePart), // 模板部分
    Expr(Expr),             // 表达式
    Atomic(Atomic),
}
impl SqlPart {
    pub fn parse(input: &str) -> IResult<&str, SqlPart> {
        debug!("SqlSegment parse({}", input);
        let (input, _) = multispace0(input)?; // 跳过前导空格
        alt((
            map(Expr::parse, SqlPart::Expr),
            map(TemplatePart::parse, SqlPart::Template),
            map(Atomic::parse, SqlPart::Atomic),
        ))(input)
    }
}

impl ToSqlSegment for SqlPart {
    fn gen_sql_segments(&self) -> Vec<SqlSegment> {
        match self {
            SqlPart::Template(template) => vec![SqlSegment::Simple(template.to_string())],
            SqlPart::Expr(expr) => expr.gen_sql_segments(),
            SqlPart::Atomic(atomic) => atomic.gen_sql_segments(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_binary_expr() {}
}
