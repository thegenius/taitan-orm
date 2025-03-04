use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::map,
    sequence::{preceded, tuple},
    IResult,
};
use nom::bytes::complete::tag_no_case;
use nom::character::complete::multispace0;
use crate::template_parser::TemplateSqlValue;

pub fn parse_operator(
    input: &str,
) -> IResult<&str, String> {
    let (remaining, parsed) = alt((
        // 多字符操作符优先匹配，允许中间有空格
        map(
            tuple((
                preceded(multispace0, tag(">")),
                multispace0,
                preceded(multispace0, tag("=")),
            )),
            |_| ">= ".to_string(),
        ),
        map(
            tuple((
                preceded(multispace0, tag("<")),
                multispace0,
                preceded(multispace0, tag("=")),
            )),
            |_| "<= ".to_string(),
        ),
        map(
            tuple((
                preceded(multispace0, tag("<")),
                multispace0,
                preceded(multispace0, tag(">")),
            )),
            |_| "<> ".to_string(),
        ),
        map(
            tuple((
                preceded(multispace0, tag("!")),
                multispace0,
                preceded(multispace0, tag("=")),
            )),
            |_| "<> ".to_string(),
        ),

        // 单字符操作符
        map(preceded(multispace0, tag("=")), |s: &str| s.to_string()),
        map(preceded(multispace0, tag("<")), |s: &str| s.to_string()),
        map(preceded(multispace0, tag(">")), |s: &str| s.to_string()),
        // 确保单个符号也可以被解析
        map(preceded(multispace0, tag(">=")), |s: &str| s.to_string()),
        map(preceded(multispace0, tag("<=")), |s: &str| s.to_string()),
        map(preceded(multispace0, tag("!=")), |s: &str| "<>".to_string()),
        map(preceded(multispace0, tag("<>")), |s: &str| "<>".to_string()),
        map(preceded(multispace0, tag_no_case("like")), |_| "LIKE".to_string())
    ))(input)?;

    Ok((remaining, parsed.trim().to_string()))
}

pub fn parse_operator_as_value(
    input: &str,
) -> IResult<&str, TemplateSqlValue> {
    let (remaining, parsed) = parse_operator(input)?;
    Ok((remaining, TemplateSqlValue::Operator(parsed)))
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_parse_operator() {
        let (remaining, parsed) =
            parse_operator("> =").unwrap();
        assert_eq!(
            parsed,
            ">=".to_string()
        );

        let (remaining, parsed) =
            parse_operator("<=").unwrap();
        assert_eq!(
            parsed,
            "<=".to_string()
        );

        let (remaining, parsed) =
            parse_operator("<>").unwrap();
        assert_eq!(
            parsed,
            "<>".to_string()
        );

        let (remaining, parsed) =
            parse_operator("! =").unwrap();
        assert_eq!(
            parsed,
            "<>".to_string()
        );

        let (remaining, parsed) =
            parse_operator("lIkE").unwrap();
        assert_eq!(
            parsed,
            "LIKE".to_string()
        );
    }
}