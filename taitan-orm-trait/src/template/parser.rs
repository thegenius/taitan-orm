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

use crate::template::template_value::TemplateValue;
use nom::IResult;

pub fn parse_back_quote_string<'a, E: ParseError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, TemplateValue, E> {
    let (remaining, parsed) = preceded(
        preceded(multispace0, char('`')),
        cut(terminated(take_until("`"), char('`'))),
    )(input)?;
    let value = TemplateValue::BackQuoteString(format!("`{}`", parsed));
    return Ok((remaining, value));
}

pub fn parse_single_quote_string<'a, E: ParseError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, TemplateValue, E> {
    let (remaining, parsed) = preceded(
        preceded(multispace0, char('\'')),
        cut(terminated(take_until("'"), char('\''))),
    )(input)?;
    let value = TemplateValue::SingleQuoteString(format!("'{}'", parsed));
    return Ok((remaining, value));
}

pub fn parse_double_quote_string<'a, E: ParseError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, TemplateValue, E> {
    let (remaining, parsed) = preceded(
        preceded(multispace0, char('"')),
        cut(terminated(take_until("\""), char('"'))),
    )(input)?;
    let value = TemplateValue::DoubleQuoteString(format!("\"{}\"", parsed));
    return Ok((remaining, value));
}

fn parse_string<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, TemplateValue, E> {
    context(
        "string",
        alt((
            parse_single_quote_string,
            parse_double_quote_string,
            parse_back_quote_string,
        )),
    )(input)
}

//pub fn parse_string(input: &str) -> IResult<&str, &str> {
//    alt((parse_single_quote_string, parse_double_quote_string))(input)
//}

pub fn parse_dot<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, &'a str, E> {
    delimited(multispace0, tag("."), multispace0)(input)
}
pub fn parse_variable_seg<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, &'a str, E> {
    context("variable_seg", preceded(multispace0, alphanumeric1))(input)
}

/*
pub fn check_following_dot<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, (), E> {
    if input.starts_with(".") {
        let error = VerboseError::from_char(input, '.');
        return Err(nom::Err::Failure(error));
    }
    return Ok((input, ()));
}
*/

pub fn parse_hash_variable<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, TemplateValue, E> {
    //let (remaining, parsed) = parse_variable_seg(input)?;
    //let first_seg: &str = parsed;

    let segs_result = delimited(
        preceded(multispace0, tag("#{")),
        separated_list1(parse_dot, parse_variable_seg),
        preceded(multispace0, tag("}")),
    )(input)?;
    let (remaining, parsed) = segs_result;
    let following_segs: Vec<&str> = parsed;
    let mut segs: Vec<&str> = Vec::new();
    //segs.push(first_seg);
    segs.extend(following_segs);
    let variable = segs.join(".");
    return Ok((remaining, TemplateValue::HashVariable(variable.to_string())));
}

pub fn parse_dollar_variable<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, TemplateValue, E> {
    //let (remaining, parsed) = parse_variable_seg(input)?;
    //let first_seg: &str = parsed;

    let segs_result = delimited(
        preceded(multispace0, tag("${")),
        separated_list1(parse_dot, parse_variable_seg),
        preceded(multispace0, tag("}")),
    )(input)?;
    let (remaining, parsed) = segs_result;
    let following_segs: Vec<&str> = parsed;
    let mut segs: Vec<&str> = Vec::new();
    //segs.push(first_seg);
    segs.extend(following_segs);
    let variable = segs.join(".");
    return Ok((remaining, TemplateValue::DollarVariable(variable.to_string())));
}

pub fn parse_segment<'a, E: ParseError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, TemplateValue, E> {
    let (remaining, parsed) = alt((
        preceded(multispace0, alphanumeric1),
        preceded(multispace0, tag("*")),
        preceded(multispace0, tag(",")),
        preceded(multispace0, tag(";")),
        preceded(multispace0, tag("=")),
        preceded(multispace0, tag("?")),
        preceded(multispace0, tag("<")),
        preceded(multispace0, tag(">")),
        preceded(multispace0, tag("%")),
        preceded(multispace0, tag("(")),
        preceded(multispace0, tag(")")),
    ))(input)?;
    return Ok((remaining, TemplateValue::Segment(parsed.to_string())));
}

/*
pub fn parse_variable(input: &str) -> IResult<&str, &str> {
    delimited(
        tuple((tag("#{"), multispace0)),
        parse_dot_variable,
        tuple((multispace0, tag("}"))),
    )(input)
}
*/

pub fn parse_template_value(input: &str) -> IResult<&str, TemplateValue> {
    alt((parse_dollar_variable, parse_hash_variable, parse_string, parse_segment))(input)
}
pub fn parse_template_sql(input: &str) -> IResult<&str, Vec<TemplateValue>> {
    let mut values: Vec<TemplateValue> = Vec::new();
    let (mut remaining, mut parsed) = parse_template_value(input)?;
    values.push(parsed);
    while !remaining.is_empty() {
        (remaining, parsed) = parse_template_value(remaining)?;
        values.push(parsed);
    }
    Ok((remaining, values))
}

#[cfg(test)]
mod test {

    use super::TemplateValue;
    use super::*;
    use crate::template::parsed_template_sql::ParsedTemplateSql;
    use nom::error::ErrorKind;

    #[test]
    pub fn test_string() {
        let (remaining, parsed) =
            parse_string::<(&str, ErrorKind)>("'this is single string'").unwrap();
        assert_eq!(
            parsed,
            TemplateValue::SingleQuoteString("'this is single string'".to_string())
        );
        let (remaining, parsed) =
            parse_string::<(&str, ErrorKind)>("\"this is double string\"").unwrap();
        assert_eq!(
            parsed,
            TemplateValue::DoubleQuoteString("\"this is double string\"".to_string())
        );
    }

    #[test]
    pub fn test_hash_variable() {
        let (remaining, parsed) =
            parse_hash_variable::<(&str, ErrorKind)>("#{ var1 . var2 }").unwrap();
        assert_eq!(parsed, TemplateValue::HashVariable("var1.var2".to_string()));
    }

    #[test]
    pub fn test_dollar_variable() {
        let (remaining, parsed) =
            parse_dollar_variable::<(&str, ErrorKind)>("${ var1 . var2 }").unwrap();
        assert_eq!(parsed, TemplateValue::DollarVariable("var1.var2".to_string()));
    }

    #[test]
    pub fn test_template() {
        let (remaining, parsed) =
            parse_template_sql("SELECT * `test` user #{v1. v2. v3} where id = 23").unwrap();
        let result_vec = vec![
            TemplateValue::Segment("SELECT".to_string()),
            TemplateValue::Segment("*".to_string()),
            TemplateValue::BackQuoteString("`test`".to_string()),
            TemplateValue::Segment("user".to_string()),
            TemplateValue::HashVariable("v1.v2.v3".to_string()),
            TemplateValue::Segment("where".to_string()),
            TemplateValue::Segment("id".to_string()),
            TemplateValue::Segment("=".to_string()),
            TemplateValue::Segment("23".to_string()),
        ];
        assert_eq!(parsed, result_vec);

        let parsed_sql = ParsedTemplateSql::new(parsed);

        assert_eq!(parsed_sql.sql, "SELECT * `test` user ? where id = 23");
        let variables = vec!["v1.v2.v3".to_string()];
        assert_eq!(parsed_sql.variables, variables);
    }
}
