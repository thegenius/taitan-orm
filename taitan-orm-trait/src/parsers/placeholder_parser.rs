use nom::bytes::complete::{take_while, take_while1};
use nom::character::complete::alphanumeric1;
use nom::combinator::recognize;
use nom::sequence::pair;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{alpha1, char},
    multi::many0,
    sequence::delimited,
    IResult,
};
use tracing::debug;

fn parse_placeholder_name(input: &str) -> IResult<&str, &str> {
    // 匹配以下划线或字母开头，后接字母、数字或下划线
    let name_parser = recognize(pair(
        alt((tag("_"), alpha1)),               // 以下划线或字母开头
        many0(alt((alphanumeric1, tag("_")))), // 后接字母、数字或下划线
    ));
    delimited(tag(":{"), name_parser, char('}'))(input)
}
fn parse_string(input: &str) -> IResult<&str, &str> {
    // 匹配单引号包裹的字符串
    let (remaining, content) = delimited(
        char('\''),
        take_while1(|c| c != '\''), // 匹配单引号之间的内容
        char('\''),
    )(input)?;
    debug!("parse_string: {}", content);
    Ok((remaining, content))
}

fn take_until_terminator(input: &str) -> IResult<&str, &str> {
    take_while(|c| !&[':', '\''].contains(&c))(input)
}
pub struct PlaceholderParser;

impl PlaceholderParser {
    pub fn parse_indexed(input: &str) -> (String, Vec<String>) {
        let mut names = Vec::new();
        let mut output = String::new();
        let mut remaining = input;

        let mut index = 0;
        while !remaining.is_empty() {
            if let Ok((rest, string_content)) = parse_string(remaining) {
                // 如果是字符串，直接将其添加到输出中
                output.push('\'');
                output.push_str(string_content);
                output.push('\'');
                remaining = rest;
            } else if let Ok((rest, name)) = parse_placeholder_name(remaining) {
                names.push(name.to_string());
                index = index + 1;
                let placeholder = format!("${}", index);
                output.push_str(placeholder.as_ref());
                remaining = rest;
            } else {
                match take_while::<_, _, nom::error::Error<_>>(|c| ![':','\''].contains(&c))(remaining) {
                    Ok((rest, text)) => {
                        // 如果找到 #，将 # 之前的内容添加到 output
                        output.push_str(text);
                        remaining = rest;
                    }
                    Err(_) => {
                        // 如果没有找到 #，将剩余内容全部添加到 output 并结束循环
                        output.push_str(remaining);
                        break; // 结束循环
                    }
                }
            }
        }
        (output, names)
    }

    pub fn parse(input: &str) -> (String, Vec<String>) {
        let mut names = Vec::new();
        let mut output = String::new();
        let mut remaining = input;

        while !remaining.is_empty() {
            if let Ok((rest, string_content)) = parse_string(remaining) {
                // 如果是字符串，直接将其添加到输出中
                output.push('\'');
                output.push_str(string_content);
                output.push('\'');
                remaining = rest;
            } else if let Ok((rest, name)) = parse_placeholder_name(remaining) {
                names.push(name.to_string());
                output.push('?');
                remaining = rest;
            } else {
                match take_while::<_, _, nom::error::Error<_>>(|c| ![':','\''].contains(&c))(remaining) {
                    Ok((rest, text)) => {
                        // 如果找到 #，将 # 之前的内容添加到 output
                        output.push_str(text);
                        remaining = rest;
                    }
                    Err(_) => {
                        // 如果没有找到 #，将剩余内容全部添加到 output 并结束循环
                        output.push_str(remaining);
                        break; // 结束循环
                    }
                }
            }
        }
        (output, names)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_string() {
        let content = "'hello :{}' test";
        let (remaining, string) = parse_string(content).unwrap();
        assert_eq!(remaining, " test");
        assert_eq!(string, "hello :{}");

        let content = "':{name}' test";
        let (remaining, string) = parse_string(content).unwrap();
        assert_eq!(remaining, " test");
        assert_eq!(string, ":{name}");
    }
}