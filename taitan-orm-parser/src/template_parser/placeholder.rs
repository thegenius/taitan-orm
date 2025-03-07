use crate::template_parser::variable::{VariableChain};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::IResult;

#[derive(Debug, PartialEq, Clone)]
pub enum Placeholder {
    Dollar(VariableChain),
    Hash(VariableChain),
    At(VariableChain),
}
impl Placeholder {
    pub fn parse(input: &str) -> IResult<&str, Placeholder> {
        let (input, placeholder_type) = alt((tag("${"), tag("#{"), tag("@{")))(input)?;
        let (input, name) = VariableChain::parse(input)?;
        let (input, _) = tag("}")(input)?;

        let placeholder = match placeholder_type {
            "${" => Placeholder::Dollar(name),
            "#{" => Placeholder::Hash(name),
            "@{" => Placeholder::At(name),
            _ => unreachable!(),
        };
        Ok((input, placeholder))
    }
}

// pub fn parse_placeholder(input: &str) -> IResult<&str, Placeholder> {
//     let (input, placeholder_type) = alt((tag("${"), tag("#{"), tag("@{")))(input)?;
//     let (input, name) = parse_variable_chain(input)?;
//     let (input, _) = tag("}")(input)?;
//
//     let placeholder = match placeholder_type {
//         "${" => Placeholder::Dollar(name),
//         "#{" => Placeholder::Hash(name),
//         "@{" => Placeholder::At(name),
//         _ => unreachable!(),
//     };
//     Ok((input, placeholder))
// }
