use super::{DefaultTypeChecker, TypeChecker};
use quote::quote;
use syn::Path;

pub trait TypeExtractor {
    fn get_option_inner_type(ty: &syn::Type) -> Option<&syn::Type>;

    fn extract_type_path(ty: &syn::Type) -> Option<&Path>;

    fn create_option_type(ty: &syn::Type) -> proc_macro2::TokenStream;
}

pub struct DefaultTypeExtractor {}

impl TypeExtractor for DefaultTypeExtractor {
    fn get_option_inner_type(ty: &syn::Type) -> Option<&syn::Type> {
        if !<DefaultTypeChecker as TypeChecker>::type_is_option(ty) {
            return None;
        }

        let syn::Type::Path(ty) = ty else { return None };
        if ty.qself.is_some() {
            return None;
        }

        let last_segment = ty.path.segments.last().unwrap();
        let syn::PathArguments::AngleBracketed(generics) = &last_segment.arguments else {
            return None;
        };
        if generics.args.len() != 1 {
            return None;
        }
        let syn::GenericArgument::Type(inner_type) = &generics.args[0] else {
            return None;
        };

        Some(inner_type)
    }

    fn extract_type_path(ty: &syn::Type) -> Option<&Path> {
        match *ty {
            syn::Type::Path(ref type_path) if type_path.qself.is_none() => Some(&type_path.path),
            _ => None,
        }
    }

    fn create_option_type(ty: &syn::Type) -> proc_macro2::TokenStream {
        let output = quote!(
            std::option::Option<#ty>
        );
        output
    }
}
