
use nom::bytes::complete::tag_no_case;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::multispace0,
    combinator::map,
    sequence::preceded,
    IResult,
};
use crate::template_parser::{TemplateConnective, TemplateSqlValue};

pub fn parse_connective(input: &str) -> IResult<&str, TemplateConnective> {
    let (remaining, parsed) = alt((
        map(preceded(multispace0, tag(",")), |s: &str| {
            TemplateConnective::Comma(s.to_string())
        }),
        map(preceded(multispace0, tag_no_case("AND")), |s: &str| {
            TemplateConnective::And(s.to_string())
        }),
        map(preceded(multispace0, tag_no_case("OR")), |s: &str| {
            TemplateConnective::Or(s.to_string())
        }),
    ))(input)?;
    Ok((remaining, parsed))
}

pub fn parse_connective_as_value(input: &str) -> IResult<&str, TemplateSqlValue> {
    let (remaining, parsed) = parse_connective(input)?;
    Ok((remaining, TemplateSqlValue::Connective(parsed)))
}
