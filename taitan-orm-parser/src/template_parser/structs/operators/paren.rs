
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Paren {
    Left,
    Right,
}

impl Paren {
    pub fn parse(input: &str) -> nom::IResult<&str, Self> {
        alt((
            map(tag("("), |_| Self::Left),
            map(tag(")"), |_| Self::Right),
        ))(input)
    }
}

impl std::fmt::Display for Paren {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Paren::Left => write!(f, "("),
            Paren::Right => write!(f, ")"),
        }
    }
}
