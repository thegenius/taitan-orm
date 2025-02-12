use syn::{GenericParam, Generics, Lifetime, Type};
use syn::visit::Visit;



pub struct LifetimeParser;

#[derive(Default)]
struct LifetimeFinder {
    lifetime: Option<Lifetime>,
}

impl<'ast> Visit<'ast> for LifetimeFinder {
    fn visit_lifetime(&mut self, lifetime: &'ast syn::Lifetime) {
        self.lifetime = Some(lifetime.clone());
    }
}
impl LifetimeParser {
    pub fn has_lifetime(ty: &Type) -> bool {
        Self::get_lifetime(ty).is_some()
    }

    pub fn get_lifetime(ty: &Type) -> Option<Lifetime> {
        let mut finder = LifetimeFinder::default();
        finder.visit_type(ty);
        finder.lifetime
    }

    pub fn get_generic_lifetimes(generics: &Generics) -> Vec<Lifetime> {
        let mut lifetimes: Vec<Lifetime> = Vec::new();
        for param in generics.params.iter() {
            if let GenericParam::Lifetime(lifetime_def) = param {
                lifetimes.push(lifetime_def.lifetime.clone());
            }
        }
        lifetimes
    }

    // fn has_lifetime_in_type_path(type_path: &TypePath) -> bool {
    //     for segment in &type_path.path.segments {
    //         if let PathArguments::AngleBracketed(ref args) = segment.arguments {
    //             for arg in &args.args {
    //                 match arg {
    //                     GenericArgument::Lifetime(_) => return true,
    //                     GenericArgument::Type(ty) => {
    //                         if Self::has_lifetime(ty) {
    //                             return true;
    //                         }
    //                     }
    //                     _ => {}
    //                 }
    //             }
    //         }
    //     }
    //     false
    // }
    //
    // fn extract_lifetime_in_type_path(type_path: &TypePath) -> Vec<Lifetime> {
    //     let mut lifetimes: Vec<Lifetime> = Vec::new();
    //     for segment in &type_path.path.segments {
    //         if let PathArguments::AngleBracketed(ref args) = segment.arguments {
    //             for arg in &args.args {
    //                 match arg {
    //                     GenericArgument::Lifetime(life_time) => lifetimes.push(life_time.clone()),
    //                     GenericArgument::Type(ty) => {
    //                         let life_times = Self::extract_lifetime(ty);
    //                         lifetimes.extend(life_times);
    //                     }
    //                     _ => {}
    //                 }
    //             }
    //         }
    //     }
    //     lifetimes
    // }
}
