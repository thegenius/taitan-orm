#![allow(dead_code)]
#![forbid(unsafe_code)]
use proc_macro::TokenStream;
use std::borrow::Cow;
use std::error::Error;
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput};
use taitan_orm_parser::{ConditionDef, DatabaseType, EntityTraitImplGenerator, IndexEnum, IndexStructGenerator, InputParser, LocationEnumGenerator, LocationTraitImplGenerator, MutationStructGenerator, MutationTraitImplGenerator, OrderByStructGenerator, ParameterTraitImplGenerator, SelectedDefaultImplGenerator, SelectedStructGenerator, SelectedTraitImplGenerator, TableDef, TemplateArgTraitImplGenerator, TemplateTraitImplGenerator};

// mod attrs;
// mod db_type;
// mod expands;
// mod fields;
// mod location;
// mod schema;
// mod selected;
// mod template;
// mod types;
// mod util;

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

// fn write_content_to_file(content: &str, file_path: &str) -> std::io::Result<()> {
//     // match env::current_dir() {
//     //     Ok(current_dir) => {
//     //         println!("当前工作目录: {:?}", current_dir);
//     //         panic!("{:?}", current_dir);
//     //     },
//     //     Err(e) => eprintln!("无法获取当前工作目录: {}", e),
//     // }
//     let mut file = std::fs::File::create(file_path)?;
//     file.write_all(content.as_bytes())?;
//     file.sync_all()?;
//     Ok(())
// }

#[proc_macro_derive(
    Schema,
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
        let index_stream: TokenStream = index_generator
            .generate(&table_def, &index_type, &supported_database_types)
            .into();
        stream.extend(index_stream);
    }
    for index in &table_def.indexes {
        let index_type = IndexEnum::Index {
            name: index.name.to_string(),
        };
        let index_stream: TokenStream = index_generator
            .generate(&table_def, &index_type, &supported_database_types)
            .into();
        stream.extend(index_stream);
    }
    let mutation_generator = MutationStructGenerator::default();
    let mutation_struct_stream: TokenStream = mutation_generator.generate(&table_def).into();
    stream.extend(mutation_struct_stream);

    let location_generator = LocationEnumGenerator::default();
    let location_stream: TokenStream = location_generator.generate(&table_def).into();
    stream.extend(location_stream);

    let selected_generator = SelectedStructGenerator::default();
    let selected_struct_stream: TokenStream = selected_generator.generate(&table_def).into();
    stream.extend(selected_struct_stream);

    let order_by_generator = OrderByStructGenerator::default();
    let order_by_struct_stream: TokenStream = order_by_generator.generate(&table_def).into();
    stream.extend(order_by_struct_stream);

    // panic!("{}", stream);
    stream.into()
}



fn generate_order_by(stream: &mut TokenStream, table_def: &TableDef) {

}


fn generate_param_impl(stream: &mut TokenStream, table_def: &TableDef) {
    let generator = ParameterTraitImplGenerator::default();
    let supported_database_types = get_supported_database_types();
    for database_type in supported_database_types {
        let s: TokenStream = generator.gen_add_to_args(&database_type, &table_def).into();
        stream.extend(s);
    }
}

fn generate_enum_param_impl(stream: &mut TokenStream, cond_def: &ConditionDef) {
    let generator = ParameterTraitImplGenerator::default();
    let supported_database_types = get_supported_database_types();
    for database_type in supported_database_types {
        let s: TokenStream = generator.gen_enum_add_to_args(&database_type, &cond_def).into();
        stream.extend(s);
    }
}

#[proc_macro_derive(Parameter, attributes(field))]
pub fn expand_param_macro(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);
    let is_enum = InputParser::is_enum(&derive_input.data);
    if !is_enum {
        let table_def = TableDef::parse(&derive_input);
        let mut stream = TokenStream::new();
        generate_param_impl(&mut stream, &table_def);
        // panic!("{}", stream);
        stream.into()
    } else {
        let cond_def = ConditionDef::parse(&derive_input);
        let mut stream = TokenStream::new();
        generate_enum_param_impl(&mut stream, &cond_def);
        // panic!("{}", stream);
        stream.into()
    }

}

fn generate_template_arg_impl(stream: &mut TokenStream, table_def: &TableDef) {
    let generator = TemplateArgTraitImplGenerator::default();
    let supported_database_types = get_supported_database_types();
    for database_type in supported_database_types {
        let s: TokenStream = generator.gen_add_to_args(&database_type, &table_def).into();
        stream.extend(s);
    }
}

#[proc_macro_derive(TemplateArg, attributes(field))]
pub fn expand_template_arg_macro(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);
    let table_def = TableDef::parse(&derive_input);
    let mut stream = TokenStream::new();
    generate_template_arg_impl(&mut stream, &table_def);
    // panic!("{}", stream);
    stream.into()
}

fn generate_template_new_impl(stream: &mut TokenStream, table_def: &TableDef) {
    let generator = TemplateTraitImplGenerator::default();
    let supported_database_types = get_supported_database_types();
    for database_type in supported_database_types {
        let s: TokenStream = generator.generate(&database_type, &table_def).into();
        stream.extend(s);
    }

    // add impl TemplateSqlTrait
    let struct_name = &table_def.struct_name;
    let struct_ident = format_ident!("{}", &struct_name);
    let s: TokenStream = quote! {
        impl taitan_orm::traits::TemplateSqlTrait for #struct_ident {}
    }.into();
    stream.extend(s);
}

#[proc_macro_derive(Template, attributes(sql))]
pub fn expand_template_new_macro(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);
    let table_def = TableDef::parse(&derive_input);
    let mut stream = TokenStream::new();
    generate_template_new_impl(&mut stream, &table_def);
    generate_template_arg_impl(&mut stream, &table_def);
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

#[proc_macro_derive(Entity, attributes(field))]
pub fn expand_entity_new_macro(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);
    let table_def = TableDef::parse(&derive_input);
    let mut stream = TokenStream::new();
    generate_entity_impl(&mut stream, &table_def);
    // panic!("{}", stream);
    stream.into()
}

#[proc_macro_derive(Location, attributes(table, field))]
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

#[proc_macro_derive(Mutation, attributes(field))]
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

#[proc_macro_derive(Selected, attributes(field))]
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

// #[proc_macro_derive(
//     Schema,
//     attributes(
//         table_name,
//         primary_key,
//         unique_key,
//         auto_increment,
//         generated,
//         field_name,
//         serde_struct,
//         index
//     )
// )]
// pub fn expand_schema_macro(input: TokenStream) -> TokenStream {
//     impl_schema_macro(input)
// }
//
// #[proc_macro_derive(Selected, attributes(table_name))]
// pub fn expand_selected(input: TokenStream) -> TokenStream {
//     impl_selected_macro(input)
// }
//
// #[proc_macro_derive(Condition, attributes(table_name))]
// pub fn expand_location(input: TokenStream) -> TokenStream {
//     impl_condition_macro(input)
// }
//
// #[proc_macro_derive(TemplateRecord, attributes(sql, count_sql, limit_field))]
// pub fn expand_template_record(input: TokenStream) -> TokenStream {
//     impl_template_macro(input)
// }
