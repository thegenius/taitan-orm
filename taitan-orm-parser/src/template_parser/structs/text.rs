use nom::branch::alt;
use nom::bytes::complete::take_until;
use nom::character::complete::{char, multispace0};
use nom::combinator::cut;
use nom::error::context;
use nom::IResult;
use nom::sequence::{preceded, terminated};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Text {
    SingleQuote(String),
    DoubleQuote(String),
}
impl Text {
    pub fn parse(input: &str) -> IResult<&str, Text> {
        context(
            "text",
            alt((parse_single_quote_text, parse_double_quote_text)),
        )(input)
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