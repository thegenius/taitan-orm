use std::fmt::Debug;
use proc_macro2::TokenStream;

use crate::fields::FieldsContainer;


pub trait SelectedParser: FieldsContainer {
        fn gen_from_row(&self) -> TokenStream;

        fn gen_from_row_full(&self) -> TokenStream;

}