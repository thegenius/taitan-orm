use crate::template::MaybeValue;
use crate::template_parser::structs::bool_value::Bool;
use crate::template_parser::structs::keyword::{MySqlKeyword};
use crate::template_parser::structs::text::Text;
use crate::template_parser::to_sql::SqlSegment;
use crate::{Atomic, Number, Operator, Sign, ToSqlSegment};
use nom::branch::alt;
use nom::combinator::map;
use nom::IResult;
use tracing::debug;
use crate::template_parser::structs::atomics::atomic_trait::AtomicTrait;

#[derive(Debug, Clone, PartialEq)]
pub enum MySqlAtomic {
    Number(Number),
    Text(Text),
    Bool(Bool),
    Operator(Operator), // 各类操作符+-*/% like in = > < <> !=
    Maybe(MaybeValue),  // 可能是Number/Text/Bool/Operator
    Sign(Sign),         // 各种特殊符号，例如括号()[]{}等
    Keyword(MySqlKeyword),
}
impl AtomicTrait for MySqlAtomic {
    fn parse(input: &str) -> IResult<&str, MySqlAtomic> {
        debug!("PostgresAtomic parse({})", &input);
        let (remaining, parsed) = alt((
            map(Number::parse, MySqlAtomic::Number),
            map(Text::parse, MySqlAtomic::Text),
            map(Bool::parse, MySqlAtomic::Bool),
            map(Sign::parse, MySqlAtomic::Sign), // 需要保证+ - * 先被解析为Sign, + - 可能是number修饰符，也可能是算术操作符，*可能是算术操作符，也可能是星号
            map(Operator::parse, MySqlAtomic::Operator),
            map(MySqlKeyword::parse, MySqlAtomic::Keyword),
            map(MaybeValue::parse, MySqlAtomic::Maybe),
        ))(input)?;
        debug!("PostgresAtomic parse -> {:?}", &parsed);
        Ok((remaining, parsed))
    }
}

impl ToSqlSegment for MySqlAtomic {
    fn gen_sql_segment(&self) -> SqlSegment {
        match self {
            MySqlAtomic::Sign(s) => SqlSegment::Simple(s.to_string()),
            MySqlAtomic::Maybe(m) => {
                SqlSegment::Simple(m.gen_sql_segment().to_sql(false).to_string())
            }
            MySqlAtomic::Operator(b) => SqlSegment::Simple(b.to_string()),
            MySqlAtomic::Bool(b) => SqlSegment::Simple(b.to_string()),
            MySqlAtomic::Text(t) => SqlSegment::Simple(t.to_string()),
            MySqlAtomic::Number(n) => SqlSegment::Simple(n.to_string()),
            MySqlAtomic::Keyword(k) => SqlSegment::Simple(k.to_string()),
        }
    }
}
