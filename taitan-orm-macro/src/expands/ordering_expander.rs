use crate::attrs::{AttrParser, DefaultAttrParser};
use crate::fields::{FieldsContainer, FieldsFilter, FieldsParser, NamesConstructor};
use case::CaseExt;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::{Attribute, Field, FieldsNamed, LitStr};

fn generate_struct_and_impl(struct_name: &str, all_fields: &Vec<String>, fields: &Vec<Vec<String>>) -> TokenStream {
    let struct_ident = Ident::new(&struct_name, Span::call_site());
    let all_fields = transform_string_list(all_fields);
    let unique_fields_stream = transform_string_vec_list(fields);

    let output = quote! {

        #[derive(Debug, Default)]
        pub struct #struct_ident<'a> {
            fields: Vec<std::borrow::Cow<'a, str>>,
        }

        impl<'a> taitan_orm::traits::OrderBy for #struct_ident<'a> {
            fn unique_fields(&self) -> &[&[&str]] {
                #unique_fields_stream
            }

             fn all_fields(&self) -> &[&str] {
                #all_fields
            }

            fn get_fields(&self) -> &[std::borrow::Cow<'a, str>] {
                &self.fields
            }
        }

        impl<'a> #struct_ident<'a> {
            pub fn build<I, S>(fields: I) -> Result<Self, Box<dyn std::error::Error + 'static>>
            where
                I: IntoIterator<Item = S> + Clone,
                S: AsRef<str> + Into<std::borrow::Cow<'a, str>>, // 确保每个元素可以转换为 Cow<'a, str>
            {
                let order_by = Self::default();
                taitan_orm::traits::validate_order_by(
                    fields.clone(),
                    taitan_orm::traits::OrderBy::all_fields(&order_by),
                    taitan_orm::traits::OrderBy::unique_fields(&order_by)
                )?;

                Ok(Self {
                    fields: fields.into_iter().map(Into::into).collect(),
                })
            }
        }
    };

    output
}

fn transform_string_list(fields: &Vec<String>) -> TokenStream {
    let unique_field_list: Vec<TokenStream> = fields.iter().map(|field| {
        let field_lit = LitStr::new(field, Span::call_site());
        quote! { #field_lit }
    }).collect();
    quote! {
        &[ #(#unique_field_list,)* ]
    }
}

// to &[&["id"]]
fn transform_string_vec_list(fields_vec: &Vec<Vec<String>>) -> TokenStream {
    let stream_list = fields_vec.iter().map(transform_string_list).collect::<Vec<_>>();
    quote! {
        &[ #(#stream_list,)* ]
    }
}

pub fn generate_ordering_struct_and_impl(
    ident: &Ident,
    attrs: &Vec<Attribute>,
    fields: &FieldsNamed,
) -> TokenStream {
    let table_name = DefaultAttrParser::extract_table_name(ident, attrs);
    let parser = FieldsParser::from_named(fields);
    let all_fields = parser.get_fields();
    let all_fields_names = all_fields.iter().map(|field| field.ident.as_ref().unwrap().to_string()).collect::<Vec<_>>();

    let primary_fields = parser.filter_annotated_fields("primary_key");
    let primary_names = primary_fields.iter().map(|f| f.ident.as_ref().unwrap().to_string()).collect::<Vec<_>>();

    let mut order_fields_vec = DefaultAttrParser::extract_unique_key(attrs);
    order_fields_vec.push(primary_names);
    // panic!("{:?}", unique_keys);
    // let mut order_fields_vec: Vec<Vec<String>> = Vec::new();
    // order_fields_vec.push(vec!["id".to_owned(), "test".to_owned()]);
    // order_fields_vec.push(vec!["id".to_owned(), "age".to_owned()]);

    let primary_order_struct_name = format!("{}Ordering", table_name.to_camel());
    let ordering_stream = generate_struct_and_impl(&primary_order_struct_name, &all_fields_names, &order_fields_vec);
    ordering_stream
}
