use super::{KeywordsEscaper, SingleFieldMapper};
use crate::FieldDef;
use proc_macro2::TokenStream;
use quote::quote;
use std::borrow::Cow;

pub trait MultiFieldMapper: SingleFieldMapper {
    fn map<'a, T>(&self, fields: T, escaper: &dyn KeywordsEscaper) -> TokenStream
    where
        T: IntoIterator<Item = &'a FieldDef<'a>>,
    {
        let stream = fields
            .into_iter()
            .map(|field| {
                let name = SingleFieldMapper::map(self, field.as_ref(), escaper);
                name
            })
            .collect::<Vec<Cow<'_, str>>>()
            .join(",");
        quote! {
            #stream
        }
    }

    fn map_indexed<'a, T>(&self, fields: T, escaper: &dyn KeywordsEscaper) -> TokenStream
    where
        T: IntoIterator<Item = &'a FieldDef<'a>>,
    {
        let stream = fields
            .into_iter()
            .enumerate()
            .map(|(index, field)| {
                let name = SingleFieldMapper::map_indexed(self, field, escaper, index);
                name
            })
            .collect::<Vec<Cow<'_, str>>>()
            .join(",");
        quote! {
            #stream
        }
    }

    fn map_with_leading_comma<'a, T>(
        &self,
        fields: T,
        escaper: &dyn KeywordsEscaper,
    ) -> TokenStream
    where
        T: IntoIterator<Item = &'a FieldDef<'a>>,{
        let stream = fields
            .into_iter()
            .map(|field| {
                let name = SingleFieldMapper::map_with_leading_comma(self, field, escaper);
                name
            })
            .collect::<Vec<Cow<'_, str>>>()
            .join("");
        quote! {
            #stream
        }
    }
    //
    // fn map_indexed_with_leading_comma<'a, T>(
    //     &self,
    //     fields: T,
    //     escaper: &dyn KeywordsEscaper,
    // ) -> TokenStream
    // where
    //     T: IntoIterator<Item = &'a FieldDef<'a>>,{
    //     let stream = fields
    //         .into_iter()
    //         .enumerate()
    //         .map(|(index, field)| {
    //             let name =
    //                 SingleFieldMapper::map_indexed_with_leading_comma(self, field, escaper, index);
    //             name
    //         })
    //         .collect::<Vec<Cow<'_, str>>>()
    //         .join("");
    //     quote! {
    //         #stream
    //     }
    // }
}

impl<T: SingleFieldMapper> MultiFieldMapper for T {}
