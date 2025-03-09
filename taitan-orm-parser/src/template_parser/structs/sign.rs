use crate::template_parser::structs::number::Number;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_while1};
use nom::character::complete::satisfy;
use nom::combinator::{map, recognize};
use nom::IResult;

// not alpha,  it is variable
// not number, it is number
// not begin with "/[/`, if begin with "/[/`, it is variable
// not begin with ', if begin with ', it is text
// not begin with @/#/$, it is placeholder
// not begin with {, it is rinja template
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Sign {
    Star,
    Semicolon,
    Unknown(char),
}
impl Sign {
    pub fn parse(input: &str) -> IResult<&str, Sign> {
        alt((
            map(tag("*"), |_| Sign::Star),
            map(tag(";"), |_| Sign::Semicolon),
            parse_unknown,
        ))(input)
    }
}

// 判断字符是否为字母、数字或下划线
fn is_alphanumeric_or_underscore(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}

// 判断字符是否不是字母、数字、下划线或空格
fn sign_condition(c: char) -> bool {
    !is_alphanumeric_or_underscore(c)
        && !c.is_whitespace()
        && c != '('
        && c != ')'
        && c != '['
        && c != ']'
        && c != '{'
        && c != '}'
        && c != '@'
        && c != '#'
        && c != '$'
        && c != '+'
        && c != '-'
        && c != '`'
        && c != '%'
        && c != '"'
        && c != '\''
        && c != ','
}

fn parse_unknown(input: &str) -> IResult<&str, Sign> {
    let parser = satisfy(sign_condition);
    map(parser, |c: char| Sign::Unknown(c))(input)
}

#[cfg(test)]
mod tests {
    use crate::template_parser::structs::sign::Sign;

    #[test]
    fn sign_parser_spec_001() {
        let template = "*";
        let (_, parsed) = Sign::parse(template).unwrap();
        assert_eq!(parsed, Sign::Star);

        let template = ";";
        let (_, parsed) = Sign::parse(template).unwrap();
        assert_eq!(parsed, Sign::Semicolon);

    }
    #[test]
    fn sign_parser_spec_002() {
        let template = "s1%$@#\"'+-{}[]()`,";
        for c in template.chars() {
            let content = c.to_string();
            let parse_result = Sign::parse(content.as_str());
            assert!(parse_result.is_err());
        }
    }
}
