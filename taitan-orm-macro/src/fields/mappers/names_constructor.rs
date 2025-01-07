use crate::attrs::{AttrParser, DefaultAttrParser};
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{Attribute};
use crate::fields::{FieldsContainer};
use crate::fields::mappers::names_add_constructor::NamesAddConstructor;


// TODO: 现在返回这个Vec是为了构建sql，应该可以不使用vec，让sql的构建完全需要分配vec内存
// Mutation:      fn get_mutation_fields_name(&self) -> Vec<FieldName>;
// Selection:     fn get_selected_fields(&self) -> Vec<String>;
// Entity:        fn get_insert_fields(&self) -> Vec<FieldName>;
//                fn get_upsert_set_fields(&self) -> Vec<FieldName>;
// Unique:        fn get_unique_field_names(&self) -> &'static [&'static str];
// Location:      fn get_location_fields_name(&self) -> Vec<FieldName>;


pub trait NamesConstructor : FieldsContainer + NamesAddConstructor {
    // fn of_table_name(ident: &Ident, attrs: &Vec<Attribute>) -> String {
    //     let name = ident.to_string();
    //     DefaultAttrParser::extract_val_from_attrs(attrs, "TableName").unwrap_or(name)
    // }

    fn of_names_array(&self) -> TokenStream {
        let tokens = self.map_field_vec(&<Self as NamesAddConstructor>::of_str);
        quote!(
            &[ #(#tokens,)* ]
        )
    }

    fn of_maybe_option_names_vec(&self) -> TokenStream {
        let tokens =self.map_field_vec(&<Self as NamesAddConstructor>::of_maybe_option);
        quote!(
            let mut fields = Vec::new();
            #(#tokens;)*
            return fields;
        )
    }

    fn of_option_names_vec(&self) -> TokenStream {
        let tokens =self.map_field_vec(&<Self as NamesAddConstructor>::of_option);
        quote!(
            let mut fields = Vec::new();
            #(#tokens;)*
            return fields;
        )
    }

    fn of_enum_names_vec(&self) -> TokenStream {
        let tokens =self.map_field_vec(&<Self as NamesAddConstructor>::of_enum);
        quote!(
            let mut fields = Vec::new();
            match self {
                #(#tokens)*
            }
            return fields;
        )
    }

    fn of_not_option_names_vec(&self) -> TokenStream {
        let tokens =self.map_field_vec(&<Self as NamesAddConstructor>::of_not_option);
        quote!(
            let mut fields = Vec::new();
            #(#tokens;)*
            return fields;
        )
    }

    fn of_bool_names_vec(&self) -> TokenStream {
        let tokens =self.map_field_vec(&<Self as NamesAddConstructor>::of_bool);
        quote!(
            let mut fields = Vec::new();
            #(#tokens;)*
            return fields;
        )
    }
    fn of_self_optional_names_vec(&self) -> TokenStream {
        let tokens =self.map_field_vec(&<Self as NamesAddConstructor>::of_self_optional);
        quote!(
            let mut fields = Vec::new();
            #(#tokens;)*
            return fields;
        )
    }

    fn of_bool_bits_vec(&self) -> TokenStream {
        let tokens =self.map_field_vec(&<Self as NamesAddConstructor>::of_bool_bit);
        quote!(
            let mut bits = bit_vec::BitVec::new();
            #(#tokens;)*
            return bits;
        )
    }

    fn of_where_clause(&self) -> TokenStream {
        let tokens =self.map_field_vec(&<Self as NamesAddConstructor>::of_where_seg);
        quote! {
            let mut sql = String::default();
            #(#tokens)*
            sql
        }
    }

}
