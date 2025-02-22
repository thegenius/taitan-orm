use syn::{GenericArgument, Path, PathArguments, Type};

pub struct TypeParser;

impl TypeParser {
    pub fn has_prefix(ty: &Type, name: &str) -> bool {
        if let Type::Path(type_path) = ty {
            let idents_of_path =
                type_path
                    .path
                    .segments
                    .iter()
                    .fold(String::new(), |mut acc, v| {
                        acc.push_str(&v.ident.to_string());
                        acc.push_str("::");
                        acc
                    });
            idents_of_path.starts_with(name)
        } else {
            false
        }
    }

    pub fn has_one_of_names(ty: &Type, names: &[&str]) -> bool {
        names.iter().any(|name| Self::has_prefix(ty, name))
    }

    pub fn is_option(ty: &Type) -> bool {
        Self::has_one_of_names(
            ty,
            &[
                "Option::",
                "std::option::Option::",
                "core::option::Option::",
                "Optional::",
                "taitan_orm::prelude::Optional::",
                "taitan_orm::result::Optional::",
            ],
        )
    }

    pub fn is_location_expr(ty: &Type) -> bool {
        Self::has_one_of_names(
            ty,
            &[
                "LocationExpr",
                "taitan_orm::traits::LocationExpr",
            ],
        )
    }

    pub fn get_path(ty: &Type) -> Option<&Path> {
        match *ty {
            Type::Path(ref type_path) if type_path.qself.is_none() => Some(&type_path.path),
            _ => None,
        }
    }

    pub fn get_path_string(ty: &Type) -> String {
        match ty {
            Type::Path(type_path) => {
                let idents_of_path =
                    type_path
                        .path
                        .segments
                        .iter()
                        .fold(String::new(), |mut acc, v| {
                            acc.push_str(&v.ident.to_string());
                            acc.push_str("::");
                            acc
                        });
                idents_of_path
            }
            _ => "".to_string(),
        }
    }

    pub fn get_option_inner_type(ty: &Type) -> Option<&Type> {
        if !Self::is_option(ty) {
            return None;
        }

        let Type::Path(ty) = ty else { return None };
        if ty.qself.is_some() {
            return None;
        }

        let last_segment = ty.path.segments.last().unwrap();
        let PathArguments::AngleBracketed(generics) = &last_segment.arguments else {
            return None;
        };
        if generics.args.len() != 1 {
            return None;
        }
        let GenericArgument::Type(inner_type) = &generics.args[0] else {
            return None;
        };

        Some(inner_type)
    }

    pub fn get_inner_type(ty: &Type) -> Option<&Type> {
        if !Self::is_option(ty) {
            return Some(ty);
        }
        Self::get_option_inner_type(ty)
    }

}





// fn type_is_location_mode(ty: &Type) -> bool {
//     TypeParser::has_one_of_names(ty, &["taitan_orm::prelude::LocationMode", "LocationMode"])
// }




// fn has_lifetime(ty: &Type) -> bool {
//     match ty {
//         Type::Path(type_path) => has_lifetime_in_type_path(type_path),
//         _ => false,
//     }
// }
//
// fn has_lifetime_in_type_path(type_path: &TypePath) -> bool {
//     // 遍历路径段
//     for segment in &type_path.path.segments {
//         // 检查泛型参数
//         if let PathArguments::AngleBracketed(ref args) = segment.arguments {
//             for arg in &args.args {
//                 match arg {
//                     GenericArgument::Lifetime(_) => return true,
//                     GenericArgument::Type(ty) => {
//                         if has_lifetime(ty) {
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

// pub fn field_optional_inner_type(field: &Field) -> Option<&Type> {
//     let ty = &field.ty;
//     get_option_inner_type(ty)
// }
//
// pub fn field_inner_type(field: &Field) -> Type {
//     let ty = &field.ty;
//     if type_is_option(ty) {
//         return field_optional_inner_type(field).unwrap().clone();
//     }
//     ty.clone()
// }
// pub fn field_type_path(field: &Field) -> String {
//     let ty = &field.ty;
//     match ty {
//         Type::Path(type_path) => {
//             let idents_of_path =
//                 type_path
//                     .path
//                     .segments
//                     .iter()
//                     .fold(String::new(), |mut acc, v| {
//                         acc.push_str(&v.ident.to_string());
//                         acc.push_str("::");
//                         acc
//                     });
//             idents_of_path
//         }
//         _ => "".to_string(),
//     }
// }
//
// pub fn field_is_option(field: &Field) -> bool {
//     type_is_option(&field.ty)
// }
//
// pub fn field_is_type_of(field: &Field, type_name: &str) -> bool {
//     let ty: &Type = &field.ty;
//     type_has_prefix(ty, type_name)
// }
