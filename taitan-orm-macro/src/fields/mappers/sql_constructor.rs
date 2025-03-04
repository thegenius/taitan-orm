use proc_macro2::TokenStream;
use quote::quote;
use syn::{Field, LitStr};
use syn::spanned::Spanned;

pub trait SqlConstructor {

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
    fn get_expr_field(field: Field) -> TokenStream {
        let span = field.span();
        let field_ident = field.ident.unwrap();
        let field_name = field_ident.to_string();
        let quote_field_name = format!("`{}`", field_name);
        let quote_field_name_string = LitStr::new(&quote_field_name, span);
        quote! {
            sql.push_str(#quote_field_name_string);
            sql.push_str(#field_ident.cmp.get_sql());
            sql.push('?');
        }
    }
}