
use syn::{GenericArgument, PathArguments, Type};

pub struct OptionParser;
impl OptionParser {
    pub fn get_option_inner_type(ty: &Type) -> Option<&Type> {
        let Type::Path(type_path) = ty else {
            return None;
        };
        if type_path.qself.is_some() {
            return None;
        }

        let last_segment = type_path.path.segments.last();
        if last_segment.is_none() {
            return None;
        }

        let Some(last_segment) = last_segment else {
            return None;
        };

        if last_segment.ident != "Option" {
            return None;
        }

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
    pub fn get_nested_option_inner_type_recursively(ty: &Type, max_nest_level: usize) -> (&Type, usize) {
        if max_nest_level == 0 {
            return (ty, 0);
        }

        let inner_type_opt = Self::get_option_inner_type(ty);
        let Some(inner_type) = inner_type_opt else {
            return (ty, 0);
        };

        if max_nest_level == 1 {
            (inner_type, 1)
        } else {
            // max_nest_level > 1
            let (inner_most_type, nesting_level) =
                Self::get_nested_option_inner_type_recursively(inner_type, max_nest_level - 1);
            (inner_most_type, nesting_level + 1)
        }
    }
    pub fn get_nested_option_inner_type(ty: &Type) -> (&Type, usize) {
        Self::get_nested_option_inner_type_recursively(ty, usize::MAX)
    }

    pub fn is_option_type(ty: &Type) -> bool {
        Self::get_option_inner_type(ty).is_some()
    }
}
