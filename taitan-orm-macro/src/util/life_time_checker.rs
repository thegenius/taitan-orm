
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields, Field, Type, GenericParam, Generics, Lifetime};

use syn::visit::Visit;
use crate::util::create_path_from_str;

// 自定义访问器用于查找生命周期
struct LifetimeFinder {
    has_lifetime: bool,
}

impl<'ast> Visit<'ast> for LifetimeFinder {
    fn visit_lifetime(&mut self, _: &'ast syn::Lifetime) {
        self.has_lifetime = true;
    }
}

pub fn build_impl_trait_token(struct_ident: &Ident, generics: &Generics, trait_name: &str) -> TokenStream {
    let mut lifetimes: Vec<Lifetime> = Vec::new();
    for param in generics.params.iter() {
        if let GenericParam::Lifetime(lifetime_def) = param {
            lifetimes.push(lifetime_def.lifetime.clone());
        }
    }

    let trait_name = create_path_from_str(trait_name);
    if !lifetimes.is_empty() {
        quote! {
            impl <#(#lifetimes),*> #trait_name for #struct_ident<#(#lifetimes),*>
        }
    } else {
        quote! {
            impl #trait_name for #struct_ident
        }
    }
}

pub fn build_struct_ident(ident: &Ident, lifetimes: &Vec<Lifetime>) -> TokenStream {
    if lifetimes.is_empty() {
        quote! {
            #ident
        }
    } else {
        quote! {
            #ident<#(#lifetimes),*>
        }
    }
}

pub fn extract_generic_lifetimes(generics: &Generics) -> Vec<Lifetime> {
    let mut lifetimes: Vec<Lifetime> = Vec::new();
    for param in generics.params.iter() {
        if let GenericParam::Lifetime(lifetime_def) = param {
            lifetimes.push(lifetime_def.lifetime.clone());
        }
    }
    lifetimes
}


pub fn check_type_lifetime(ty: &Type) -> bool {
    let mut finder = LifetimeFinder { has_lifetime: false };
    finder.visit_type(ty);
    finder.has_lifetime
}
pub fn check_field_lifetime(field: &Field) -> bool {
    let mut finder = LifetimeFinder { has_lifetime: false };
    finder.visit_field(field);
    finder.has_lifetime
}

