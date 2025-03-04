use case::CaseExt;
use proc_macro2::{Ident, TokenTree};
use quote::__private::ext::RepToTokensExt;
use syn::{Attribute, Expr, Field, Lit, Meta, Path};
use quote::format_ident;

pub trait AttrParser {
    fn extract_field_db_ident(field: &Field) -> Ident;
    fn extract_val_from_attr(attr: &Attribute, name: &str) -> Option<String>;
    fn extract_path_val_from_attr(attr: &Attribute, path: &str) -> Option<IndexAttribute>;

    fn check_is_attr(attr: &Attribute, name: &str) -> bool;

    fn extract_val_from_attrs(attrs: &Vec<Attribute>, name: &str) -> Option<String>;
    fn extract_index_fields(attrs: &Vec<Attribute>) -> Vec<IndexAttribute>;

    fn extract_val_vec_from_attrs(attrs: &Vec<Attribute>, name: &str) -> Vec<String>;
    // fn extract_path_val_from_attrs(attrs: &Vec<Attribute>, name: &str) -> Vec<String>;

    fn check_has_attr(attrs: &Vec<Attribute>, name: &str) -> bool;

    fn extract_table_name(ident: &Ident, attrs: &Vec<Attribute>) -> String;

    fn extract_template_sql(attrs: &Vec<Attribute>) -> Option<String>;
    fn extract_template_count_sql(attrs: &Vec<Attribute>) -> Option<String>;

    fn extract_unique_key(attrs: &Vec<Attribute>) -> Vec<Vec<String>>;

    fn extract_serde_names(attrs: &Vec<Attribute>) -> Vec<&'static str>;
}

pub struct DefaultAttrParser {}


#[derive(Debug, Clone)]
pub struct IndexAttribute {
    pub name: String,
    pub fields: Vec<String>,
}

impl AttrParser for DefaultAttrParser {

    fn extract_path_val_from_attr(attr: &Attribute, name: &str) -> Option<IndexAttribute> {
        let path: &Path = attr.path();
        if !path.is_ident(name) {
            return None;
        }


        let mut name: Option<String> = None;
        let mut fields = Vec::new();
        match &attr.meta {
            Meta::List(meta_list) => {
                let token_stream: proc_macro2::TokenStream = meta_list.clone().tokens;
                let mut tokens = token_stream.into_iter();
                while let Some(token) = tokens.next() {
                    match token {
                        proc_macro2::TokenTree::Ident(ident) if ident == "name" => {
                            if let Some(TokenTree::Punct(punct)) = tokens.next() {
                                if punct.as_char() == '=' {
                                    if let Some(TokenTree::Literal(lit)) = tokens.next() {
                                        name = Some(lit.to_string().trim_matches('"').to_string());
                                    }
                                }
                            }
                        },
                        TokenTree::Ident(ident) if ident == "fields" => {
                            if let Some(TokenTree::Group(group)) = tokens.next() {
                                for field in group.stream() {
                                    if let TokenTree::Literal(lit) = field {
                                        fields.push(lit.to_string().trim_matches('"').to_string());
                                    }
                                }
                            }
                        }
                        _=>{}
                    }
                }
            },
            _ => {},
        }

        if name.is_some() {
            return Some(IndexAttribute {
                name: name.unwrap(),
                fields
            });
        } else {
            return None;
        }
    }

    fn extract_val_from_attr(attr: &Attribute, name: &str) -> Option<String> {
        let path: &Path = attr.path();
        if !path.is_ident(name) {
            return None;
        }

        // if name == "index" {
        //     panic!("{:?}", attr);
        // }


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

    // fn extract_path_val_from_attrs(attrs: &Vec<Attribute>, path: &str) -> Vec<String> {
    //     let mut values:Vec<String> = Vec::new();
    //     for attr in attrs {
    //         let val_opt = <DefaultAttrParser as AttrParser>::extract_path_val_from_attr(attr, path);
    //         if val_opt.is_some() {
    //             values.push(val_opt.unwrap());
    //         }
    //     }
    //     values
    // }

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

    fn extract_index_fields(attrs: &Vec<Attribute>) -> Vec<IndexAttribute> {
        let mut results: Vec<IndexAttribute> = Vec::new();
        for attr in attrs {
            let val_opt = <DefaultAttrParser as AttrParser>::extract_path_val_from_attr(attr, "index");
            if let Some(val) = val_opt {
                results.push(val);
            }
        }
        results
    }
    fn extract_serde_names(attrs: &Vec<Attribute>) -> Vec<&'static str> {
        let mut results: Vec<&'static str> = Vec::new();
        for attr in attrs {
            let val_opt = <DefaultAttrParser as AttrParser>::extract_val_from_attr(attr, "serde_struct");
            if let Some(val) = val_opt {
                if val.eq("primary") {
                    results.push("primary");
                }
                if val.eq("unique") {
                    results.push("unique");
                }
                if val.eq("location") {
                    results.push("location");
                }
                if val.eq("mutation") {
                    results.push("mutation");
                }
                if val.eq("selected") {
                    results.push("selected");
                }
            }
        }
        results
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
        let name = ident.to_string().to_snake();
        <DefaultAttrParser as AttrParser>::extract_val_from_attrs(attrs, "table_name")
            .unwrap_or(name)
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
