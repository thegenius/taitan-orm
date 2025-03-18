use crate::template::MaybeValue;
use crate::template_parser::structs::atomics::postgres_atomic::PostgresAtomic;
use crate::template_parser::structs::bool_value::Bool;
use crate::template_parser::structs::text::Text;
use crate::template_parser::to_sql::SqlSegment;
use crate::{Number, Operator, Sign, ToSqlSegment};
use crate::template_parser::structs::atomics::mysql_atomic::MySqlAtomic;
use crate::template_parser::structs::atomics::sqlite_atomic::SqliteAtomic;

#[derive(Debug, Clone, PartialEq)]
pub enum GenericAtomic {
    Number(Number),
    Text(Text),
    Bool(Bool),
    Operator(Operator), // 各类操作符+-*/% like in = > < <> !=
    Maybe(MaybeValue),  // 可能是Number/Text/Bool/Operator
    Sign(Sign),         // 各种特殊符号，例如括号()[]{}等
    Keyword(&'static str),
}

impl From<PostgresAtomic> for GenericAtomic {
    fn from(pg_atomic: PostgresAtomic) -> Self {
        match pg_atomic {
            PostgresAtomic::Number(n) => Self::Number(n),
            PostgresAtomic::Text(t) => Self::Text(t),
            PostgresAtomic::Bool(b) => Self::Bool(b),
            PostgresAtomic::Operator(o) => Self::Operator(o),
            PostgresAtomic::Maybe(m) => Self::Maybe(m),
            PostgresAtomic::Sign(s) => Self::Sign(s),
            PostgresAtomic::Keyword(k) => Self::Keyword(k.0),
        }
    }
}

impl From<MySqlAtomic> for GenericAtomic  {
    fn from(pg_atomic: MySqlAtomic) -> Self {
        match pg_atomic.into() {
            MySqlAtomic::Number(n) => Self::Number(n),
            MySqlAtomic::Text(t) => Self::Text(t),
            MySqlAtomic::Bool(b) => Self::Bool(b),
            MySqlAtomic::Operator(o) => Self::Operator(o),
            MySqlAtomic::Maybe(m) => Self::Maybe(m),
            MySqlAtomic::Sign(s) => Self::Sign(s),
            MySqlAtomic::Keyword(k) => Self::Keyword(k.0),
        }
    }
}

impl From<SqliteAtomic> for GenericAtomic {
    fn from(pg_atomic: SqliteAtomic) -> Self {
        match pg_atomic.into() {
            SqliteAtomic::Number(n) => Self::Number(n),
            SqliteAtomic::Text(t) => Self::Text(t),
            SqliteAtomic::Bool(b) => Self::Bool(b),
            SqliteAtomic::Operator(o) => Self::Operator(o),
            SqliteAtomic::Maybe(m) => Self::Maybe(m),
            SqliteAtomic::Sign(s) => Self::Sign(s),
            SqliteAtomic::Keyword(k) => Self::Keyword(k.0),
        }
    }
}
impl ToSqlSegment for GenericAtomic {
    fn gen_sql_segment(&self) -> SqlSegment {
        match self {
            GenericAtomic::Sign(s) => SqlSegment::Simple(s.to_string()),
            GenericAtomic::Maybe(m) => SqlSegment::Simple(m.gen_sql_segment().to_sql(false).to_string()),
            GenericAtomic::Operator(b) => SqlSegment::Simple(b.to_string()),
            GenericAtomic::Bool(b) => SqlSegment::Simple(b.to_string()),
            GenericAtomic::Text(t) => SqlSegment::Simple(t.to_string()),
            GenericAtomic::Number(n) => SqlSegment::Simple(n.to_string()),
            GenericAtomic::Keyword(k) => SqlSegment::Simple(k.to_string()),
        }
    }
}
