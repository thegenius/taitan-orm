#![allow(dead_code)]
#![forbid(unsafe_code)]
use crate::schema::impl_schema_macro;
use crate::selected::impl_selected_macro;
use crate::template::impl_template_macro;
use proc_macro::{TokenStream};
use std::env;
use std::io::Write;
use std::process::id;
use syn::{parse_macro_input, Attribute, DeriveInput};
use crate::brave_new::extract_table_def;
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
mod brave_new;

fn write_content_to_file(content: &str, file_path: &str) -> std::io::Result<()> {
    // match env::current_dir() {
    //     Ok(current_dir) => {
    //         println!("当前工作目录: {:?}", current_dir);
    //         panic!("{:?}", current_dir);
    //     },
    //     Err(e) => eprintln!("无法获取当前工作目录: {}", e),
    // }
    let mut file = std::fs::File::create(file_path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

#[proc_macro_derive(
    SchemaNew,
    attributes(
        debug,
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
pub fn expand_schema_new_macro(input: TokenStream) -> TokenStream {
    let DeriveInput {
        attrs, ident, data, ..
    } = parse_macro_input!(input);

    let debugs= brave_new::attr_parser::extract_named_attrs_val(&attrs, "debug");
    if debugs.len() > 1 {
        panic!("cannot use more than one debug attribute");
    }
    let table_def = extract_table_def(&ident.to_string(), &attrs, &data);

    if debugs.len() == 1 {
        let debug_file = debugs.first().unwrap();
        // panic!("{}",debug_file.to_string());
        let table_def_json = serde_json::to_string(&table_def);
        if let Err(err) = write_content_to_file(table_def_json.as_ref().unwrap(), &debug_file) {
            panic!("cannot write to file: {}", err);
        }
    }

    TokenStream::new()
}

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
