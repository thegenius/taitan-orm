use crate::template::MaybeValue;
use crate::template_parser::structs::bool_value::Bool;
use crate::template_parser::structs::keyword::PostgresKeyword;
use crate::template_parser::structs::text::Text;
use crate::template_parser::to_sql::SqlSegment;
use crate::{Atomic, Number, Operator, Sign, ToSqlSegment};
use nom::branch::alt;
use nom::combinator::map;
use nom::IResult;
use tracing::debug;
use crate::template_parser::structs::atomics::atomic_trait::AtomicTrait;

#[derive(Debug, Clone, PartialEq)]
pub enum PostgresAtomic {
    Number(Number),
    Text(Text),
    Bool(Bool),
    Operator(Operator), // 各类操作符+-*/% like in = > < <> !=
    Maybe(MaybeValue),  // 可能是Number/Text/Bool/Operator
    Sign(Sign),         // 各种特殊符号，例如括号()[]{}等
    Keyword(PostgresKeyword),
}
impl AtomicTrait for PostgresAtomic {
    fn parse(input: &str) -> IResult<&str, PostgresAtomic> {
        debug!("PostgresAtomic parse({})", &input);
        let (remaining, parsed) = alt((
            map(Number::parse, PostgresAtomic::Number),
            map(Text::parse, PostgresAtomic::Text),
            map(Bool::parse, PostgresAtomic::Bool),
            map(Sign::parse, PostgresAtomic::Sign), // 需要保证+ - * 先被解析为Sign, + - 可能是number修饰符，也可能是算术操作符，*可能是算术操作符，也可能是星号
            map(Operator::parse, PostgresAtomic::Operator),
            map(PostgresKeyword::parse, PostgresAtomic::Keyword),
            map(MaybeValue::parse, PostgresAtomic::Maybe),
        ))(input)?;
        debug!("PostgresAtomic parse -> {:?}", &parsed);
        Ok((remaining, parsed))
    }
}

impl ToSqlSegment for PostgresAtomic {
    fn gen_sql_segment(&self) -> SqlSegment {
        match self {
            PostgresAtomic::Sign(s) => SqlSegment::Simple(s.to_string()),
            PostgresAtomic::Maybe(m) => {
                SqlSegment::Simple(m.gen_sql_segment().to_sql(false).to_string())
            }
            PostgresAtomic::Operator(b) => SqlSegment::Simple(b.to_string()),
            PostgresAtomic::Bool(b) => SqlSegment::Simple(b.to_string()),
            PostgresAtomic::Text(t) => SqlSegment::Simple(t.to_string()),
            PostgresAtomic::Number(n) => SqlSegment::Simple(n.to_string()),
            PostgresAtomic::Keyword(k) => SqlSegment::Simple(k.to_string()),
        }
    }
}
