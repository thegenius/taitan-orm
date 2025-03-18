use crate::field_mapper::{MYSQL_KEYWORDS, POSTGRES_KEYWORDS, SQLITE_KEYWORDS};
use nom::bytes::complete::tag_no_case;
use nom::character::complete::multispace0;
use nom::sequence::preceded;
use nom::IResult;
use std::fmt::Display;
use std::str::FromStr;

fn parse_keywords<'a, T>(input: &'a str, keywords: &'static [&'static str]) -> IResult<&'a str, T>
where
    T: From<&'static str>,
{
    for keyword in keywords {
        if let Ok((remaining, _)) = preceded(
            multispace0,
            tag_no_case::<_, _, nom::error::Error<&'a str>>(*keyword),
        )(input)
        {
            // 直接使用 keyword，因为它是 &'static str
            let parsed = T::from(*keyword);
            return Ok((remaining, parsed));
        }
    }
    // 如果没有匹配的关键字，返回错误
    Err(nom::Err::Error(nom::error::Error::new(
        input,
        nom::error::ErrorKind::Tag,
    )))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PostgresKeyword(pub(crate) &'static str);

impl From<&'static str> for PostgresKeyword {
    fn from(s: &'static str) -> Self {
        PostgresKeyword(s)
    }
}

impl PostgresKeyword {
    pub fn parse(input: &str) -> IResult<&str, PostgresKeyword> {
        parse_keywords(input, POSTGRES_KEYWORDS)
    }
}

impl Display for PostgresKeyword {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SqliteKeyword(pub(crate) &'static str);

impl From<&'static str> for SqliteKeyword {
    fn from(s: &'static str) -> Self {
        SqliteKeyword(s)
    }
}

impl SqliteKeyword {
    pub fn parse(input: &str) -> IResult<&str, SqliteKeyword> {
        parse_keywords(input, SQLITE_KEYWORDS)
    }
}

impl Display for SqliteKeyword {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MySqlKeyword(pub(crate) &'static str);

impl From<&'static str> for MySqlKeyword {
    fn from(s: &'static str) -> Self {
        MySqlKeyword(s)
    }
}

impl MySqlKeyword {
    pub fn parse(input: &str) -> IResult<&str, MySqlKeyword> {
        parse_keywords(input, MYSQL_KEYWORDS)
    }
}

impl Display for MySqlKeyword {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "{}", self.0)
    }
}

#[cfg(test)]
mod postgres_keyword_inline_tests {
    use crate::template_parser::structs::keyword::{MySqlKeyword, PostgresKeyword, SqliteKeyword};

    #[test]
    fn test_keyword_001() {
        let template = "Select";
        let (_, parsed) = PostgresKeyword::parse(template).unwrap();
        assert_eq!(parsed, PostgresKeyword("SELECT"));

        let (_, parsed) = MySqlKeyword::parse(template).unwrap();
        assert_eq!(parsed, MySqlKeyword("SELECT"));

        let (_, parsed) = SqliteKeyword::parse(template).unwrap();
        assert_eq!(parsed, SqliteKeyword("SELECT"));
    }
}
