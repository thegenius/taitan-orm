mod name_mapper;
mod mark_mapper;
mod set_mapper;
mod condition_mapper;
mod connector;
mod field_wrapper;
mod single_field_mapper;
mod multi_field_mapper;

pub use crate::sql_generator::KeywordsEscaper;
pub use crate::field_mapper::field_wrapper::FieldWrapper;
pub use crate::field_mapper::mark_mapper::MarkMapper;
pub use crate::field_mapper::name_mapper::NameMapper;

pub use crate::field_mapper::single_field_mapper::SingleFieldMapper;
pub use crate::field_mapper::multi_field_mapper::MultiFieldMapper;
pub use crate::field_mapper::single_field_mapper::NamesMapper;
pub use crate::field_mapper::single_field_mapper::MarksMapper;
pub use connector::Connector;


pub enum CommaType {
    NoComma,
    LeadingComma,
    CheckedComma,
}
impl CommaType {
    pub fn parse(index: usize, first_required_index: usize) -> Self {
        if index == first_required_index {
            CommaType::NoComma
        } else if index < first_required_index {
            CommaType::CheckedComma
        } else {
            CommaType::LeadingComma
        }
    }
}
