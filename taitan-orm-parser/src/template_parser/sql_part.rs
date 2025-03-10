use crate::template_parser::expr::Expr;
use crate::template_parser::structs::atomic::Atomic;
use crate::template_parser::structs::template_part::TemplatePart;
use nom::{branch::alt, character::complete::multispace0, combinator::map, IResult};
use tracing::debug;

#[derive(Debug, PartialEq, Clone)]
pub enum SqlSegment {
    Template(TemplatePart), // 模板部分
    Expr(Expr),             // 表达式
    Atomic(Atomic),
}
impl SqlSegment {
    pub fn parse(input: &str) -> IResult<&str, SqlSegment> {
        debug!("SqlSegment parse({}", input);
        let (input, _) = multispace0(input)?; // 跳过前导空格
        alt((
            map(Expr::parse, SqlSegment::Expr),
            map(TemplatePart::parse, SqlSegment::Template),
            map(Atomic::parse, SqlSegment::Atomic),
        ))(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_binary_expr() {}
}
