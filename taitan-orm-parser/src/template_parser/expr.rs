use crate::template_parser::segment::Segment;
use crate::template_parser::simple_expr::SimpleExpr;
use crate::template_parser::structs::atomic::Atomic;
use crate::template_parser::structs::binary_op::BinaryOp;
use crate::template_parser::structs::placeholder::Placeholder;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, multispace0};
use nom::combinator::map;
use nom::multi::{many0, separated_list0};
use nom::sequence::{delimited, preceded};
use nom::IResult;
// Keyword{val: String, is_mysql: bool, is_postgres: bool, is_sqlite: bool}
//
//

// Variable:
//    SimpleName : 字母开头的任意字符串，不能是关键字，不包含空格
//    QuoteName  : PG中使用双引号"",MySql使用``,Sqlite中使用""或``或[]，然后以字母开头，不包含空格

// VariableChain : 用.连接起来的Variable

// Number
// Text:
//    SingleQuote
//    DoubleQuote
// Placeholder:
//    Hash.  : #{VariableChain}
//    At     : @{VariableChain}
//    Dollar : ${VariableChain}

// BinaryOp: in like > >= < <= = <> !=

// Sign: * ( ) [ ] + - /

// Atomic:
//    Text
//    Number
//    VariableChain
//    Placeholder
//    BinaryOp
//    Sign

// simple expr: atomic op atomic
// not       expr: NOT expr
// and       expr: expr_l AND expr_r
// or        expr: expr_l OR  expr_r
// ,         expr: expr_l , expr_r
// nested    expr: (expr)
// func call expr: fn_name(expr)

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Atomic(Atomic),
    Simple(SimpleExpr),
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

#[derive(Debug, PartialEq, Clone)]
enum ExprPair {
    ExprAnd { left: Box<Expr>, right: Box<Expr> }, // AND 连接的表达式
    ExprOr { left: Box<Expr>, right: Box<Expr> },  // OR 连接的表达式
    ExprComma { left: Box<Expr>, right: Box<Expr> }, // 逗号连接的表达式
}

impl Expr {
    pub fn parse(input: &str) -> IResult<&str, Expr> {
        parse_expr(input)
    }
}

fn parse_atomic_expr(input: &str) -> IResult<&str, Expr> {
    alt((
        map(Atomic::parse, Expr::Atomic),
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
        alt((map(SimpleExpr::parse, Expr::Simple), parse_expr)), // 嵌套表达式可以包含逗号表达式
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
    alt((parse_comma_expr, parse_binary_expr, parse_atomic_expr))(input)
}
