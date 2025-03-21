use crate::{FieldMapper, TableDef};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::fmt::Debug;


#[derive(Debug, Default)]
pub struct SelectedStructGenerator;

impl SelectedStructGenerator {
    pub fn generate(&self, table_def: &TableDef) -> TokenStream {
        let struct_name = &table_def.struct_name;
        let struct_ident = format_ident!("{}Selected", struct_name);

        let field_mapper = FieldMapper::new();
        let fields = &table_def.fields;
        let fields_stream = field_mapper.gen_struct_fields(fields, true);
        quote! {
            #[derive(Clone, Debug, taitan_orm::macros::Selected, serde::Serialize, serde::Deserialize)]
            pub struct #struct_ident {
                #fields_stream
            }
        }
    }
}
