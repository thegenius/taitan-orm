
mod field_def;
mod table_def;
mod info_parser;
mod struct_generator;
mod impl_generator;
pub mod utils;
mod sql_generator;
mod field_mapper;
mod db_type;
mod sql_type;
mod args_generator;
mod condition_def;
mod template_parser;

// pub use info_parser::schema_parser::extract_table_def;
pub use info_parser::attr_parser;

pub use field_def::StructFieldDef;
pub use field_def::TableColumnDef;

pub use table_def::TableDef;
pub use field_def::FieldDef;
pub use field_def::FieldName;
pub use condition_def::ConditionDef;

pub use info_parser::LifetimeParser;
pub use info_parser::InputParser;
pub use info_parser::FieldParser;
pub use info_parser::FieldAttrParser;
pub use info_parser::NamedVariant;
pub use info_parser::NamedVariantDef;
pub use info_parser::ConditionParser;

// pub use field_mapper::KeywordsEscaper;
// pub use field_mapper::MySqlKeywordEscaper;
// pub use field_mapper::PostgresKeywordEscaper;
// pub use field_mapper::SqliteKeywordEscaper;
//
// pub use field_mapper::SingleFieldMapper;
// pub use field_mapper::MultiFieldMapper;
// pub use field_mapper::Connector;
// pub use field_mapper::NamesMapper;
// pub use field_mapper::MarksMapper;
// pub use field_mapper::SetsMapper;
// pub use field_mapper::ConditionsMapper;
pub use db_type::DatabaseType;
pub use struct_generator::IndexEnum;
pub use field_mapper::FieldMapper;
pub use field_mapper::KeywordsEscaper;
pub use sql_generator::SqlGenerator;
pub use sql_type::SqlType;
pub use impl_generator::ParameterTraitImplGenerator;
pub use impl_generator::EntityTraitImplGenerator;
pub use impl_generator::LocationTraitImplGenerator;
pub use impl_generator::MutationTraitImplGenerator;
pub use impl_generator::SelectedTraitImplGenerator;
pub use impl_generator::SelectedDefaultImplGenerator;
pub use struct_generator::IndexStructGenerator;
pub use struct_generator::MutationStructGenerator;
pub use struct_generator::LocationEnumGenerator;