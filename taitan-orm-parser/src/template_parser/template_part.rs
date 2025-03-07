use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::multispace0,
    combinator::{map, opt},
    sequence::delimited,
    IResult,
};

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Modifier {
    Plus,
    Minus,
    Tilde,
}

#[derive(Debug, PartialEq, Clone)]
pub struct EndBlock {
    name: String,
    start_modifier: Option<Modifier>,
    end_modifier: Option<Modifier>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct StartBlock {
    name: String,
    start_modifier: Option<Modifier>,
    end_modifier: Option<Modifier>,
    expr: String,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TemplatePart {
    Expression(String), // {{ }} 表达式及其过滤器
    ControlBlock {
        start_block: StartBlock,
        content: String,
        end_block: EndBlock,
    }, // {% %} 控制块,包括macro
    Call(String),       // {% call %} call 语句
    Comment(String),    // {# #}注释块
}

impl TemplatePart {
    pub fn parse(input: &str) -> IResult<&str, TemplatePart> {
        let (input, part) = delimited(
            multispace0,
            alt((
                map(parse_call, |part| part),
                map(parse_comment, |part| part),
                map(parse_expression, |part| part),
                map(parse_control_block, |part| part),
            )),
            multispace0,
        )(input)?;
        Ok((input, part))
    }
}

fn parse_expression(input: &str) -> IResult<&str, TemplatePart> {
    let (input, _) = tag("{{")(input)?;
    let (input, expr) = take_until("}}")(input)?;
    let (input, _) = tag("}}")(input)?;
    Ok((input, TemplatePart::Expression(expr.trim().to_string())))
}

// 辅助函数：生成一个解析器，该解析器可以识别并消耗特定的结束标记
fn take_until_and_consume<F, R>(parser: F) -> impl Fn(&str) -> IResult<&str, String>
where
    F: Fn(&str) -> IResult<&str, R>,
{
    move |input: &str| {
        let mut buffer = String::new();
        let mut remaining = input;

        loop {
            match parser(remaining) {
                Ok((_, _)) => {
                    // 找到了结束标记，停止循环
                    break;
                }
                Err(_) => {
                    if remaining.is_empty() {
                        // 如果没有剩余输入且未找到结束标记，返回错误
                        return Err(nom::Err::Error(nom::error::Error::new(
                            input,
                            nom::error::ErrorKind::TakeUntil,
                        )));
                    }

                    // 将当前字符添加到缓冲区并继续
                    buffer.push(remaining.chars().next().unwrap());
                    remaining = &remaining[1..];
                }
            }
        }
        Ok((remaining, buffer))
    }
}
fn parse_modifier(input: &str) -> IResult<&str, Modifier> {
    let (input, modifier_str) = alt((tag("+"), tag("-"), tag("~")))(input)?;
    let modifier = match modifier_str {
        "+" => Modifier::Plus,
        "-" => Modifier::Minus,
        "~" => Modifier::Tilde,
        _ => unreachable!(),
    };
    Ok((input, modifier))
}

fn parse_start_brace(input: &str) -> IResult<&str, Option<Modifier>> {
    let (input, _) = tag("{%")(input)?;
    let (input, _) = multispace0(input)?; // 允许空格
    let (input, modifier) = opt(parse_modifier)(input)?;
    Ok((input, modifier))
}

fn parse_end_brace(input: &str) -> IResult<&str, Option<Modifier>> {
    let (input, modifier) = opt(parse_modifier)(input)?;
    let (input, _) = multispace0(input)?; // 允许空格
    let (input, _) = tag("%}")(input)?;
    Ok((input, modifier))
}

fn parse_start_block_with_name<'a>(
    input: &'a str,
    block_name: &'static str,
) -> IResult<&'a str, StartBlock> {
    let (input, start_modifier) = parse_start_brace(input)?;
    let (input, _) = multispace0(input)?; // 允许空格
    let (input, parsed_block_name) = tag(block_name)(input)?;
    let (input, _) = multispace0(input)?; // 允许空格
    let (input, expr) = take_until_and_consume(parse_end_brace)(input)?; // 表达式可以包含任意字符
    let (input, _) = multispace0(input)?; // 允许空格
    let (input, end_modifier) = parse_end_brace(input)?; // 解析可能的结束修饰符

