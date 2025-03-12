use nom::branch::alt;
use nom::bytes::complete::{tag, tag_no_case};
use nom::character::complete::multispace0;
use nom::combinator::map;
use nom::sequence::{preceded, tuple};
use nom::IResult;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LogicOp {
    And,
    Or,
}

impl LogicOp {
    pub fn parse(input: &str) -> IResult<&str, LogicOp> {
        alt((
            map(preceded(multispace0, tag_no_case("and")), |_| LogicOp::And),
            map(preceded(multispace0, tag_no_case("or")), |_| LogicOp::Or),
        ))(input)
    }
}

impl Display for LogicOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogicOp::And => f.write_str("AND"),
            LogicOp::Or => f.write_str("OR"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArithmeticOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

impl ArithmeticOp {
    pub fn parse(input: &str) -> IResult<&str, ArithmeticOp> {
        alt((
            map(preceded(multispace0, tag("+")), |s: &str| ArithmeticOp::Add),
            map(preceded(multispace0, tag("-")), |s: &str| ArithmeticOp::Sub),
            map(preceded(multispace0, tag("*")), |s: &str| ArithmeticOp::Mul),
            map(preceded(multispace0, tag("/")), |s: &str| ArithmeticOp::Div),
            map(preceded(multispace0, tag("%")), |s: &str| ArithmeticOp::Mod),
        ))(input)
    }
}

impl Display for ArithmeticOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArithmeticOp::Add => f.write_str("+"),
            ArithmeticOp::Sub => f.write_str("-"),
            ArithmeticOp::Mul => f.write_str("*"),
            ArithmeticOp::Div => f.write_str("/"),
            ArithmeticOp::Mod => f.write_str("%"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ComparisonOp {
    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
}

impl ComparisonOp {
    pub fn parse(input: &str) -> IResult<&str, ComparisonOp> {
        let (remaining, parsed) = alt((
            // 多字符操作符优先匹配，允许中间有空格
            map(
                tuple((
                    preceded(multispace0, tag(">")),
                    multispace0,
                    preceded(multispace0, tag("=")),
                )),
                |_| ComparisonOp::GreaterThanOrEqual,
            ),
            map(
                tuple((
                    preceded(multispace0, tag("<")),
                    multispace0,
                    preceded(multispace0, tag("=")),
                )),
                |_| ComparisonOp::LessThanOrEqual,
            ),
            map(
                tuple((
                    preceded(multispace0, tag("<")),
                    multispace0,
                    preceded(multispace0, tag(">")),
                )),
                |_| ComparisonOp::NotEqual,
            ),
            map(
                tuple((
                    preceded(multispace0, tag("!")),
                    multispace0,
                    preceded(multispace0, tag("=")),
                )),
                |_| ComparisonOp::NotEqual,
            ),
            // 单字符操作符
            map(preceded(multispace0, tag("=")), |s: &str| {
                ComparisonOp::Equal
            }),
            map(preceded(multispace0, tag("<")), |s: &str| {
                ComparisonOp::LessThan
            }),
            map(preceded(multispace0, tag(">")), |s: &str| {
                ComparisonOp::GreaterThan
            }),
        ))(input)?;

        Ok((remaining, parsed))
    }
}

impl Display for ComparisonOp {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ComparisonOp::LessThan => write!(fmt, "<"),
            ComparisonOp::LessThanOrEqual => write!(fmt, "<="),
            ComparisonOp::GreaterThan => write!(fmt, ">"),
            ComparisonOp::GreaterThanOrEqual => write!(fmt, ">="),
            ComparisonOp::Equal => write!(fmt, "="),
            ComparisonOp::NotEqual => write!(fmt, "<>"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MatchOp {
    Like,
    In,
}

impl MatchOp {
    pub fn parse(input: &str) -> IResult<&str, MatchOp> {
        alt((
            map(preceded(multispace0, tag_no_case("like")), |_| {
                MatchOp::Like
            }),
            map(preceded(multispace0, tag_no_case("in")), |_| MatchOp::In),
        ))(input)
    }
}

impl Display for MatchOp {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MatchOp::Like => write!(fmt, "LIKE"),
            MatchOp::In => write!(fmt, "IN"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinaryOp {
    Compare(ComparisonOp),
    Arithmetic(ArithmeticOp),
    Match(MatchOp),
    Logic(LogicOp),
    Comma,
}

impl BinaryOp {
    pub fn parse(input: &str) -> IResult<&str, BinaryOp> {
        alt((
            map(ComparisonOp::parse, BinaryOp::Compare),
            map(ArithmeticOp::parse, BinaryOp::Arithmetic),
            map(MatchOp::parse, BinaryOp::Match),
            map(LogicOp::parse, BinaryOp::Logic),
            map(preceded(multispace0, tag_no_case(",")), |s: &str| {
                BinaryOp::Comma
            }),
        ))(input)
    }
    pub fn extract_and(&self) -> Option<BinaryOp> {
        if let BinaryOp::Logic(logic_op) = self {
            if logic_op.to_string() == "AND" {
                return Some(BinaryOp::Logic(LogicOp::And));
            }
        }
        None
    }
    pub fn extract_or(&self) -> Option<BinaryOp> {
        if let BinaryOp::Logic(logic_op) = self {
            if logic_op.to_string() == "OR" {
                return Some(BinaryOp::Logic(LogicOp::And));
            }
        }
        None
    }
    pub fn extract_comma(&self) -> Option<BinaryOp> {
        if let BinaryOp::Comma = self {
            return Some(BinaryOp::Comma);
        }
        None
    }
}

impl Display for BinaryOp {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BinaryOp::Arithmetic(a) => a.fmt(fmt),
            BinaryOp::Match(m) => m.fmt(fmt),
            BinaryOp::Logic(l) => l.fmt(fmt),
            BinaryOp::Compare(c) => c.fmt(fmt),
            BinaryOp::Comma => write!(fmt, ","),
        }
    }
}
