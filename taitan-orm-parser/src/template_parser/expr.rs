use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, multispace0};
use nom::combinator::map;
use nom::IResult;
use nom::multi::{many0, separated_list0};
use nom::sequence::{delimited, preceded};
use crate::template_parser::binary_op::BinaryOp;
use crate::template_parser::placeholder::Placeholder;
use crate::template_parser::segment::Segment;

#[derive(Debug, PartialEq, Clone)]
enum ExprPair {
    ExprAnd { left: Box<Expr>, right: Box<Expr> }, // AND 连接的表达式
    ExprOr { left: Box<Expr>, right: Box<Expr> },  // OR 连接的表达式
    ExprComma { left: Box<Expr>, right: Box<Expr> }, // 逗号连接的表达式
}
#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Placeholder(Placeholder), // (type, name)，支持#{name} ${name} @{name}
    Segment(Segment),
    BinaryExpr {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
    },
    FunctionCall {
        name: String,
        args: Vec<Expr>,
    },
    Nested(Box<Expr>),  // ()嵌套表达式
    Not(Box<Expr>),     // NOT 表达式
    ExprPair(ExprPair), // 表达式对
}
impl Expr {
    pub fn parse(input: &str) -> IResult<&str, Expr> {
        parse_expr(input)
    }
}


fn parse_atomic_expr(input: &str) -> IResult<&str, Expr> {
    alt((
        map(Placeholder::parse, Expr::Placeholder),
        map(Segment::parse, Expr::Segment),
        parse_function_call,
        parse_not_expr,
        parse_nested_expr,
    ))(input)
}
fn parse_binary_expr(input: &str) -> IResult<&str, Expr> {
    let (input, left) = parse_atomic_expr(input)?; // 解析左操作数
    let (input, op) = preceded(multispace0, BinaryOp::parse)(input)?; // 解析操作符
    let (input, right) = preceded(multispace0, parse_atomic_expr)(input)?; // 解析右操作数

    Ok((
        input,
        Expr::BinaryExpr {
            left: Box::new(left),
            op,
            right: Box::new(right),
        },
    ))
}

fn parse_not_expr(input: &str) -> IResult<&str, Expr> {
    let (input, _) = tag("NOT")(input)?;
    let (input, expr) = preceded(multispace0, parse_atomic_expr)(input)?;
    Ok((input, Expr::Not(Box::new(expr))))
}

fn parse_function_call(input: &str) -> IResult<&str, Expr> {
    let (input, name) = alpha1(input)?; // 函数名
    let (input, _) = tag("(")(input)?;
    let (input, args) = separated_list0(tag(","), preceded(multispace0, parse_atomic_expr))(input)?; // 参数列表
    let (input, _) = tag(")")(input)?;
    Ok((
        input,
        Expr::FunctionCall {
            name: name.to_string(),
            args,
        },
    ))
}
fn parse_nested_expr(input: &str) -> IResult<&str, Expr> {
    delimited(
        tag("("),
        parse_expr, // 嵌套表达式可以包含逗号表达式
        tag(")"),
    )(input)
}

fn parse_and_expr(input: &str) -> IResult<&str, Expr> {
    let (input, first_expr) = parse_atomic_expr(input)?; // 解析原子表达式
    let (input, rest_exprs) = many0(preceded(
        preceded(multispace0, tag("AND")),
        preceded(multispace0, parse_binary_expr),
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
    alt((
        parse_comma_expr,
        parse_binary_expr,
        parse_atomic_expr,
    ))(input)
}
