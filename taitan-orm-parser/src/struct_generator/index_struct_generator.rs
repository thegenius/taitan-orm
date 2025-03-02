use std::borrow::Cow;
use std::fmt::Debug;
use case::CaseExt;
use crate::{DatabaseType, FieldMapper, SqlGenerator, TableDef};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use crate::condition_def::ConditionDef;

#[derive(Debug, Default)]
pub struct IndexStructGenerator;


#[derive(Debug, Clone)]
pub enum IndexEnum {
    Primary,
    Unique{name: String},
    Index{name: String},
}


impl IndexStructGenerator {

    pub fn generate(
        &self,
        table_def: &TableDef,
        index_enum: &IndexEnum
    ) -> TokenStream {

        let struct_name = &table_def.struct_name;


        let field_mapper = FieldMapper::new();
        // let table_name =  field_mapper.escape(&table_def.table_name, db_type);
        let sql_generator = SqlGenerator::default();
        let struct_ident = match index_enum {
            IndexEnum::Primary => {format_ident!("{}Primary", struct_name)},
            IndexEnum::Unique{name} => {format_ident!("{}Unique{}", struct_name, name.to_camel())},
            IndexEnum::Index{name} => {format_ident!("{}Index{}", struct_name, name.to_camel())},
        };

        let fields = match index_enum {
            IndexEnum::Primary => {
                table_def.get_primary_fields()
            }
            IndexEnum::Unique {name} => {
                table_def.get_unique_fields(name)
            }
            IndexEnum::Index {name} => {
                table_def.get_index_fields(name)
            }
        };

        let fields_stream = field_mapper.gen_struct_fields(fields);


        quote! {
            pub struct #struct_ident {
                #fields_stream
            }
        }
    }
}
