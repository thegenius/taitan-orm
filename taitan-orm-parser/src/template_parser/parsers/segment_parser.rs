
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alphanumeric1, multispace0};
use nom::error::ParseError;
use nom::sequence::preceded;
use nom::IResult;
use crate::template_parser::TemplateSqlValue;

pub fn parse_segment_as_value(input: &str) -> IResult<&str, TemplateSqlValue> {
    let (remaining, parsed) = parse_segment(input)?;
    Ok((remaining, TemplateSqlValue::Segment(parsed.to_string())))
}

pub fn parse_segment(input: &str) -> IResult<&str, String> {
    let (remaining, parsed) = alt((
        preceded(multispace0, tag("*")),
        preceded(multispace0, tag(",")),
        preceded(multispace0, tag(";")),
        preceded(multispace0, tag("?")),
        preceded(multispace0, tag("(")),
        preceded(multispace0, tag(")")),
    ))(input)?;
    Ok((remaining, parsed.to_string()))
}
