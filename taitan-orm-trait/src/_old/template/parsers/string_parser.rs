use nom::branch::alt;
use nom::bytes::complete::escaped;
use nom::bytes::complete::{tag, take_till, take_until, take_while};
use nom::character::complete::{
    alpha0, alphanumeric0, alphanumeric1, char, multispace0, multispace1, one_of,
};
use nom::character::is_alphanumeric;
use nom::combinator::{cut, iterator, map, opt, value};
use nom::error::{context, ContextError, ParseError, VerboseError};
use nom::multi::separated_list1;
use nom::sequence::Tuple;
use nom::sequence::{delimited, terminated};
use nom::sequence::{preceded, tuple};
use nom::Compare;
use nom::InputLength;
use nom::InputTake;

use crate::template::{TemplateSqlValue, TemplateString};
use nom::IResult;

pub fn parse_string_as_value(
    input: &str,
) -> IResult<&str, TemplateSqlValue> {
    let (remaining, parsed) = parse_string(input)?;
    Ok((remaining, TemplateSqlValue::String(parsed)))
}

pub fn parse_string(
    input: &str,
) -> IResult<&str, TemplateString> {
    context(
        "string",
        alt((
            parse_single_quote_string,
            parse_double_quote_string,
        )),
    )(input)
}

pub fn parse_single_quote_string(
    input: &str,
) -> IResult<&str, TemplateString> {
    let (remaining, parsed) = preceded(
        preceded(multispace0, char('\'')),
        cut(terminated(take_until("'"), char('\''))),
    )(input)?;
    let value = TemplateString::SingleQuoteString(format!("'{}'", parsed));
    Ok((remaining, value))
}

pub fn parse_double_quote_string(
    input: &str,
) -> IResult<&str, TemplateString> {
    let (remaining, parsed) = preceded(
        preceded(multispace0, char('"')),
        cut(terminated(take_until("\""), char('"'))),
    )(input)?;
    let value = TemplateString::DoubleQuoteString(format!("\"{}\"", parsed));
    Ok((remaining, value))
}





#[cfg(test)]
mod test {
    use super::TemplateString;
    use super::*;
    use crate::template::parsed_template_sql::ParsedTemplateSql;
    use nom::error::ErrorKind;
    use crate::template::parsers::parse_string;

    #[test]
    pub fn test_parse_string() {
        let (remaining, parsed) =
            parse_string("'this is single string'").unwrap();
        assert_eq!(
            parsed,
            TemplateString::SingleQuoteString("'this is single string'".to_string())
        );
        let (remaining, parsed) =
            parse_string("\"this is double string\"").unwrap();
        assert_eq!(
            parsed,
            TemplateString::DoubleQuoteString("\"this is double string\"".to_string())
        );
    }
}