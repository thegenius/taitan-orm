use nom::branch::alt;
use nom::bytes::complete::take_until;
use nom::character::complete::{char, multispace0};
use nom::combinator::cut;
use nom::error::context;
use nom::sequence::{preceded, terminated};
use nom::IResult;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Text {
    SingleQuote(String),
    DoubleQuote(String),
}
impl Text {
    pub fn parse(input: &str) -> IResult<&str, Text> {
        context(
            "text",
            alt((
                parse_single_quote_text,
                // parse_double_quote_text
            )),
        )(input)
    }
}

impl Display for Text {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Text::SingleQuote(text) => write!(fmt, "'{}'", text),
            Text::DoubleQuote(text) => write!(fmt, "\"{}\"", text),
        }
    }
}

fn parse_single_quote_text(input: &str) -> IResult<&str, Text> {
    let (remaining, parsed) = preceded(
        preceded(multispace0, char('\'')),
        cut(terminated(take_until("'"), char('\''))),
    )(input)?;
    let value = Text::SingleQuote(format!("'{}'", parsed));
    Ok((remaining, value))
}

fn parse_double_quote_text(input: &str) -> IResult<&str, Text> {
    let (remaining, parsed) = preceded(
        preceded(multispace0, char('"')),
        cut(terminated(take_until("\""), char('"'))),
    )(input)?;
    let value = Text::DoubleQuote(format!("\"{}\"", parsed));
    Ok((remaining, value))
}
