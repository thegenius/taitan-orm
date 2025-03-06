
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{alphanumeric1, multispace0, space0},
    combinator::{map, opt, recognize, value},
    multi::{many0},
    sequence::{delimited, pair, preceded, terminated, tuple},
    IResult,
};
use nom::character::complete::alpha1;
use nom::character::streaming::digit1;
use nom::error::Error;
use nom::multi::{many1, separated_list0};

#[derive(Debug, PartialEq)]
enum SqlPart {
    Text(String), // 普通文本
    Expr(Expr),   // 表达式
    Template(TemplatePart), // 模板部分
}

#[derive(Debug, PartialEq)]
enum TemplatePart {
    Text(String), // 普通文本
    Variable(String, Option<Vec<String>>),   // 变量及其过滤器
    Expression(String, Option<Vec<String>>), // 表达式及其过滤器
    ControlBlock(String, String),            // 控制块
    Call(String),                            // call 语句
    Comment(String),                         // 注释块
}

#[derive(Debug, PartialEq)]
enum Expr {
    Placeholder(String, String), // (type, name)
    BinaryOp {
        left: Box<Expr>,
        op: String,
        right: Box<Expr>,
    },
    Literal(String),
    FunctionCall {
        name: String,
        args: Vec<Expr>,
    },
    Nested(Box<Expr>), // 嵌套表达式
    Not(Box<Expr>),    // NOT 表达式
    ExprPair(ExprPair), // 表达式对
}

#[derive(Debug, PartialEq)]
enum ExprPair {
    ExprAnd { left: Box<Expr>, right: Box<Expr> }, // AND 连接的表达式
    ExprOr { left: Box<Expr>, right: Box<Expr> },  // OR 连接的表达式
    ExprComma { left: Box<Expr>, right: Box<Expr> }, // 逗号连接的表达式
}

fn parse_not_expr(input: &str) -> IResult<&str, Expr> {
    let (input, _) = tag("NOT")(input)?;
    let (input, expr) = preceded(multispace0, parse_atomic_expr)(input)?;
    Ok((input, Expr::Not(Box::new(expr))))
}

fn parse_placeholder(input: &str) -> IResult<&str, Expr> {
    let (input, placeholder_type) = alt((tag("${"), tag("#{"), tag("@{")))(input)?;
    let (input, name) = alphanumeric1(input)?;
    let (input, _) = tag("}")(input)?;

    let placeholder_type = match placeholder_type {
        "${" => "${",
        "#{" => "#{",
        "@{" => "@{",
        _ => unreachable!(),
    };

    Ok((input, Expr::Placeholder(placeholder_type.to_string(), name.to_string())))
}

fn parse_literal(input: &str) -> IResult<&str, Expr> {
    // 解析标识符（字母、数字、下划线）
    let parse_identifier = recognize(many1(alt((
        alphanumeric1, // 字母或数字
        tag("_"),      // 下划线
    ))));

    // 解析数字字面量
    let parse_number = recognize(many1(digit1));

    // 解析字符串字面量（用双引号包裹）
    let parse_string = delimited(
        tag("\""),
        take_until("\""),
        tag("\""),
    );

    // 尝试解析标识符、数字或字符串
    let (input, literal) = alt((parse_identifier, parse_number, parse_string))(input)?;

    Ok((input, Expr::Literal(literal.to_string())))
}
fn parse_function_call(input: &str) -> IResult<&str, Expr> {
    let (input, name) = alpha1(input)?; // 函数名
    let (input, _) = tag("(")(input)?;
    let (input, args) = separated_list0(tag(","), preceded(multispace0, parse_expr))(input)?; // 参数列表
    let (input, _) = tag(")")(input)?;

    Ok((input, Expr::FunctionCall {
        name: name.to_string(),
        args,
    }))
}
fn parse_nested_expr(input: &str) -> IResult<&str, Expr> {
    delimited(
        tag("("),
        parse_comma_expr, // 嵌套表达式可以包含逗号表达式
        tag(")"),
    )(input)
}
fn parse_atomic_expr(input: &str) -> IResult<&str, Expr> {
    alt((
        parse_placeholder,
        parse_literal,
        parse_function_call,
        parse_not_expr,
        parse_nested_expr,
    ))(input)
}

fn parse_and_expr(input: &str) -> IResult<&str, Expr> {
    let (input, first_expr) = parse_atomic_expr(input)?; // 解析原子表达式
    let (input, rest_exprs) = many0(preceded(
        preceded(multispace0, tag("AND")),
        preceded(multispace0, parse_atomic_expr),
    ))(input)?;

    let folded_expr = rest_exprs.into_iter().fold(first_expr, |left, right| {
        Expr::ExprPair(ExprPair::ExprAnd {
            left: Box::new(left),
            right: Box::new(right),
        })
    });

    Ok((input, folded_expr))
}

