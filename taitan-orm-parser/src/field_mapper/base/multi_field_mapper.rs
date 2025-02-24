use super::{KeywordsEscaper, LeadingCommaType, SingleFieldMapper};
use crate::FieldDef;
use proc_macro2::TokenStream;
use quote::{quote};
use std::borrow::Cow;

pub trait MultiFieldMapper: SingleFieldMapper {


    fn map_group<'a, T>(
        &self,
        fields: T,
        escaper: &dyn KeywordsEscaper,
        indexed: bool,
        leading_comma: bool,
    ) -> TokenStream
    where
        T: IntoIterator<Item = &'a FieldDef<'a>>,
    {
        if indexed {
            if leading_comma {
                MultiFieldMapper::map_indexed_with_leading_comma(self, fields, escaper)
            } else {
                MultiFieldMapper::map_indexed(self, fields, escaper)
            }
        } else {
            if leading_comma {
                MultiFieldMapper::map_with_leading_comma(self, fields, escaper)
            } else {
                MultiFieldMapper::map(self, fields, escaper)
            }
        }
    }

    fn map<'a, T>(&self, fields: T, escaper: &dyn KeywordsEscaper) -> TokenStream
    where
        T: IntoIterator<Item = &'a FieldDef<'a>>,
    {
        let stream = fields
            .into_iter()
            .enumerate()
            .map(|(index, field)| SingleFieldMapper::map(self, field.as_ref(), escaper, LeadingCommaType::NoLeading))
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
                let name = SingleFieldMapper::map_indexed(self, field, escaper, LeadingCommaType::NoLeading, index);
                name
            })
            .collect::<Vec<Cow<'_, str>>>()
            .join(",");
        quote! {
            #stream
        }
    }

    fn map_with_leading_comma<'a, T>(&self, fields: T, escaper: &dyn KeywordsEscaper) -> TokenStream
    where
        T: IntoIterator<Item = &'a FieldDef<'a>>,
    {
        let stream = fields
            .into_iter()
            .map(|field| {
                let name = SingleFieldMapper::map(self, field, escaper, LeadingCommaType::Leading);
                name
            })
            .collect::<Vec<Cow<'_, str>>>()
            .join("");
        quote! {
            #stream
        }
    }

    fn map_indexed_with_leading_comma<'a, T>(
        &self,
        fields: T,
        escaper: &dyn KeywordsEscaper,
    ) -> TokenStream
    where
        T: IntoIterator<Item = &'a FieldDef<'a>>,
    {
        let stream = fields
            .into_iter()
            .enumerate()
            .map(|(index, field)| {
                let name =
                    SingleFieldMapper::map_indexed(self, field, escaper, LeadingCommaType::Leading, index);
                name
            })
            .collect::<Vec<Cow<'_, str>>>()
            .join("");
        quote! {
            #stream
        }
    }
}

impl<T: SingleFieldMapper> MultiFieldMapper for T {}
