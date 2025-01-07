use crate::template::parsers::variable_parser::parse_variable_chain;
use crate::template::template_value::{TemplatePlaceholder, TemplateSqlValue};
use crate::template::TemplateVariableChain;
use nom::character::complete::multispace0;
use nom::error::ParseError;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::{alpha1, alphanumeric1, space0},
    combinator::{map, recognize},
    multi::many0,
    sequence::{delimited, pair, preceded, tuple},
    IResult,
};

pub fn parse_placeholder_as_value(input: &str) -> IResult<&str, TemplateSqlValue> {
    let (remaining, parsed) = parse_placeholder(input)?;
    Ok((remaining, TemplateSqlValue::Placeholder(parsed)))
}

pub fn parse_placeholder(input: &str) -> IResult<&str, TemplatePlaceholder> {
    let (remaining, parsed) = alt((
        parse_dollar_placeholder,
        parse_hash_placeholder,
        parse_percent_placeholder,
    ))(input)?;
    Ok((remaining, parsed))
}

pub fn parse_dollar_placeholder(input: &str) -> IResult<&str, TemplatePlaceholder> {
    // 解析模板占位符，例如 %{name}
    let (remaining, parsed) = delimited(
        map(
            tuple((
                preceded(multispace0, tag("$")),
                multispace0,
                preceded(multispace0, tag("{")),
            )),
            |_| "%{ ".to_string(),
        ),
        preceded(multispace0, parse_variable_chain),
        preceded(multispace0, tag("}")),
    )(input)?;
    Ok((remaining, TemplatePlaceholder::Dollar(parsed)))
}

pub fn parse_hash_placeholder(input: &str) -> IResult<&str, TemplatePlaceholder> {
    // 解析模板占位符，例如 %{name}
    let (remaining, parsed) = delimited(
        map(
            tuple((
                preceded(multispace0, tag("#")),
                multispace0,
                preceded(multispace0, tag("{")),
            )),
            |_| "%{ ".to_string(),
        ),
        preceded(multispace0, parse_variable_chain),
        preceded(multispace0, tag("}")),
    )(input)?;
    Ok((remaining, TemplatePlaceholder::Hash(parsed)))
}

pub fn parse_percent_placeholder(input: &str) -> IResult<&str, TemplatePlaceholder> {
    // 解析模板占位符，例如 %{name}
    let (remaining, parsed) = delimited(
        map(
            tuple((
                preceded(multispace0, tag("%")),
                multispace0,
                preceded(multispace0, tag("{")),
            )),
            |_| "%{ ".to_string(),
        ),
        preceded(multispace0, parse_variable_chain),
        preceded(multispace0, tag("}")),
    )(input)?;
    Ok((remaining, TemplatePlaceholder::Percent(parsed)))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::template::parsers::parse_placeholder;
    use crate::template::template_value::TemplateVariable;

    #[test]
    pub fn test_parse_percent_placeholder() {
        let (remaining, parsed) = parse_placeholder("%  { sdf_d . sdf_sv_1 }").unwrap();
        assert_eq!(
            parsed,
            TemplatePlaceholder::Percent(TemplateVariableChain {
                variables: vec![
                    TemplateVariable::Simple("sdf_d".to_string()),
                    TemplateVariable::Simple("sdf_sv_1".to_string())
                ]
            })
        );
    }

    #[test]
    pub fn test_parse_dollar_placeholder() {
        let (remaining, parsed) = parse_placeholder("$  { sdf_d . sdf_sv_1 }").unwrap();
        assert_eq!(
            parsed,
            TemplatePlaceholder::Dollar(TemplateVariableChain {
                variables: vec![
                    TemplateVariable::Simple("sdf_d".to_string()),
                    TemplateVariable::Simple("sdf_sv_1".to_string())
                ]
            })
        );
    }

    #[test]
    pub fn test_parse_hash_placeholder() {
        let (remaining, parsed) = parse_placeholder("#  { sdf_d . sdf_sv_1 }").unwrap();
        assert_eq!(
            parsed,
            TemplatePlaceholder::Hash(TemplateVariableChain {
                variables: vec![
                    TemplateVariable::Simple("sdf_d".to_string()),
                    TemplateVariable::Simple("sdf_sv_1".to_string())
                ]
            })
        );
    }
}
