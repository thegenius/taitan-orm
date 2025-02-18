use std::borrow::Cow;
use crate::field_mapper::single_field_mapper::SingleFieldMapper;
use crate::{FieldDef, KeywordsEscaper};
use proc_macro2::TokenStream;
use quote::quote;

pub trait MultiFieldMapper: SingleFieldMapper {
    fn map(&self, fields: &[FieldDef], escaper: &dyn KeywordsEscaper) -> TokenStream {
        let stream = fields
            .iter()
            .map(|field| {
                let name = SingleFieldMapper::map(self, field, escaper);
                name
            })
            .collect::<Vec<Cow<'_, str>>>().join(",");
        quote! {
            #stream
        }
    }

    fn map_with_leading_comma(&self, fields: &[FieldDef], escaper: &dyn KeywordsEscaper) -> TokenStream {
        let stream = fields
            .iter()
            .map(|field| {
                let name = SingleFieldMapper::map_with_leading_comma(self, field, escaper);
                name
            })
            .collect::<Vec<Cow<'_, str>>>().join("");
        quote! {
            #stream
        }
    }

    fn map_indexed_static(
        &self,
        fields: &[FieldDef],
        escaper: &dyn KeywordsEscaper,
    ) -> TokenStream {
        let stream = fields
            .iter()
            .enumerate()
            .map(|(index, field)| {
                SingleFieldMapper::map_indexed_static(self, field, index, escaper)
            })
            .collect::<Vec<TokenStream>>();
        quote! {
            #(#stream)*
        }
    }

    fn map_indexed_dynamic(
        &self,
        fields: &[FieldDef],
        escaper: &dyn KeywordsEscaper,
    ) -> TokenStream {
        let stream = fields
            .iter()
            .enumerate()
            .map(|(index, field)| {
                SingleFieldMapper::map_indexed_dynamic(self, field, escaper)
            })
            .collect::<Vec<TokenStream>>();
        quote! {
            #(#stream)*
        }
    }
}

impl<T: SingleFieldMapper> MultiFieldMapper for T {}
