use proc_macro2::Ident;
use syn::{parenthesized, Attribute, Expr, Field, Lit, LitStr, Meta, MetaNameValue, Path, Token};
use syn::meta::ParseNestedMeta;
use syn::parse::ParseStream;
use darling::ast::NestedMeta;
use quote::format_ident;

pub trait AttrParser {
    fn extract_field_db_ident(field: &Field) -> Ident;
    fn extract_val_from_attr(attr: &Attribute, name: &str) -> Option<String>;

    fn check_is_attr(attr: &Attribute, name: &str) -> bool;

    fn extract_val_from_attrs(attrs: &Vec<Attribute>, name: &str) -> Option<String>;

    fn extract_val_vec_from_attrs(attrs: &Vec<Attribute>, name: &str) -> Vec<String>;

    fn check_has_attr(attrs: &Vec<Attribute>, name: &str) -> bool;

    fn extract_table_name(ident: &Ident, attrs: &Vec<Attribute>) -> String;

    fn extract_template_sql(attrs: &Vec<Attribute>) -> Option<String>;
    fn extract_template_count_sql(attrs: &Vec<Attribute>) -> Option<String>;

    fn extract_unique_key(attrs: &Vec<Attribute>) -> Vec<Vec<String>>;
}

pub struct DefaultAttrParser {}

impl AttrParser for DefaultAttrParser {
    fn extract_val_from_attr(attr: &Attribute, name: &str) -> Option<String> {
        let path: &Path = attr.path();
        if !path.is_ident(name) {
            return None;
        }

        match &attr.meta {
            Meta::NameValue(name_value) => {
                match &name_value.value {
                     Expr::Lit(s) => {
                        match &s.lit {
                            Lit::Str(s) => Some(s.value()),
                            _ => None,
                        }
                    },
                    _ => None,
                }
            }
            _ => None,
        }



        // attr.parse_nested_meta( |meta| {
        //     let result = match meta {
        //         Meta::NameValue(name_value) => {
        //             match name_value.value {
        //                 syn::Expr::Lit(s) => {
        //                     match s.lit {
        //                         Lit::Str(s) => Some(s.value()),
        //                         _ => None,
        //                     }
        //                 },
        //                 _ => None,
        //             }
        //         },
        //         _ => None
        //     };
        //     Ok(())
        // });

        // let name: syn::Result<String>= attr.parse_args_with(|stream: ParseStream| {
        //     let lit_str = stream.parse::<LitStr>()?;
        //     Ok(lit_str.value())
        //     // stream.parse::<LitStr>().ok().and_then(|lit_str| {
        //     //     Some(lit_str.value())
        //     // })
        // });
        // name.ok()





        // let path_ident = path.get_ident().unwrap();
        // let attr_path_name = path_ident.to_string();
        // if attr_path_name != name {
        //     return None;
        // }

        //
        // meta_info_result.ok()

        // match attr.parse_meta() {
        //     Ok(Meta::NameValue(meta_name_value)) => {
        //         Some(meta_name_value.value())
        //     },
        //     _ => None,
        // }

        // let meta_info = meta_info_result.unwrap();
        // let value = match meta_info {
        //     syn::Meta::NameValue(syn::MetaNameValue {
        //         lit: syn::Lit::Str(s),
        //         ..
        //     }) => s.value(),
        //     _ => panic!("malformed attribute syntax"),
        // };
        // return Some(value);
    }

    fn extract_field_db_ident(field: &Field) -> Ident {
        let alias = Self::extract_val_from_attrs(&field.attrs, "field_name");
        match alias {
            None => {
                return field.ident.as_ref().unwrap().clone();
            }
            Some(alias) => {
                format_ident!("{}", alias)
            }
        }
    }

    fn check_is_attr(attr: &Attribute, name: &str) -> bool {
        let path: &Path = attr.path();
        let path_ident = path.get_ident().unwrap();
        let attr_path_name = path_ident.to_string();
        return attr_path_name == name;
    }

    fn extract_val_from_attrs(attrs: &Vec<Attribute>, name: &str) -> Option<String> {
        for attr in attrs {
            let val_opt = <DefaultAttrParser as AttrParser>::extract_val_from_attr(attr, name);
            if val_opt.is_some() {
                return val_opt;
            }
        }
        return None;
    }

    fn extract_val_vec_from_attrs(attrs: &Vec<Attribute>, name: &str) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        for attr in attrs {
            let val_opt = <DefaultAttrParser as AttrParser>::extract_val_from_attr(attr, name);
            if val_opt.is_some() {
                result.push(val_opt.unwrap());
            }
        }
        return result;
    }

    fn check_has_attr(attrs: &Vec<Attribute>, name: &str) -> bool {
        for attr in attrs {
            let is_attr = <DefaultAttrParser as AttrParser>::check_is_attr(attr, name);
            if is_attr {
                return true;
            }
        }
        return false;
    }

    fn extract_table_name(ident: &Ident, attrs: &Vec<Attribute>) -> String {
        let mut name = ident.to_string();
        name = <DefaultAttrParser as AttrParser>::extract_val_from_attrs(attrs, "table_name")
            .unwrap_or(name);
        return name;
    }

    fn extract_template_sql(attrs: &Vec<Attribute>) -> Option<String> {
        <DefaultAttrParser as AttrParser>::extract_val_from_attrs(attrs, "sql")
    }
    fn extract_template_count_sql(attrs: &Vec<Attribute>) -> Option<String> {
        <DefaultAttrParser as AttrParser>::extract_val_from_attrs(attrs, "count_sql")
    }

    fn extract_unique_key(attrs: &Vec<Attribute>) -> Vec<Vec<String>> {
        let indexes =
            <DefaultAttrParser as AttrParser>::extract_val_vec_from_attrs(attrs, "unique_key");
        let result: Vec<Vec<String>> = indexes
            .iter()
            .map(|s| s.split(',').map(|e| e.trim().to_string()).collect())
            .collect();
        return result;
    }
}