    Ok((
        input,
        StartBlock {
            name: parsed_block_name.to_string(),
            start_modifier,
            end_modifier,
            expr: expr.trim().to_string(),
        },
    ))
}

fn parse_start_block(input: &str) -> IResult<&str, StartBlock> {
    alt((
        |i| parse_start_block_with_name(i, "if"),
        |i| parse_start_block_with_name(i, "match"),
        |i| parse_start_block_with_name(i, "for"),
        |i| parse_start_block_with_name(i, "macro"),
        |i| parse_start_block_with_name(i, "filter"),
        |i| parse_start_block_with_name(i, "block"),
    ))(input)
}

fn parse_end_block_with_name<'a>(
    input: &'a str,
    block_name: &'static str,
) -> IResult<&'a str, EndBlock> {
    let (input, start_modifier) = parse_start_brace(input)?;
    let (input, _) = multispace0(input)?; // 允许空格
    let (input, _) = tag(block_name)(input)?;
    let (input, _) = multispace0(input)?; // 允许空格
    let (input, end_modifier) = parse_end_brace(input)?; // 表达式可以包含任意字符
    Ok((
        input,
        EndBlock {
            name: block_name.to_string(),
            start_modifier,
            end_modifier,
        },
    ))
}

fn parse_end_block(input: &str) -> IResult<&str, EndBlock> {
    alt((
        |i| parse_end_block_with_name(i, "endif"),
        |i| parse_end_block_with_name(i, "endmatch"),
        |i| parse_end_block_with_name(i, "endfor"),
        |i| parse_end_block_with_name(i, "endmacro"),
        |i| parse_end_block_with_name(i, "endfilter"),
        |i| parse_end_block_with_name(i, "endblock"),
    ))(input)
}

fn parse_control_block(input: &str) -> IResult<&str, TemplatePart> {
    // 解析控制块开始标记
    let (input, start_block) = parse_start_block(input)?;

    let (input, control_content) = take_until_and_consume(parse_end_block)(input)?;

    let (input, end_block) = parse_end_block(input)?;

    Ok((
        input,
        TemplatePart::ControlBlock {
            start_block,
            content: control_content.trim().to_string(),
            end_block,
        },
    ))
}

fn parse_call(input: &str) -> IResult<&str, TemplatePart> {
    let (input, _) = tag("{% call ")(input)?;
    let (input, expr) = take_until("%}")(input)?;
    let (input, _) = tag("%}")(input)?;
    Ok((input, TemplatePart::Call(expr.trim().to_string())))
}

fn parse_comment(input: &str) -> IResult<&str, TemplatePart> {
    let (input, _) = tag("{#")(input)?;
    let (input, comment) = take_until("#}")(input)?;
    let (input, _) = tag("#}")(input)?;
    Ok((input, TemplatePart::Comment(comment.to_string())))
}

// fn parse_template_part(input: &str) -> IResult<&str, TemplatePart> {
//     let (input, part) = delimited(
//         multispace0,
//         alt((
//             map(parse_call, |part| part),
//             map(parse_comment, |part| part),
//             map(parse_expression, |part| part),
//             map(parse_control_block, |part| part),
//         )),
//         multispace0,
//     )(input)?;
//     Ok((input, part))
// }

