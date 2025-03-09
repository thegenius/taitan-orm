use crate::template_parser::structs::number::Number;
use crate::template_parser::structs::text::Text;
use nom::branch::alt;
use nom::bytes::complete::take_while1;
use nom::character::complete::multispace0;
use nom::combinator::map;
use nom::IResult;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Segment {
    Number(Number),
    Text(Text),
    Unknown(String),
}

impl Segment {
    pub fn parse(input: &str) -> IResult<&str, Segment> {
        let (input, _) = multispace0(input)?; // 跳过前导空格
        alt((
            map(Number::parse, Segment::Number),
            map(Text::parse, Segment::Text),
            map(parse_unknown, Segment::Unknown),
        ))(input)
    }
}

fn parse_unknown(input: &str) -> IResult<&str, String> {
    let (input, unknown) = take_while1(|c: char| !c.is_whitespace())(input)?;
    Ok((input, unknown.to_string()))
}
