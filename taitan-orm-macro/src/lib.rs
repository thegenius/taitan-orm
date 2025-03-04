#![allow(dead_code)]
#![forbid(unsafe_code)]
use crate::schema::impl_schema_macro;
use crate::selected::impl_selected_macro;
use crate::template::impl_template_macro;
use proc_macro::TokenStream;
use std::io::Write;
use syn::{parse_macro_input, DeriveInput};
use taitan_orm_parser::{ConditionDef, DatabaseType, EntityTraitImplGenerator, IndexEnum, IndexStructGenerator, LocationEnumGenerator, LocationTraitImplGenerator, MutationStructGenerator, MutationTraitImplGenerator, ParameterTraitImplGenerator, SelectedDefaultImplGenerator, SelectedTraitImplGenerator, TableDef};
// use crate::brave_new::extract_table_def;
use crate::location::impl_condition_macro;

mod attrs;
mod db_type;
mod expands;
mod fields;
mod location;
mod schema;
mod selected;
mod template;
mod types;
mod util;

fn get_supported_database_types() -> Vec<DatabaseType> {
    let mut supported_database_types: Vec<DatabaseType> = Vec::new();
    #[cfg(feature = "sqlite")]
    supported_database_types.push(DatabaseType::Sqlite);
    #[cfg(feature = "mysql")]
    supported_database_types.push(DatabaseType::MySql);
    #[cfg(feature = "postgres")]
    supported_database_types.push(DatabaseType::Postgres);

    if supported_database_types.is_empty() {
        panic!("The database type is empty, you should set features");
    }

    supported_database_types
}

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
    file.sync_all()?;
    Ok(())
}

#[proc_macro_derive(
    SchemaNew,
    attributes(
        debug,
        table,
        primary,
        unique,
        auto_increment,
        generated,
        field,
        serde_struct,
        index
    )
)]
pub fn expand_schema_new_macro(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);
    let table_def = TableDef::parse(&derive_input);

    // panic!("{:?}", table_def);

    let index_generator = IndexStructGenerator::default();
    let mut stream = TokenStream::new();
    generate_param_impl(&mut stream, &table_def);
    generate_entity_impl(&mut stream, &table_def);

    let supported_database_types = get_supported_database_types();
    let primary_stream: TokenStream = index_generator
        .generate(&table_def, &IndexEnum::Primary, &supported_database_types)
        .into();
    stream.extend(primary_stream.clone());

    for unique in &table_def.uniques {
        let index_type = IndexEnum::Unique {
            name: unique.name.to_string(),
        };
        let index_stream: TokenStream = index_generator.generate(&table_def, &index_type, &supported_database_types).into();
        stream.extend(index_stream);
    }
    for index in &table_def.indexes {
        let index_type = IndexEnum::Index {
            name: index.name.to_string(),
        };
        let index_stream: TokenStream = index_generator.generate(&table_def, &index_type, &supported_database_types).into();
        stream.extend(index_stream);
    }
    let mutation_generator = MutationStructGenerator::default();
    let mutation_struct_stream: TokenStream = mutation_generator.generate(&table_def).into();
    stream.extend(mutation_struct_stream);

    let location_generator = LocationEnumGenerator::default();
    let location_stream: TokenStream = location_generator.generate(&table_def).into();
    stream.extend(location_stream);

    // panic!("{}", stream);
    stream.into()
}

fn generate_param_impl(stream: &mut TokenStream, table_def: &TableDef) {
    let generator = ParameterTraitImplGenerator::default();
    let supported_database_types = get_supported_database_types();
    for database_type in supported_database_types {
        let s: TokenStream = generator.gen_add_to_args(&database_type, &table_def).into();
        stream.extend(s);
    }
}

#[proc_macro_derive(Parameter, attributes(field))]
pub fn expand_param_macro(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);
    let table_def = TableDef::parse(&derive_input);
    let mut stream = TokenStream::new();
    generate_param_impl(&mut stream, &table_def);
    // panic!("{}", stream);
    stream.into()
}

fn generate_entity_impl(stream: &mut TokenStream, table_def: &TableDef) {
    let generator = EntityTraitImplGenerator::default();
    let supported_database_types = get_supported_database_types();
    for database_type in supported_database_types {
        let s: TokenStream = generator.generate(&database_type, &table_def).into();
        stream.extend(s);
    }
}

#[proc_macro_derive(EntityNew, attributes(field))]
pub fn expand_entity_new_macro(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);
    let table_def = TableDef::parse(&derive_input);
    let mut stream = TokenStream::new();
    generate_entity_impl(&mut stream, &table_def);
    // panic!("{}", stream);
    stream.into()
}

#[proc_macro_derive(LocationNew, attributes(field))]
pub fn expand_location_new_macro(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);
    let condition_def = ConditionDef::parse(&derive_input);
    let generator = LocationTraitImplGenerator::default();
    let supported_database_types = get_supported_database_types();
    let mut stream = TokenStream::new();
    for database_type in supported_database_types {
        let s: TokenStream = generator.generate(&database_type, &condition_def).into();
        stream.extend(s);
    }
    // panic!("{}", stream);
    stream.into()
}

#[proc_macro_derive(MutationNew, attributes(field))]
pub fn expand_mutation_new_macro(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);
    let table_def = TableDef::parse(&derive_input);
    let generator = MutationTraitImplGenerator::default();
    let supported_database_types = get_supported_database_types();
    let mut stream = TokenStream::new();
    for database_type in supported_database_types {
        let s: TokenStream = generator.generate(&database_type, &table_def).into();
        stream.extend(s);
    }
    // panic!("{}", stream);
    stream.into()
}

#[proc_macro_derive(SelectedNew, attributes(field))]
pub fn expand_selected_new_macro(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);
    let table_def = TableDef::parse(&derive_input);
    let generator = SelectedTraitImplGenerator::default();
    let supported_database_types = get_supported_database_types();
    let mut stream = TokenStream::new();
    for database_type in supported_database_types {
        let s: TokenStream = generator.generate(&database_type, &table_def).into();
        stream.extend(s);
    }
    let default_generator = SelectedDefaultImplGenerator::default();
    let default_stream: TokenStream = default_generator.generate(&table_def).into();
    stream.extend(default_stream);

    // panic!("{}", stream);
    stream.into()
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
