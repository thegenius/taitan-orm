use proc_macro2::TokenStream;
use quote::quote_spanned;
// use syn::spanned::Spanned;
use syn::{Field, LitStr};
use crate::types::{DefaultTypeChecker, TypeChecker};

pub trait RowGetConstructor {

    // // TODO: 有可能使用try_get(0)性能更好，所以这个方案可能需要废弃掉
    // #[deprecated]
    // fn of_selected_row(field: Field) -> TokenStream {
    //     let field_name = field.ident.unwrap();
    //     let span = field_name.span();
    //     let field_name_lit = LitStr::new(&field_name.to_string(), span);
    //     quote_spanned! { span =>
    //         if selection.#field_name {
    //             selected.#field_name = sqlx::Row::try_get(&row, #field_name_lit).ok().into();
    //         }
    //     }
    // }

    // #[deprecated]
    // fn of_selected_row_i(field: Field) -> TokenStream {
    //     let field_name = field.ident.unwrap();
    //     let span = field_name.span();
    //     // let field_name_lit = LitStr::new(&field_name.to_string(), span);
    //     quote_spanned! { span =>
    //         if selection.#field_name {
    //             selected.#field_name = sqlx::Row::try_get(&row, i).ok().into();
    //             i += 1;
    //         }
    //     }
    // }

    // used by from_selected_row
    fn of_selected_self_row_i(field: Field) -> TokenStream {
        let field_name = field.ident.unwrap();
        let span = field_name.span();
        if DefaultTypeChecker::type_is_option(&field.ty) {
            quote_spanned! { span =>
                if selection.#field_name.is_selected() {
                    selected.#field_name = taitan_orm::result::Optional::Some(sqlx::Row::try_get(&row, i)?);
                    i += 1;
                }
            }
        } else {
            quote_spanned! { span =>
                selected.#field_name = sqlx::Row::try_get(&row, i)?;
                i += 1;
            }
        }
    }

    // TODO: 也许这个函数也是不需要的
    // used by from_row_full
    fn of_row_i(field: Field) -> TokenStream {
        let field_name = field.ident.unwrap();
        let span = field_name.span();
        if DefaultTypeChecker::type_is_option(&field.ty) {
            quote_spanned! { span =>
                selected.#field_name = taitan_orm::result::Optional::Some(sqlx::Row::try_get(&row, i)?);
                i += 1;
            }
        } else {
            quote_spanned! { span =>
                selected.#field_name = sqlx::Row::try_get(&row, i)?;
                i += 1;
            }
        }

    }

    // fn of_selected_bits_row_i(field: Field) -> TokenStream {
    //     let field_name = field.ident.unwrap();
    //     let span = field_name.span();
    //     // let field_name_lit = LitStr::new(&field_name.to_string(), span);
    //     quote_spanned! { span =>
    //         if bits.get(0).unwrap_or(false)  {
    //             selected.#field_name = sqlx::Row::try_get(&row, i).ok().into();
    //             i += 1;
    //         };
    //     }
    // }

    // fn of_selected_bits_index_row_i(field: &Field, index: usize) -> TokenStream {
    //     let field_name = field.clone().ident.unwrap();
    //     let span = field_name.span();
    //     // let field_name_lit = LitStr::new(&field_name.to_string(), span);
    //     quote_spanned! { span =>
    //         if bits.get(#index).unwrap_or(false)  {
    //             selected.#field_name = sqlx::Row::try_get(&row, i).ok().into();
    //             i += 1;
    //         };
    //     }
    // }

    // fn of_row(field: Field) -> TokenStream {
    //     let field_name = field.ident.unwrap();
    //     let span = field_name.span();
    //     let field_name_lit = LitStr::new(&field_name.to_string(), span);
    //     quote_spanned! { span =>
    //         selected.#field_name = sqlx::Row::try_get(&row, #field_name_lit).ok().into();
    //     }
    // }


}