fn parse_or_expr(input: &str) -> IResult<&str, Expr> {
    let (input, first_expr) = parse_and_expr(input)?; // 解析 AND 表达式
    let (input, rest_exprs) = many0(preceded(
        preceded(multispace0, tag("OR")),
        preceded(multispace0, parse_and_expr),
    ))(input)?;

    let folded_expr = rest_exprs.into_iter().fold(first_expr, |left, right| {
        Expr::ExprPair(ExprPair::ExprOr {
            left: Box::new(left),
            right: Box::new(right),
        })
    });

    Ok((input, folded_expr))
}

fn parse_comma_expr(input: &str) -> IResult<&str, Expr> {
    let (input, first_expr) = parse_or_expr(input)?; // 解析 OR 表达式
    let (input, rest_exprs) = many0(preceded(
        preceded(multispace0, tag(",")),
        preceded(multispace0, parse_or_expr),
    ))(input)?;

    let folded_expr = rest_exprs.into_iter().fold(first_expr, |left, right| {
        Expr::ExprPair(ExprPair::ExprComma {
            left: Box::new(left),
            right: Box::new(right),
        })
    });

    Ok((input, folded_expr))
}

fn parse_expr(input: &str) -> IResult<&str, Expr> {
    parse_comma_expr(input)
}


fn parse_variable(input: &str) -> IResult<&str, TemplatePart> {
    let (input, _) = tag("{{")(input)?;
    let (input, var) = take_until("}}")(input)?;
    let (input, filters) = opt(parse_filters)(input)?;
    let (input, _) = tag("}}")(input)?;

    Ok((
        input,
        TemplatePart::Variable(var.trim().to_string(), filters),
    ))
}

fn parse_expression(input: &str) -> IResult<&str, TemplatePart> {
    let (input, _) = tag("{{")(input)?;
    let (input, expr) = take_until("}}")(input)?;
    let (input, filters) = opt(parse_filters)(input)?;
    let (input, _) = tag("}}")(input)?;

    Ok((
        input,
        TemplatePart::Expression(expr.trim().to_string(), filters),
    ))
}

fn parse_filter(input: &str) -> IResult<&str, String> {
    let (input, filter_name) = alphanumeric1(input)?;
    Ok((input, filter_name.to_string()))
}

fn parse_filters(input: &str) -> IResult<&str, Vec<String>> {
    many0(delimited(tag("|"), parse_filter, multispace0))(input)
}

fn parse_control_block(input: &str) -> IResult<&str, TemplatePart> {
    let (input, _) = tag("{% ")(input)?;
    let (input, block_name) = alphanumeric1(input)?;
    let (input, _) = tag(" %}")(input)?;

    // 使用 take_until 找到结束标记
    let end_tag = format!("{{% end{} %}}", block_name);
    let (input, content) = take_until(end_tag.as_str())(input)?;
    let (input, _) = tag(end_tag.as_str())(input)?;
    Ok((
        input,
        TemplatePart::ControlBlock(block_name.to_string(), content.trim().to_string()),
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

fn parse_text_template(input: &str) -> IResult<&str, TemplatePart> {
    let (input, text) = take_until("{{")(input)
        .or_else(|_: nom::Err<Error<&str>>| take_until("{%")(input))
        .or_else(|_: nom::Err<Error<&str>>| take_until("{#")(input))?;
    Ok((input, TemplatePart::Text(text.to_string())))
}

fn parse_template_part(input: &str) -> IResult<&str, TemplatePart> {
    // 使用 multispace0 忽略前后的空白字符
    let mut parser = delimited(multispace0, alt((
        map(parse_variable, |part| part),
        map(parse_expression, |part| part),
        map(parse_control_block, |part| part),
        map(parse_call, |part| part),
        map(parse_comment, |part| part),
        map(parse_text_template, |part| part), // 解析模板中的普通文本
    )), multispace0);

    parser(input)
}

fn parse_text(input: &str) -> IResult<&str, String> {
    // 解析普通文本，直到遇到模板语法或表达式语法
    let (input, text) = take_until("{{")(input)
        .or_else(|_: nom::Err<Error<&str>>| take_until("{%")(input))
        .or_else(|_: nom::Err<Error<&str>>| take_until("{#")(input))
        .or_else(|_: nom::Err<Error<&str>>| take_until("${")(input)) // 如果支持占位符语法
        .or_else(|_: nom::Err<Error<&str>>| take_until("#{")(input)) // 如果支持占位符语法
        .or_else(|_: nom::Err<Error<&str>>| take_until("@{")(input)) // 如果支持占位符语法
        .or_else(|_: nom::Err<Error<&str>>| Ok(("", input)))?; // 如果没有模板语法，则解析剩余全部内容

    // 返回解析出的文本
    Ok((input, text.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;


}