// fn parse_template(input: &str) -> IResult<&str, Vec<TemplatePart>> {
//     many0(parse_template_part)(input)
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_end_brace() {
        let template = "%}";
        let (remaining, end_block) = parse_end_brace(template).unwrap();
        assert_eq!(end_block, None);

        let template = "- %}";
        let (remaining, end_block) = parse_end_brace(template).unwrap();
        assert_eq!(end_block, Some(Modifier::Minus));

        let template = "~ %}";
        let (remaining, end_block) = parse_end_brace(template).unwrap();
        assert_eq!(end_block, Some(Modifier::Tilde));
    }

    #[test]
    fn test_parse_start_block() {
        let template = r#"{% if active %}"#;
        let (remaining, parsed) = parse_start_block(template).unwrap();
        let expected = StartBlock {
            name: "if".to_string(),
            start_modifier: None,
            end_modifier: None,
            expr: "active".to_string(),
        };
        assert_eq!(expected, parsed);
        let template = r#"{%~ for active set as test -%}"#;
        let (remaining, parsed) = parse_start_block(template).unwrap();
        let expected = StartBlock {
            name: "for".to_string(),
            start_modifier: Some(Modifier::Tilde),
            end_modifier: Some(Modifier::Minus),
            expr: "active set as test".to_string(),
        };
        assert_eq!(expected, parsed);
    }

    #[test]
    fn test_parse_end_block() {
        let template = r#"{% endif %}"#;
        let (remaining, parsed) = parse_end_block(template).unwrap();
        assert_eq!(
            parsed,
            EndBlock {
                name: String::from("endif"),
                start_modifier: None,
                end_modifier: None
            }
        );

        let template = r#"{% if active %}
            You are active.
        {% endif -%}"#;
        let (remaining, _) = parse_start_block(template).unwrap();
        assert_eq!(
            remaining,
            "\n            You are active.\n        {% endif -%}"
        );
        let (remaining, parsed) = take_until_and_consume(parse_end_block)(remaining).unwrap();
        assert_eq!(parsed, "\n            You are active.\n        ");
    }

    #[test]
    fn test_parse_template_part() {
        let template = r#"{{ name | capitalize }}"#;
        let (remaining, parsed_template_parts) = TemplatePart::parse(template).unwrap();
        let expected = TemplatePart::Expression("name | capitalize".to_string());
        assert_eq!(parsed_template_parts, expected);

        let template = r#"{% if active %}
            You are active.
        {% endif -%}"#;
        let (remaining, parsed_template_parts) = TemplatePart::parse(template).unwrap();
        let expected = TemplatePart::ControlBlock {
            start_block: StartBlock {
                name: "if".to_string(),
                start_modifier: None,
                end_modifier: None,
                expr: "active".to_string(),
            },
            content: "You are active.".to_string(),
            end_block: EndBlock {
                name: "endif".to_string(),
                start_modifier: None,
                end_modifier: Some(Modifier::Minus),
            },
        };
        assert_eq!(parsed_template_parts, expected);

        let template = r#"
        {% for item in items %}
            Item: {{ item | upper }}
        {% endfor %}"#;
        let (remaining, parsed_template_parts) = TemplatePart::parse(template).unwrap();
        let expected = TemplatePart::ControlBlock {
            start_block: StartBlock {
                name: "for".to_string(),
                start_modifier: None,
                end_modifier: None,
                expr: "item in items".to_string(),
            },
            content: "Item: {{ item | upper }}".to_string(),
            end_block: EndBlock {
                name: "endfor".to_string(),
                start_modifier: None,
                end_modifier: None,
            },
        };
        assert_eq!(parsed_template_parts, expected);

        let template = r#"{% call macro %}"#;
        let (remaining, parsed_template_parts) = TemplatePart::parse(template).unwrap();
        let expected = TemplatePart::Call("macro".to_string());
        assert_eq!(parsed_template_parts, expected);

        let template = r#"
        {# This is a comment #}"#;
        let (remaining, parsed_template_parts) = TemplatePart::parse(template).unwrap();
        let expected = TemplatePart::Comment(" This is a comment ".to_string());
        assert_eq!(parsed_template_parts, expected);
    }
}
