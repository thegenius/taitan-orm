use std::borrow::Cow;
use super::{SingleFieldMapper, KeywordsEscaper};
use crate::{FieldDef};
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

    fn map_indexed(&self, fields: &[FieldDef], escaper: &dyn KeywordsEscaper) -> TokenStream {
        let stream = fields
            .iter().enumerate()
            .map(|(index, field)| {
                let name = SingleFieldMapper::map_indexed(self, field, escaper, index);
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

    fn map_indexed_with_leading_comma(&self, fields: &[FieldDef], escaper: &dyn KeywordsEscaper) -> TokenStream {
        let stream = fields
            .iter().enumerate()
            .map(|(index, field)| {
                let name = SingleFieldMapper::map_indexed_with_leading_comma(self, field, escaper, index);
                name
            })
            .collect::<Vec<Cow<'_, str>>>().join("");
        quote! {
            #stream
        }
    }


}

impl<T: SingleFieldMapper> MultiFieldMapper for T {}
