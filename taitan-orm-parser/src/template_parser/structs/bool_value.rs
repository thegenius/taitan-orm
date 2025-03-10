use nom::branch::alt;
use nom::bytes::complete::{tag_no_case};
use nom::combinator::map;
use nom::IResult;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BoolValue {
    True,
    False,
}
impl BoolValue {
    pub fn parse(input: &str) -> IResult<&str, BoolValue> {
        alt((
            map(tag_no_case("TRUE"), |_| BoolValue::True),
            map(tag_no_case("FALSE"), |_| BoolValue::False),
        ))(input)
    }
}