use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DeriveInput, Fields, GenericParam, Generics, Lifetime};
use crate::util::extract_generic_lifetimes;

pub fn copy_to_template_struct(struct_ident: &Ident, data: &Data, generics: &Generics, sql: &str, struct_suffix: &str) -> TokenStream {
    let mut lifetimes: Vec<Lifetime> = extract_generic_lifetimes(generics);
    let struct_name = struct_ident;
    let template_struct_ident = format_ident!("{}{}", struct_name, struct_suffix);

    // 获取字段信息
    let fields = match data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => panic!("Only structs with named fields are supported"),
        },
        _ => panic!("This macro only works on structs"),
    };

    // 生成新的结构体名称


    // 生成字段定义
    let field_defs: Vec<_> = fields
        .iter()
        .map(|f| {
            let field_name = &f.ident;
            let field_type = &f.ty;
            quote! { #field_name: #field_type }
        })
        .collect();

    // 生成字段初始化
    let field_inits: Vec<_> = fields
        .iter()
        .map(|f| {
            let field_name = f.ident.as_ref().unwrap();
            quote! { #field_name: orig.#field_name.clone() }
        })
        .collect();

    if lifetimes.is_empty() {
        quote! {
            #[derive(Clone, rinja::Template)]
            #[template(source = #sql, ext="txt")]
            pub struct #template_struct_ident {
                #(#field_defs),*
            }

            impl From<&#struct_name> for #template_struct_ident {
                fn from(orig: &#struct_name) -> Self {
                    Self {
                        #(#field_inits),*
                    }
                }
            }
        }
    } else {
        quote! {
            #[derive(Clone, rinja::Template)]
            #[template(source = #sql, ext="txt")]
            pub struct #template_struct_ident <#(#lifetimes),*> {
                #(#field_defs),*
            }

            impl <#(#lifetimes),*> From<&#struct_name<#(#lifetimes),*>> for #template_struct_ident<#(#lifetimes),*> {
                fn from(orig: &#struct_name<#(#lifetimes),*>) -> #template_struct_ident<#(#lifetimes),*> {
                    Self {
                        #(#field_inits),*
                    }
                }
            }
        }
    }
}
