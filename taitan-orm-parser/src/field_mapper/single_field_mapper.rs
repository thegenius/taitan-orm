use crate::{FieldDef, KeywordsEscaper};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::borrow::Cow;

// pub enum FieldMapResult<'a> {
//     Literal(Cow<'a, str>),
//     Stream(TokenStream),
// }

pub trait SingleFieldMapper {
    fn get_value_name(&self) -> &'static str;
    fn map<'a>(&'a self, field: &'a FieldDef<'a>, escaper: &dyn KeywordsEscaper) -> Cow<'a, str>;
    fn map_indexed<'a>(
        &'a self,
        field: &'a FieldDef<'a>,
        escaper: &dyn KeywordsEscaper,
        index: usize,
    ) -> Cow<'a, str>;

    fn map_with_leading_comma<'a>(
        &'a self,
        field: &'a FieldDef<'a>,
        escaper: &dyn KeywordsEscaper,
    ) -> Cow<'a, str>;

    fn map_indexed_with_leading_comma<'a>(
        &'a self,
        field: &'a FieldDef<'a>,
        escaper: &dyn KeywordsEscaper,
        index: usize,
    ) -> Cow<'a, str>;

    fn map_dynamic(&self, field: &FieldDef, escaper: &dyn KeywordsEscaper) -> TokenStream;

    fn map_dynamic_with_leading_comma(&self, field: &FieldDef, escaper: &dyn KeywordsEscaper) -> TokenStream;

    fn map_dynamic_indexed(&self, field: &FieldDef, escaper: &dyn KeywordsEscaper) -> TokenStream;
    fn map_dynamic_indexed_with_leading_comma(&self, field: &FieldDef, escaper: &dyn KeywordsEscaper) -> TokenStream;
}

#[derive(Default)]
pub struct NamesMapper {}

impl SingleFieldMapper for NamesMapper {
    fn get_value_name(&self) -> &'static str {
        "names"
    }

    // fn map(&self, field: &FieldDef, escaper: &dyn KeywordsEscaper) -> TokenStream {
    //     let column_name = field.column_name(escaper);
    //     quote! { #column_name }
    // }

    fn map<'a>(&'a self, field: &'a FieldDef<'a>, escaper: &dyn KeywordsEscaper) -> Cow<'a, str> {
        field.column_name(escaper)
    }

    fn map_indexed<'a>(&'a self, field: &'a FieldDef<'a>, escaper: &dyn KeywordsEscaper, index: usize) -> Cow<'a, str> {
        field.column_name(escaper)
    }


    fn map_with_leading_comma<'a>(
        &'a self,
        field: &'a FieldDef<'a>,
        escaper: &dyn KeywordsEscaper,
    ) -> Cow<'a, str> {
        Cow::Owned(format!(",{}", field.column_name(escaper)))
    }

    fn map_indexed_with_leading_comma<'a>(&'a self, field: &'a FieldDef<'a>, escaper: &dyn KeywordsEscaper, index: usize) -> Cow<'a, str> {
        Cow::Owned(format!(",{}", field.column_name(escaper)))
    }

    fn map_dynamic(&self, field: &FieldDef, escaper: &dyn KeywordsEscaper) -> TokenStream {
        let name = field.column_name(escaper);
        quote! { #name }
    }

    fn map_dynamic_with_leading_comma(&self, field: &FieldDef, escaper: &dyn KeywordsEscaper) -> TokenStream {
        let name = field.column_name(escaper);
        quote! { #name }
    }


    fn map_dynamic_indexed(&self, field: &FieldDef, escaper: &dyn KeywordsEscaper) -> TokenStream {
        let name = field.column_name(escaper);
        quote! { #name }
    }

    fn map_dynamic_indexed_with_leading_comma(&self, field: &FieldDef, escaper: &dyn KeywordsEscaper) -> TokenStream {
        let name = format!(",{}", field.column_name(escaper));
        quote! { #name }
    }
}

#[derive(Default)]
pub struct MarksMapper {}

impl SingleFieldMapper for MarksMapper {
    fn get_value_name(&self) -> &'static str {
        "marks"
    }

    // fn map(&self, field: &FieldDef, escaper: &dyn KeywordsEscaper) -> TokenStream {
    //     quote! { '?' }
    // }

    fn map(&self, field: &FieldDef, escaper: &dyn KeywordsEscaper) -> Cow<'_, str> {
        Cow::Borrowed("?")
    }

    fn map_indexed<'a>(&'a self, field: &'a FieldDef<'a>, escaper: &dyn KeywordsEscaper, index: usize) -> Cow<'a, str> {
        Cow::Owned(format!("${}", index + 1))
    }

    fn map_with_leading_comma<'a>(
        &'a self,
        field: &'a FieldDef<'a>,
        escaper: &dyn KeywordsEscaper,
    ) -> Cow<'a, str> {
        Cow::Borrowed(",?")
    }

    fn map_indexed_with_leading_comma<'a>(&'a self, field: &'a FieldDef<'a>, escaper: &dyn KeywordsEscaper, index: usize) -> Cow<'a, str> {
        Cow::Owned(format!(",${}", index + 1))
    }

    fn map_dynamic(&self, field: &FieldDef, escaper: &dyn KeywordsEscaper) -> TokenStream {
        quote! { "?" }
    }

    fn map_dynamic_with_leading_comma(&self, field: &FieldDef, escaper: &dyn KeywordsEscaper) -> TokenStream {
        quote! { ",?" }
    }


    fn map_dynamic_indexed(&self, field: &FieldDef, escaper: &dyn KeywordsEscaper) -> TokenStream {
        quote! {format!("${}", index)}
    }

    fn map_dynamic_indexed_with_leading_comma(&self, field: &FieldDef, escaper: &dyn KeywordsEscaper) -> TokenStream {
        quote! {format!(",${}", index)}
    }
}

