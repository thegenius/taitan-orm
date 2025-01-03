use case::CaseExt;
use proc_macro2::TokenStream;
use syn::Field;
use quote::{format_ident, quote};
use crate::types::{DefaultTypeChecker, DefaultTypeExtractor, TypeChecker, TypeExtractor};

pub trait StructFieldConstructor {

    // field_name: T
    fn get_not_option_field(field: Field) -> TokenStream {
        let field_ident = field.ident;
        if DefaultTypeChecker::type_is_option(&field.ty) {
            let inner_type = DefaultTypeExtractor::get_option_inner_type(&field.ty).unwrap();
            quote! {
                pub #field_ident: #inner_type
            }
        } else {
            let field_ty = field.ty;
            quote!{
                pub #field_ident: #field_ty
            }
        }
    }

    // field_name: Option<T>
    fn get_option_field(field: Field) -> TokenStream {
        let field_ident = field.ident;
        let field_ty = field.ty;
        if DefaultTypeChecker::type_is_option(&field_ty) {
            let inner_type = DefaultTypeExtractor::get_option_inner_type(&field_ty).unwrap();
            // quote! {
            //     #field_ident: #field_ty
            // }
            quote! {
                pub #field_ident: taitan_orm::Optional<#inner_type>
            }
        } else {
            quote! {
                pub #field_ident: taitan_orm::Optional<#field_ty>
            }
        }
    }

    // field_name: Option<LocationExpr<T>>
    fn get_location_field(field: Field) -> TokenStream {
        let field_ident = field.ident;
        let field_ty = field.ty;
        if DefaultTypeChecker::type_is_option(&field_ty) {
            let inner_type = DefaultTypeExtractor::get_option_inner_type(&field_ty).unwrap();
            quote! {
                pub #field_ident: taitan_orm::Optional<taitan_orm::traits::LocationExpr<#inner_type>>
            }
        } else {
            quote! {
                pub #field_ident: taitan_orm::Optional<taitan_orm::traits::LocationExpr<#field_ty>>
            }
        }
    }

    fn get_location_expr_enum_field(field: Field) -> TokenStream {
        let field_ident = field.ident;
        let field_ident_name = &field_ident.unwrap().to_string();
        let enum_field_ident = format_ident!("{}", field_ident_name.to_camel());

        let field_ty = field.ty;
        if DefaultTypeChecker::type_is_option(&field_ty) {
            let inner_type = DefaultTypeExtractor::get_option_inner_type(&field_ty).unwrap();
            quote! {
                #enum_field_ident(taitan_orm::traits::LocationExpr<#inner_type>)
            }
        } else {
            quote! {
                #enum_field_ident(taitan_orm::traits::LocationExpr<#field_ty>)
            }
        }
    }

    // field_name: bool
    fn get_bool_field(field: Field) -> TokenStream {
        let field_ident = field.ident;
        quote!{
            pub #field_ident: bool
        }
    }

    fn get_bool_true_field(field: Field) -> TokenStream {
        let field_ident = field.ident;
        quote!{
            #field_ident: true
        }
    }

    fn get_optional_selected_field(field: Field) -> TokenStream {
        let field_ident = field.ident;
        quote!{
            #field_ident: taitan_orm::Optional::Selected
        }
    }
}