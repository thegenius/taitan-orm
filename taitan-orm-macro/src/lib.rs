#![allow(dead_code)]
#![forbid(unsafe_code)]
use crate::schema::impl_schema_macro;
use crate::selected::impl_selected_macro;
use crate::template::impl_template_macro;
use proc_macro::TokenStream;
use crate::location::impl_condition_macro;

mod attrs;
mod expands;
mod fields;
mod schema;
mod selected;
mod template;
mod types;
mod util;
mod location;

#[proc_macro_derive(
    Schema,
    attributes(
        table_name,
        primary_key,
        unique_key,
        auto_increment,
        generated,
        field_name,
        serde_struct,
        index
    )
)]
pub fn expand_schema_macro(input: TokenStream) -> TokenStream {
    impl_schema_macro(input)
}

#[proc_macro_derive(Selected, attributes(table_name))]
pub fn expand_selected(input: TokenStream) -> TokenStream {
    impl_selected_macro(input)
}

#[proc_macro_derive(Condition, attributes(table_name))]
pub fn expand_location(input: TokenStream) -> TokenStream {
    impl_condition_macro(input)
}

#[proc_macro_derive(TemplateRecord, attributes(sql, count_sql, limit_field))]
pub fn expand_template_record(input: TokenStream) -> TokenStream {
    impl_template_macro(input)
}
