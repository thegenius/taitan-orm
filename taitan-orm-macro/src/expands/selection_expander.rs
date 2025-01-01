use crate::attrs::{AttrParser, DefaultAttrParser};
use crate::fields::{FieldsParser, NamesConstructor, StructConstructor};
use case::CaseExt;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::{Attribute, FieldsNamed};

pub fn generate_selection_struct_and_impl(
    ident: &Ident,
    attrs: &Vec<Attribute>,
    fields: &FieldsNamed,
) -> TokenStream {
    let parser = FieldsParser::from_named(fields);

    let bool_names_vec = parser.of_bool_names_vec();
    let bool_names_bits = parser.of_bool_bits_vec();
    let full_fields_stream = parser.of_bool_true();

    let table_name = DefaultAttrParser::extract_table_name(ident, attrs);
    let struct_name = format!("{}Selection", table_name.to_camel());
    let struct_ident = Ident::new(&struct_name, Span::call_site());
    let struct_stream = FieldsParser::from_named(fields).of_bool(&struct_name);

    // fn get_selected_bits(&self) -> bit_vec::BitVec {
    //     #bool_names_bits
    // }

    let output = quote! {

        #struct_stream

        impl taitan_orm::traits::Selection for #struct_ident {

            fn get_table_name(&self) -> &'static str {
                #table_name
            }

            fn get_selected_fields(&self) -> Vec<String> {
                #bool_names_vec
            }

            fn full_fields() -> Self
                where Self: Sized,
            {
                #full_fields_stream
            }
        }
    };

    output
}
