use std::borrow::Cow;
use std::fmt::Debug;
use case::CaseExt;
use crate::{DatabaseType, FieldMapper, SqlGenerator, TableDef};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

#[derive(Debug, Default)]
pub struct IndexStructGenerator;


#[derive(Debug, Clone)]
pub enum IndexEnum {
    Primary,
    Unique{name: String},
    Index{name: String},
}



impl IndexStructGenerator {

    fn impl_unique<'a>(db_types: &[DatabaseType], struct_name: &Cow<'a, str>,  struct_ident: &Ident) -> TokenStream {
        let mut stream = TokenStream::new();
        for db_type in db_types {
            let mutation_ident = format_ident!("{}Mutation", struct_name);
            let db_ident = db_type.gen_ident();
            stream.extend(quote! {
                impl taitan_orm::traits::Unique<sqlx::#db_ident> for #struct_ident {
                    type Mutation = #mutation_ident;
                }
            });
        }
        stream
    }

    pub fn generate(
        &self,
        table_def: &TableDef,
        index_enum: &IndexEnum,
        db_types: &[DatabaseType]
    ) -> TokenStream {

        let struct_name = &table_def.struct_name;


        let field_mapper = FieldMapper::new();
        let table_name =  &table_def.table_name;
        let table_name_ident= format_ident!("{}", table_name);
        let sql_generator = SqlGenerator::default();
        let struct_ident = match index_enum {
            IndexEnum::Primary => {format_ident!("{}Primary", struct_name)},
            IndexEnum::Unique{name} => {format_ident!("{}Unique{}", struct_name, name.to_camel())},
            IndexEnum::Index{name} => {format_ident!("{}Index{}", struct_name, name.to_camel())},
        };

        let fields_stream = match index_enum {
            IndexEnum::Primary => {
                let fields = table_def.get_primary_fields();
                let fields_stream = field_mapper.gen_struct_fields(fields, false);
                let impl_unique = IndexStructGenerator::impl_unique(db_types, struct_name, &struct_ident);
                quote! {
                    #[derive(Debug, Clone, taitan_orm::macros::Parameter, taitan_orm::macros::Location, serde::Serialize, serde::Deserialize)]
                    #[table(#table_name_ident)]
                    pub struct #struct_ident {
                        #fields_stream
                    }
                    #impl_unique
                }
            }
            IndexEnum::Unique {name} => {
                let fields = table_def.get_unique_fields(name);
                let fields_stream = field_mapper.gen_struct_fields(fields, false);
                let impl_unique = IndexStructGenerator::impl_unique(db_types, struct_name, &struct_ident);
                quote! {
                    #[derive(Debug, Clone, taitan_orm::macros::Parameter, taitan_orm::macros::Location, serde::Serialize, serde::Deserialize)]
                    #[table(#table_name_ident)]
                    pub struct #struct_ident {
                        #fields_stream
                    }
                    #impl_unique
                }
            }
            IndexEnum::Index {name} => {
                let fields = table_def.get_index_fields(name);
                let fields_stream = field_mapper.gen_enum_variants( fields);
                quote! {
                    #[derive(Debug, Clone, taitan_orm::macros::Parameter, taitan_orm::macros::Location, serde::Serialize, serde::Deserialize)]
                    #[table(#table_name_ident)]
                    pub enum #struct_ident {
                        #fields_stream
                    }
                }
            }
        };

        fields_stream
    }
}
