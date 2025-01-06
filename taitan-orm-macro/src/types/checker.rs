use syn::{Field, Type};

pub trait TypeChecker {
    fn type_has_prefix(ty: &Type, name: &str) -> bool;

    fn type_has_one_of_names(ty: &Type, names: &[&str]) -> bool;

    fn type_is_option(ty: &Type) -> bool;

    fn get_field_type_name(field: &Field) -> String;

    fn field_is_option(field: &Field) -> bool;

    fn field_is_type_of(field: &Field, type_name: &str) -> bool;
}

pub struct DefaultTypeChecker {}
impl TypeChecker for DefaultTypeChecker {
    fn type_has_prefix(ty: &Type, name: &str) -> bool {
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
                idents_of_path.starts_with(name)
            }
            _ => false,
        }
    }

    fn type_has_one_of_names(ty: &Type, names: &[&str]) -> bool {
        names
            .iter()
            .any(|name| <DefaultTypeChecker as TypeChecker>::type_has_prefix(ty, name))
    }

    fn type_is_option(ty: &Type) -> bool {
        <DefaultTypeChecker as TypeChecker>::type_has_one_of_names(
            ty,
            &[
                "Option::",
                "std::option::Option::",
                "core::option::Option::",
                "Optional::",
                "taitan_orm::prelude::Optional::",
                "taitan_orm::result::Optional::"
            ],
        )
    }

    fn get_field_type_name(field: &Field) -> String {
        let ty = &field.ty;
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

    fn field_is_option(field: &Field) -> bool {
        <DefaultTypeChecker as TypeChecker>::type_is_option(&field.ty)
    }

    fn field_is_type_of(field: &Field, type_name: &str) -> bool {
        let ty: &Type = &field.ty;
        <DefaultTypeChecker as TypeChecker>::type_has_prefix(ty, type_name)
    }
}
