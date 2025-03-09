use crate::template_parser::expr::Expr;
use crate::template_parser::structs::template_part::TemplatePart;
use crate::template_parser::segment::Segment;
use nom::{branch::alt, character::complete::multispace0, combinator::map, IResult};

#[derive(Debug, PartialEq, Clone)]
pub enum SqlPart {
    Template(TemplatePart), // 模板部分
    Expr(Expr),             // 表达式
    Segment(Segment),       // 普通文本
}
impl SqlPart {
    pub fn parse(input: &str) -> IResult<&str, SqlPart> {
        let (input, _) = multispace0(input)?; // 跳过前导空格
        alt((
            map(TemplatePart::parse, SqlPart::Template),
            map(Expr::parse, SqlPart::Expr),
            map(Segment::parse, SqlPart::Segment),
        ))(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_binary_expr() {}
}
