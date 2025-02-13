use quote::ToTokens;
use std::borrow::Cow;
use syn::punctuated::Punctuated;
use syn::{Attribute, Expr, Lit, Meta, Path, Token};

pub struct AttrParser;

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct NamedAttribute<'a> {
    pub name: Cow<'a, str>,
    pub values: Vec<Cow<'a, str>>,
}

impl<'a> NamedAttribute<'a> {
    pub fn from_str<N, F>(name: N, val_str: F) -> Self
    where
        N: Into<Cow<'a, str>>,
        F: Into<Cow<'a, str>>,
    {
        let value_str: String = val_str.into().into_owned();
        let values: Vec<Cow<'a, str>> = value_str
            .split(|c| c == ' ' || c == ',')
            .filter(|s| !s.is_empty())
            .map(|s| Cow::Owned(s.to_string()))
            .collect();
        Self {
            name: name.into(),
            values,
        }
    }
    pub fn from_vec<N>(name: N, values: Vec<Cow<'a, str>>) -> Self
    where
        N: Into<Cow<'a, str>>,
    {
        Self {
            name: name.into(),
            values,
        }
    }
}
impl AttrParser {
    pub fn is_attr(attr: &Attribute, name: &str) -> bool {
        let path: &Path = attr.path();
        let path_ident = path.get_ident().unwrap();
        let attr_path_name = path_ident.to_string();
        attr_path_name == name
    }
    pub fn has_attr(attrs: &[Attribute], name: &str) -> bool {
        attrs.iter().any(|attr| Self::is_attr(attr, name))
    }

    pub fn get_attr(attrs: &[Attribute], name: &str) -> Option<Attribute> {
        attrs.iter().find(|a| Self::is_attr(a, name)).cloned()
    }
    pub fn get_attrs(attrs: &[Attribute], name: &str) -> Vec<Attribute> {
        attrs.iter().filter(|a| Self::is_attr(a, name)).cloned().collect()
    }
    pub fn parse(attr: &Attribute) -> Option<NamedAttribute<'_>> {
        let path: &Path = attr.path();
        let attr_name = path.get_ident().unwrap().to_string();
        match &attr.meta {
            Meta::NameValue(name_value) => match &name_value.value {
                Expr::Lit(s) => match &s.lit {
                    Lit::Str(s) => Some(NamedAttribute::from_str(attr_name, s.value())),
                    _ => None,
                },
                Expr::Path(expr_path) => {
                    let segments = &expr_path.path.segments;
                    if segments.is_empty() {
                        return None;
                    }
                    let first_segment = segments.first().unwrap();
                    Some(NamedAttribute::from_str(
                        attr_name,
                        first_segment.ident.to_string(),
                    ))
                }
                _ => None,
            },
            Meta::List(list) => {
                if let Ok(expr) = list.parse_args::<Expr>() {
                    return match expr {
                        Expr::Lit(expr_lit) => {
                            // 处理 #[attr("val")]
                            if let Lit::Str(s) = expr_lit.lit {
                                return Some(NamedAttribute::from_str(attr_name, s.value()));
                            }
                            None
                        }
                        Expr::Path(expr_path) => {
                            let segments = expr_path.path.segments;
                            if segments.is_empty() {
                                return None;
                            }
                            let first_segment = segments.first().unwrap();
                            Some(NamedAttribute::from_str(
                                attr_name,
                                first_segment.ident.to_string(),
                            ))
                        }
                        _ => None,
                    };
                }
                if let Ok(value_list) =
                    list.parse_args_with(Punctuated::<Expr, Token![,]>::parse_terminated)
                {
                    let values = value_list
                        .into_iter()
                        .map(|v| Cow::Owned(v.to_token_stream().to_string()))
                        .collect();
                    return Some(NamedAttribute::from_vec(attr_name, values));
                }

                None
            }
            _ => None,
        }
    }

    pub fn parse_multi(attr: &Attribute) -> Vec<NamedAttribute<'_>> {
        let path: &Path = attr.path();
        let attr_name = path.get_ident().unwrap().to_string();
        match &attr.meta {
            Meta::NameValue(name_value) => match &name_value.value {
                Expr::Lit(s) => match &s.lit {
                    Lit::Str(s) => vec![NamedAttribute::from_str(attr_name, s.value())],
                    _ => vec![],
                },
                Expr::Path(expr_path) => {
                    let segments = &expr_path.path.segments;
                    if segments.is_empty() {
                        return vec![];
                    }
                    let first_segment = segments.first().unwrap();
                    vec![NamedAttribute::from_str(
                        attr_name,
                        first_segment.ident.to_string(),
                    )]
                }
                _ => vec![],
            },
            Meta::List(list) => {
                if let Ok(expr_list) =
                    list.parse_args_with(Punctuated::<Expr, Token![,]>::parse_terminated)
                {
                    let values: Vec<NamedAttribute> = expr_list
                        .into_iter()
                        .map(|expr| {
                            if let Expr::Assign(assign) = expr {
                                let name = assign.left.into_token_stream().to_string();
                                let values_str = assign.right.into_token_stream().to_string();
                                let values_inner_str = extract_inner_string(values_str.as_str());
                                return Some(NamedAttribute::from_str(name, values_inner_str));
                            }
                            return None;
                        })
                        .filter(|v| v.is_some())
                        .map(|v| v.unwrap())
                        .collect();
                    return values;
                }

                if let Ok(expr) = list.parse_args::<Expr>() {
                    return match expr {
                        Expr::Lit(expr_lit) => {
                            // 处理 #[attr("val")]
                            if let Lit::Str(s) = expr_lit.lit {
                                return vec![NamedAttribute::from_str(attr_name, s.value())];
                            }
                            vec![]
                        }
                        Expr::Path(expr_path) => {
                            let segments = expr_path.path.segments;
                            if segments.is_empty() {
                                return vec![];
                            }
                            let first_segment = segments.first().unwrap();
                            return vec![NamedAttribute::from_str(
                                attr_name,
                                first_segment.ident.to_string(),
                            )];
                        }
                        _ => vec![],
                    };
                }
                if let Ok(value_list) =
                    list.parse_args_with(Punctuated::<Expr, Token![,]>::parse_terminated)
                {
                    let values = value_list
                        .into_iter()
                        .map(|v| Cow::Owned(v.to_token_stream().to_string()))
                        .collect();
                    return vec![NamedAttribute::from_vec(attr_name, values)];
                }

                vec![]
            }
            _ => vec![],
        }
    }
}

fn extract_inner_string(s: &str) -> String {
    let trimmed = s.trim();
    if trimmed.starts_with('(') && trimmed.ends_with(')') {
        trimmed[1..trimmed.len() - 1].trim().to_string()
    } else if trimmed.starts_with('"') && trimmed.ends_with('"') {
        trimmed[1..trimmed.len() - 1].trim().to_string()
    } else {
        trimmed.to_string()
    }
}
