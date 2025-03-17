use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::satisfy;
use nom::combinator::map;
use nom::IResult;
use std::fmt::Display;

// not alpha,  it is variable
// not number, it is number
// not begin with "/[/`, if begin with "/[/`, it is variable
// not begin with ', if begin with ', it is text
// not begin with @/#/$, it is placeholder
// not begin with {, it is rinja template
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Sign {
    Star,
    Plus,
    Minus,
    Semicolon,
    Comma,
    // LeftParen,
    // RightParen,
    Bracket(char),
    Unknown(char),
}
impl Sign {
    pub fn parse(input: &str) -> IResult<&str, Sign> {
        alt((
            map(tag("*"), |_| Sign::Star),
            map(tag("+"), |_| Sign::Plus),
            map(tag("-"), |_| Sign::Minus),
            map(tag(","), |_| Sign::Comma),
            map(tag(";"), |_| Sign::Semicolon),
            // map(tag("("), |_| Sign::LeftParen),
            // map(tag(")"), |_| Sign::RightParen),
            map(tag("["), |_| Sign::Bracket('[')),
            map(tag("]"), |_| Sign::Bracket(']')),
            map(tag("{"), |_| Sign::Bracket('{')),
            map(tag("{"), |_| Sign::Bracket('}')),
            parse_unknown,
        ))(input)
    }
}

impl Display for Sign {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Sign::Star => write!(fmt, "*"),
            Sign::Plus => write!(fmt, "+"),
            Sign::Minus => write!(fmt, "-"),
            Sign::Comma => write!(fmt, ","),
            Sign::Semicolon => write!(fmt, ";"),
            // Sign::LeftParen => write!(fmt, "("),
            // Sign::RightParen => write!(fmt, ")"),
            Sign::Bracket(c) => write!(fmt, "{}", c),
            Sign::Unknown(c) => write!(fmt, "{}", c),
        }
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
        && c != '*'
        && c != '='
        && c != '+'
        && c != '-'
        && c != '>'
        && c != '<'
        && c != '!'
        && c != '`'
        && c != '"'
        && c != '\''
        && c != ','
        && c != ';'
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
        let template = "s1$@#\"'`";
        for c in template.chars() {
            let content = c.to_string();
            let parse_result = Sign::parse(content.as_str());
            assert!(parse_result.is_err());
        }
    }
}
