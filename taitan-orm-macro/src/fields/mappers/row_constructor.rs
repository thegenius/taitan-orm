use proc_macro2::TokenStream;
use quote::quote;
use crate::fields::{FieldsContainer};
use crate::fields::mappers::row_get_constructor::RowGetConstructor;

pub trait RowConstructor : FieldsContainer + RowGetConstructor {
    fn gen_selected_named_row(&self) -> TokenStream {
        let tokens =self.map_field_vec(&<Self as RowGetConstructor>::of_selected_row);
        quote!(
            let mut selected = Self::default();
            #(#tokens;)*
            Ok(selected)
        )
    }

    fn gen_selected_row(&self) -> TokenStream {
        let tokens =self.map_field_vec(&<Self as RowGetConstructor>::of_selected_row_i);
        quote!(
            let mut selected = Self::default();
            let mut i = 0;
            #(#tokens;)*
            Ok(selected)
        )
    }

    fn gen_selected_bits_row(&self) -> TokenStream {
        let tokens = self.map_field_vec_with_index(&<Self as RowGetConstructor>::of_selected_bits_index_row_i);
        quote!(
            let mut selected = Self::default();
            let mut i = 0;
            #(#tokens;)*
            Ok(selected)
        )
    }

    fn gen_selected_self_row(&self) -> TokenStream {
        let tokens = self.map_field_vec(&<Self as RowGetConstructor>::of_selected_self_row_i);
        quote!(
            let mut selected = Self::default();
            let mut i = 0;
            #(#tokens;)*
            Ok(selected)
        )
    }

    fn gen_full_named_row(&self) -> TokenStream {
        let tokens =self.map_field_vec(&<Self as RowGetConstructor>::of_row);
        quote!(
            let mut selected = Self::default();
            #(#tokens;)*
            Ok(selected)
        )
    }

    fn gen_full_row(&self) -> TokenStream {
        let tokens =self.map_field_vec(&<Self as RowGetConstructor>::of_row_i);
        quote!(
            let mut selected = Self::default();
            let mut i = 0;
            #(#tokens;)*
            Ok(selected)
        )
    }
}