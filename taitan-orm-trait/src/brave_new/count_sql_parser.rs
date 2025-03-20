use crate::brave_new::error::{TaitanOrmError, TemplateRenderError};
use nom::bytes::complete::tag_no_case;
use nom::character::complete::{multispace0, multispace1};
use nom::sequence::preceded;
use nom::IResult;

pub struct CountSqlParser;

impl CountSqlParser {
    pub fn replace(sql: &str) -> crate::brave_new::result::Result<String> {
        // 解析 SELECT 部分并替换为 COUNT(*)
        match Self::parse_select(sql) {
            Ok((remaining, _)) => Ok(format!("SELECT COUNT(*) FROM {}", remaining)),
            Err(err) => Err(TaitanOrmError::TemplateRenderError(TemplateRenderError(
                err.to_string(),
            ))),
        }
    }

    // 解析 SELECT 部分
    fn parse_select(input: &str) -> IResult<&str, &str> {
        let (input, _) = preceded(
            multispace0,
            tag_no_case("SELECT"), // 匹配 SELECT 或 select
        )(input)?;

        let (input, _) = multispace1(input)?; // 匹配 SELECT 后的空白
        let (input, _) = Self::take_until_case_insensitive("FROM")(input)?; // 匹配直到 FROM 的部分
        let (input, _) = tag_no_case("FROM")(input)?; // 匹配 FROM
        Ok((input, "")) // 返回剩余部分
    }

    // 实现大小写不敏感的 take_until
    fn take_until_case_insensitive(tag: &str) -> impl Fn(&str) -> IResult<&str, &str> + '_ {
        move |input: &str| {
            let tag_lower = tag.to_lowercase();
            let mut index = 0;

            // 遍历输入字符串，查找大小写不敏感的 tag
            while index + tag.len() <= input.len() {
                let slice = &input[index..index + tag.len()];
                if slice.to_lowercase() == tag_lower {
                    return Ok((&input[index..], &input[0..index]));
                }
                index += 1;
            }

            // 如果没有找到，返回整个输入
            Ok((&input[input.len()..], input))
        }
    }
}
