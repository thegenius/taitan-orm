use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use crate::{FieldMapper, TableDef};

#[derive(Debug, Default)]
pub struct SelectedDefaultImplGenerator;


impl SelectedDefaultImplGenerator {
    pub fn generate(
        &self,
        table_def: &TableDef,
    ) -> TokenStream {

        let struct_name = &table_def.struct_name;
        let struct_ident = format_ident!("{}", &struct_name);
        let field_mapper = FieldMapper::new();
        let selected_default_stream = field_mapper.gen_selected_default(&table_def.fields);
        quote! {

            impl Default for #struct_ident {
                fn default() -> Self {
                    Self {
                        #selected_default_stream
                    }
                }
            }
        }
    }
}