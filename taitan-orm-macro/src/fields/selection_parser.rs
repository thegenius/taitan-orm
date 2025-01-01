use crate::fields::mappers::StructFieldConstructor;
use crate::fields::{FieldsContainer, NamesAddConstructor, NamesConstructor, StructConstructor};
use proc_macro2::TokenStream;

pub trait SelectionParser:
    FieldsContainer
    + NamesAddConstructor
    + NamesConstructor
    + StructFieldConstructor
    + StructConstructor
{
    fn get_selected_fields(&self) -> TokenStream {
        self.of_bool_names_vec()
    }

    fn full_fields(&self) -> TokenStream {
        self.of_bool_true()
    }
}
