use proc_macro::TokenStream;


mod field_def;
mod table_def;
mod extractor;
mod struct_generator;
mod impl_generator;

pub use extractor::table_def_extractor::extract_table_def;
pub use extractor::attr_parser;