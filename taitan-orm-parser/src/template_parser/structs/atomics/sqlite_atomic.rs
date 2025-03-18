use crate::template::MaybeValue;
use crate::template_parser::structs::bool_value::Bool;
use crate::template_parser::structs::keyword::SqliteKeyword;
use crate::template_parser::structs::text::Text;
use crate::template_parser::to_sql::SqlSegment;
use crate::{Atomic, Number, Operator, Sign, ToSqlSegment};
use nom::branch::alt;
use nom::combinator::map;
use nom::IResult;
use tracing::debug;
use crate::template_parser::structs::atomics::atomic_trait::AtomicTrait;

#[derive(Debug, Clone, PartialEq)]
pub enum SqliteAtomic {
    Number(Number),
    Text(Text),
    Bool(Bool),
    Operator(Operator), // 各类操作符+-*/% like in = > < <> !=
    Maybe(MaybeValue),  // 可能是Number/Text/Bool/Operator
    Sign(Sign),         // 各种特殊符号，例如括号()[]{}等
    Keyword(SqliteKeyword),
}
impl AtomicTrait for  SqliteAtomic {
    fn parse(input: &str) -> IResult<&str, SqliteAtomic> {
        debug!("PostgresAtomic parse({})", &input);
        let (remaining, parsed) = alt((
            map(Number::parse, SqliteAtomic::Number),
            map(Text::parse, SqliteAtomic::Text),
            map(Bool::parse, SqliteAtomic::Bool),
            map(Sign::parse, SqliteAtomic::Sign), // 需要保证+ - * 先被解析为Sign, + - 可能是number修饰符，也可能是算术操作符，*可能是算术操作符，也可能是星号
            map(Operator::parse, SqliteAtomic::Operator),
            map(SqliteKeyword::parse, SqliteAtomic::Keyword),
            map(MaybeValue::parse, SqliteAtomic::Maybe),
        ))(input)?;
        debug!("PostgresAtomic parse -> {:?}", &parsed);
        Ok((remaining, parsed))
    }
}

impl ToSqlSegment for SqliteAtomic {
    fn gen_sql_segment(&self) -> SqlSegment {
        match self {
            SqliteAtomic::Sign(s) => SqlSegment::Simple(s.to_string()),
            SqliteAtomic::Maybe(m) => {
                SqlSegment::Simple(m.gen_sql_segment().to_sql(false).to_string())
            }
            SqliteAtomic::Operator(b) => SqlSegment::Simple(b.to_string()),
            SqliteAtomic::Bool(b) => SqlSegment::Simple(b.to_string()),
            SqliteAtomic::Text(t) => SqlSegment::Simple(t.to_string()),
            SqliteAtomic::Number(n) => SqlSegment::Simple(n.to_string()),
            SqliteAtomic::Keyword(k) => SqlSegment::Simple(k.to_string()),
        }
    }
}
