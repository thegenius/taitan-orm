use crate::template_parser::structs::values::TextValue;
use crate::template_parser::ArithmeticOp;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::multispace0;
use nom::combinator::map;
use nom::sequence::{delimited, preceded};
use nom::IResult;
use taitan_orm_tracing::debug;
use crate::Atomic;

#[derive(Debug, Clone, PartialEq)]
pub enum TextExpr {
    Value(TextValue),
    Nested(Box<TextExpr>),
}

impl TextExpr {
    // pub fn parse<I>(atomics: I) -> Result<TextExpr, String> where I: IntoIterator<Item = Atomic>, {
    //     let atomic_list = atomics.into_iter().collect();
    //     debug!("TextExpr::parse({})", atomic_list);
    //
    // }
}

#[cfg(test)]
mod text_expr_in_file_tests {
    use crate::template_parser::structs::exprs::text_expr::TextExpr;
    use crate::template_parser::structs::text::Text;
    use crate::template_parser::structs::values::TextValue;

    #[test]
    fn test_text_expr_parse() {
        // let template = "'hello world!'";
        // let (_, parsed) = TextExpr::parse(template).unwrap();
        // let expected = TextExpr::Value(TextValue::Value(Text::SingleQuote(
        //     "'hello world!'".to_string(),
        // )));
        // assert_eq!(parsed, expected);
        //
        // let template = "((('hello world!')))";
        // let (_, parsed) = TextExpr::parse(template).unwrap();
        // let expected = TextExpr::Nested(Box::new(TextExpr::Nested(Box::new(TextExpr::Nested(
        //     Box::new(TextExpr::Value(TextValue::Value(Text::SingleQuote(
        //         "'hello world!'".to_string(),
        //     )))),
        // )))));
        // assert_eq!(parsed, expected);
    }
}
