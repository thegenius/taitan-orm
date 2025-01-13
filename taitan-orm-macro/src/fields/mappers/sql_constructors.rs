
use case::CaseExt;
use proc_macro2::{Ident, Span, TokenStream};
use quote::format_ident;
use quote::quote;
use crate::fields::FieldsContainer;
use crate::fields::mappers::sql_constructor::SqlConstructor;

pub trait SqlConstructors: FieldsContainer + SqlConstructor {
    /// pub enum UserIndexUser {
    ///     Id {
    ///         id: taitan_orm::traits::LocationExpr<i64>,
    ///     },
    ///     IdName {
    ///         id: taitan_orm::traits::LocationExpr<i64>,
    ///         name: taitan_orm::traits::LocationExpr<String>,
    ///     },
    /// }
    ///  fn get_where_clause(&self) -> String {
    ///         let mut sql = String::new();
    ///         match self {
    ///             UserIndexUser::Id { id } => {
    ///                 sql.push_str("`id`");
    ///                 sql.push_str(id.cmp.get_sql());
    ///                 sql.push('?');
    ///             },
    ///             UserIndexUser::IdName { id, name } => {
    ///                 sql.push_str("`id`");
    ///                 sql.push_str(id.cmp.get_sql());
    ///                 sql.push('?');
    ///                 sql.push_str("`name`");
    ///                 sql.push_str(id.cmp.get_sql());
    ///                 sql.push('?');
    ///             },
    ///         }
    ///         sql
    ///     }
    fn of_index_enum_where_fn(
        &self,
        struct_name: &str,
    ) -> TokenStream {
        let struct_ident = format_ident!("{}", struct_name);
        let fields = self.get_fields();
        let mut variants :Vec<TokenStream> = Vec::new();
        for i in 0..fields.len() {
            let enum_fields = fields[0..=i].to_vec();

            let enum_fields_clone = enum_fields.clone();
            let variant_fields = enum_fields_clone.iter().map(|field|field.ident.as_ref().unwrap()).collect::<Vec<_>>();

            let variant_name = enum_fields.clone()
                .iter()
                .map(|field| field.ident.as_ref().unwrap().to_string().to_camel())
                .collect::<Vec<String>>()
                .join("");
            let variant_ident = format_ident!("{}", variant_name);

            let last_index = enum_fields.len() - 1;
            let push_stmts = enum_fields
                .into_iter()
                .map(Self::get_expr_field)
                .collect::<Vec<TokenStream>>();


            let connected_push_stmts: Vec<TokenStream> = push_stmts.clone().into_iter()
                .enumerate().flat_map(|(i, item)| {
                    if i < last_index {
                        vec![item, quote! { sql.push_str(" AND "); }]
                    } else {
                        vec![item]
                    }
            }).collect::<Vec<TokenStream>>();



            let variant = quote! {
                #struct_ident::#variant_ident{#(#variant_fields,)*.. } =>{
                    #(#connected_push_stmts)*
                }
            };
            variants.push(variant);
        };
        quote!{
            fn get_where_clause(&self) -> String {
                let mut sql = String::new();
                match self {
                    #(#variants,)*
                }
                sql
            }
        }
    }
}