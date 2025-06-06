mod structs;
mod to_sql;
mod syntax_parser;
mod parser2;
mod error;

pub use structs::simple_expr::SimpleExpr;
pub use structs::sql_part::SqlPart;
pub use structs::atomic::Atomic;
pub use structs::template_part::TemplatePart;
pub use structs::variable::Variable;
pub use structs::variable::VariableChain;
// pub use structs::expr::Expr;
pub use structs::sign::Sign;
pub use structs::sql_template::SqlTemplate;
pub use to_sql::ToSqlSegment;
pub use structs::number::Number;

pub use structs::atomic::AtomicStream;
pub use structs::atomics::GenericAtomicStream;
pub use structs::atomics::GenericAtomic;
pub use structs::atomics::MySqlAtomic;
pub use structs::atomics::PostgresAtomic;
pub use structs::atomics::SqliteAtomic;



pub use structs::placeholder::Placeholder;
pub use structs::placeholder::RawPlaceholder;

pub use structs::operators::Operator;
pub use structs::operators::CompareOp;
pub use structs::operators::LogicOp;
pub use structs::operators::MatchOp;
pub use structs::operators::ArithmeticOp;
pub use structs::operators::ListInOp;


pub use structs::exprs::ArithmeticExpr;
pub use structs::exprs::LogicExpr;
pub use structs::exprs::TextExpr;

pub use structs::values::NumberValue;
pub use structs::values::BoolValue;
pub use structs::values::TextValue;
pub use structs::values::GenericValue;
pub use structs::values::MaybeValue;
pub use structs::exprs::GenericExpr;
