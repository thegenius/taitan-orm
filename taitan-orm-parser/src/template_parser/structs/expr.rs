// use crate::template_parser::segment::Segment;
use crate::template_parser::structs::atomic::Atomic;
use crate::template_parser::structs::binary_op::BinaryOp;
use crate::template_parser::structs::placeholder::Placeholder;
use crate::template_parser::structs::simple_expr::SimpleExpr;
use crate::template_parser::to_sql::{SqlSegment, ToSqlSegment};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, multispace0, multispace1};
use nom::combinator::map;
use nom::multi::{many0, separated_list0};
use nom::sequence::{delimited, preceded};
use nom::IResult;
use tracing::debug;
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

// simple expr: atomic operator atomic

// and       expr: expr_l AND expr_r
// or        expr: expr_l OR  expr_r
// ,         expr: expr_l ,   expr_r

// nested    expr: (expr)
// not       expr: NOT expr
// func call expr: fn_name(expr)

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
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
        debug!("Expr parse({})", input);
        parse_expr(input)
    }
}

impl ToSqlSegment for Expr {
    fn gen_sql_segments(&self) -> Vec<SqlSegment> {
        match self {
            Expr::Simple(expr) => expr.gen_sql_segments(),
            Expr::BinaryExpr { left, op, right } => {
                let left_segments = left.gen_sql_segments();
                let right_segments = right.gen_sql_segments();
                let mut segments =
                    Vec::with_capacity(left_segments.len() + right_segments.len() + 8);
                segments.extend(left_segments);
                segments.push(SqlSegment::Simple(op.to_string()));
                segments.extend(right_segments);
                segments
            }
            Expr::Nested(expr) => {
                let expr_segments = expr.gen_sql_segments();
                let mut segments = Vec::with_capacity(expr_segments.len() + 2);
                segments.push(SqlSegment::Simple("(".to_string()));
                segments.extend(expr_segments);
                segments.push(SqlSegment::Simple(")".to_string()));
                segments
            }
            Expr::Not(expr) => {
                let expr_segments = expr.gen_sql_segments();
                let mut segments = Vec::with_capacity(expr_segments.len() + 1);
                segments.push(SqlSegment::Simple("NOT".to_string()));
                segments.extend(expr_segments);
                segments
            }
            Expr::FunctionCall { name, args } => {
                let args_segments = args
                    .iter()
                    .map(|a| a.gen_sql_segments())
                    .flatten()
                    .collect::<Vec<_>>();
                let mut segments = Vec::with_capacity(args_segments.len() + 2);
                segments.push(SqlSegment::Simple(format!("{}(", name)));
                segments.extend(args_segments);
                segments.push(SqlSegment::Simple(")".to_string()));
                segments
            }
            Expr::ExprPair(expr_pair) => match expr_pair {
                ExprPair::ExprAnd { left, right }
                | ExprPair::ExprOr { left, right }
                | ExprPair::ExprComma { left, right } => {
                    let left_segments = left.gen_sql_segments();
                    let right_segments = right.gen_sql_segments();
                    let mut segments =
                        Vec::with_capacity(left_segments.len() + right_segments.len());
                    segments.extend(left_segments);
                    segments.extend(right_segments);
                    segments
                }
            },
        }
    }
}

// fn parse_atomic_expr(input: &str) -> IResult<&str, Expr> {
//     alt((
//         parse_function_call,
//         parse_not_expr,
//         parse_nested_expr,
//         map(Atomic::parse, Expr::Atomic),
//     ))(input)
// }
// fn parse_binary_expr(input: &str) -> IResult<&str, Expr> {
//     let (input, left) = parse_atomic_expr(input)?; // 解析左操作数
//     let (input, op) = preceded(multispace0, BinaryOp::parse)(input)?; // 解析操作符
//     let (input, right) = preceded(multispace0, parse_atomic_expr)(input)?; // 解析右操作数
//
//     Ok((
//         input,
//         Expr::BinaryExpr {
//             left: Box::new(left),
//             op,
//             right: Box::new(right),
//         },
//     ))
// }

fn parse_not_expr(input: &str) -> IResult<&str, Expr> {
    let (input, _) = tag("NOT")(input)?;
    let (input, expr) = preceded(
        multispace1,
        alt((map(SimpleExpr::parse, Expr::Simple), parse_expr)),
    )(input)?;
    Ok((input, Expr::Not(Box::new(expr))))
}

fn parse_function_call(input: &str) -> IResult<&str, Expr> {
    let (input, name) = alpha1(input)?; // 函数名
    let (input, _) = tag("(")(input)?;
    let (input, args) = separated_list0(tag(","), preceded(multispace0, parse_expr))(input)?; // 参数列表
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
    delimited(tag("("), parse_expr, tag(")"))(input)
}

fn parse_and_expr(input: &str) -> IResult<&str, Expr> {
    let (input, first_expr) = parse_expr(input)?;
    let (input, rest_exprs) = many0(preceded(
        preceded(multispace1, tag("AND")),
        preceded(multispace1, parse_expr),
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
        map(SimpleExpr::parse, Expr::Simple),
        parse_comma_expr,
        parse_not_expr,
        parse_nested_expr,
        parse_function_call,
    ))(input)
}