#[derive(Default)]
pub struct SetsMapper {}

impl SingleFieldMapper for SetsMapper {
    fn get_value_name(&self) -> &'static str {
        "sets"
    }

    fn map(&self, field: &FieldDef, escaper: &dyn KeywordsEscaper) -> Cow<'_, str> {
        Cow::Owned(format!("{}=?", field.column_name(escaper)))
    }

    fn map_indexed<'a>(&'a self, field: &'a FieldDef<'a>, escaper: &dyn KeywordsEscaper, index: usize) -> Cow<'a, str> {
        Cow::Owned(format!("{}=${}", field.column_name(escaper), index + 1))
    }

    fn map_with_leading_comma<'a>(
        &'a self,
        field: &'a FieldDef<'a>,
        escaper: &dyn KeywordsEscaper,
    ) -> Cow<'a, str> {
        Cow::Owned(format!(",{}=?", field.column_name(escaper)))
    }

    fn map_indexed_with_leading_comma<'a>(&'a self, field: &'a FieldDef<'a>, escaper: &dyn KeywordsEscaper, index: usize) -> Cow<'a, str> {
        Cow::Owned(format!(",{}=${}", field.column_name(escaper), index + 1))
    }

    fn map_dynamic(&self, field: &FieldDef, escaper: &dyn KeywordsEscaper) -> TokenStream {
        let name = field.column_name(escaper);
        let format_str = format!("{}=?", name);
        quote! { #format_str }
    }

    fn map_dynamic_with_leading_comma(&self, field: &FieldDef, escaper: &dyn KeywordsEscaper) -> TokenStream {
        let name = field.column_name(escaper);
        let format_str = format!(",{}=?", name);
        quote! { #format_str }
    }


    fn map_dynamic_indexed(&self, field: &FieldDef, escaper: &dyn KeywordsEscaper) -> TokenStream {
        let name = field.column_name(escaper);
        let format_str = format!("{}=${{}}", name);
        quote! {format!(#format_str, index)}
    }

    fn map_dynamic_indexed_with_leading_comma(&self, field: &FieldDef, escaper: &dyn KeywordsEscaper) -> TokenStream {
        let name = field.column_name(escaper);
        let format_str = format!(",{}=${{}}", name);
        quote! {format!(#format_str, index)}
    }
}


#[derive(Default)]
pub struct ConditionsMapper {}

impl SingleFieldMapper for ConditionsMapper {
    fn get_value_name(&self) -> &'static str {
        "conditions"
    }

    fn map(&self, field: &FieldDef, escaper: &dyn KeywordsEscaper) -> Cow<'_, str> {
        panic!("condition can not be mapped at compile time")
    }

    fn map_indexed<'a>(&'a self, field: &'a FieldDef<'a>, escaper: &dyn KeywordsEscaper, index: usize) -> Cow<'a, str> {
        Cow::Owned(format!("{}=${}", field.column_name(escaper), index + 1))
    }

    fn map_with_leading_comma<'a>(
        &'a self,
        field: &'a FieldDef<'a>,
        escaper: &dyn KeywordsEscaper,
    ) -> Cow<'a, str> {
        Cow::Owned(format!(",{}=?", field.column_name(escaper)))
    }

    fn map_indexed_with_leading_comma<'a>(&'a self, field: &'a FieldDef<'a>, escaper: &dyn KeywordsEscaper, index: usize) -> Cow<'a, str> {
        Cow::Owned(format!(",{}=${}", field.column_name(escaper), index + 1))
    }

    fn map_dynamic(&self, field: &FieldDef, escaper: &dyn KeywordsEscaper) -> TokenStream {
        let name = field.column_name(escaper);
        let struct_name = &field.struct_field.name;
        let struct_ident = format_ident!("{}", struct_name);
        let format_str = format!("{}{{}}?", name);
        quote! {format!(#format_str, self.#struct_ident.get_cmp_sql())}
    }

    fn map_dynamic_with_leading_comma(&self, field: &FieldDef, escaper: &dyn KeywordsEscaper) -> TokenStream {
        let name = field.column_name(escaper);
        let struct_name = &field.struct_field.name;
        let struct_ident = format_ident!("{}", struct_name);
        let format_str = format!(",{}{{}}?", name);
        quote! {format!(#format_str, self.#struct_ident.get_cmp_sql())}
    }


    fn map_dynamic_indexed(&self, field: &FieldDef, escaper: &dyn KeywordsEscaper) -> TokenStream {
        let name = field.column_name(escaper);
        let struct_name = &field.struct_field.name;
        let struct_ident = format_ident!("{}", struct_name);
        let format_str = format!("{}{{}}${{}}", name);
        quote! {format!(#format_str, self.#struct_ident.get_cmp_sql(), index)}
    }

    fn map_dynamic_indexed_with_leading_comma(&self, field: &FieldDef, escaper: &dyn KeywordsEscaper) -> TokenStream {
        let name = field.column_name(escaper);
        let struct_name = &field.struct_field.name;
        let struct_ident = format_ident!("{}", struct_name);
        let format_str = format!(",{}{{}}${{}}", name);
        quote! {format!(#format_str, self.#struct_ident.get_cmp_sql(), index)}
    }
}

