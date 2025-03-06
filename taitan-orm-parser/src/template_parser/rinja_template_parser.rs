use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{alphanumeric1, multispace0, space0},
    combinator::{map, opt, recognize, value},
    multi::{many0},
    sequence::{delimited, pair, preceded, terminated, tuple},
    IResult,
};

#[derive(Debug, PartialEq)]
enum TemplatePart {
    Expression(String), // 表达式及其过滤器
    ControlBlock(String, String, String),            // 控制块
    Call(String),                            // call 语句
    Comment(String),                         // 注释块
}


fn parse_expression(input: &str) -> IResult<&str, TemplatePart> {
    let (input, _) = tag("{{")(input)?;
    let (input, expr) = take_until("}}")(input)?;
    let (input, _) = tag("}}")(input)?;

    Ok((
        input,
        TemplatePart::Expression(expr.trim().to_string()),
    ))
}

fn parse_control_block(input: &str) -> IResult<&str, TemplatePart> {
    let (input, _) = tag("{% ")(input)?;
    let (input, block_name) = alphanumeric1(input)?;
    let (input, control_content) = take_until(" %}")(input)?;
    let (input, _) = tag(" %}")(input)?;

    // 使用 take_until 找到结束标记
    let end_tag = format!("{{% end{} %}}", block_name);
    let (input, content) = take_until(end_tag.as_str())(input)?;
    let (input, _) = tag(end_tag.as_str())(input)?;
    Ok((
        input,
        TemplatePart::ControlBlock(block_name.to_string(), control_content.trim().to_string(), content.trim().to_string()),
    ))
}

fn parse_call(input: &str) -> IResult<&str, TemplatePart> {
    let (input, _) = tag("{% call ")(input)?;
    let (input, call_name) = alphanumeric1(input)?;
    let (input, _) = tag(" %}")(input)?;

    Ok((input, TemplatePart::Call(call_name.to_string())))
}

fn parse_comment(input: &str) -> IResult<&str, TemplatePart> {
    let (input, _) = tag("{#")(input)?;
    let (input, comment) = take_until("#}")(input)?;
    let (input, _) = tag("#}")(input)?;

    Ok((input, TemplatePart::Comment(comment.to_string())))
}

fn parse_template_part(input: &str) -> IResult<&str, TemplatePart> {
    // 使用 multispace0 忽略前后的空白字符
    let mut parser = delimited(multispace0, alt((
        map(parse_expression, |part| part),
        map(parse_control_block, |part| part),
        map(parse_call, |part| part),
        map(parse_comment, |part| part),
    )), multispace0);

    parser(input)
}

fn parse_template(input: &str) -> IResult<&str, Vec<TemplatePart>> {
    many0(parse_template_part)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_template_part() {
        let template = r#"{{ name | capitalize }}"#;
        let (remaining, parsed_template_parts) = parse_template_part(template).unwrap();
        let expected = TemplatePart::Expression("name | capitalize".to_string());
        assert_eq!(parsed_template_parts, expected);

        let template = r#"{% if active %}
        You are active.
    {% endif %}"#;
        let (remaining, parsed_template_parts) = parse_template_part(template).unwrap();
        let expected = TemplatePart::ControlBlock("if".to_string(), "active".to_string(), "You are active.".to_string());
        assert_eq!(parsed_template_parts, expected);


        let template = r#"
    {% for item in items %}
        Item: {{ item | upper }}
    {% endfor %}"#;
        let (remaining, parsed_template_parts) = parse_template_part(template).unwrap();
        let expected = TemplatePart::ControlBlock("for".to_string(), "item in items".to_string(), "Item: {{ item | upper }}".to_string());
        assert_eq!(parsed_template_parts, expected);

        let template = r#"{% call macro %}"#;
        let (remaining, parsed_template_parts) = parse_template_part(template).unwrap();
        let expected = TemplatePart::Call("macro".to_string());
        assert_eq!(parsed_template_parts, expected);

        let template = r#"
    {# This is a comment #}"#;
        let (remaining, parsed_template_parts) = parse_template_part(template).unwrap();
        let expected = TemplatePart::Comment(" This is a comment ".to_string());
        assert_eq!(parsed_template_parts, expected);
    }
}