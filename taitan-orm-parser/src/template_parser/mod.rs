
mod parsed_template_sql;
mod parsers;
mod structs;
mod to_sql;

pub use to_sql::ToSql;

pub use structs::TemplatePlaceholder;
pub use structs::TemplateVariableChain;
pub use structs::TemplateVariable;
pub use structs::TemplateString;
pub use structs::TemplateExpr;
pub use structs::TemplateConnective;
pub use structs::TemplateSqlValue;

pub use structs::TemplateExprFirstPart;
pub use structs::TemplateExprSecondPart;
pub use structs::OptionalVariable;
pub use structs::UnitOptionalContext;
pub use structs::PairOptionalContext;





