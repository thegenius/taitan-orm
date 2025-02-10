use proc_macro2::TokenTree;
use syn::{Attribute, Expr, Lit, Meta, Path};

pub fn is_attr(attr: &Attribute, name: &str) -> bool {
    let path: &Path = attr.path();
    let path_ident = path.get_ident().unwrap();
    let attr_path_name = path_ident.to_string();
    attr_path_name == name
}
pub fn has_attr(attrs: &[Attribute], name: &str) -> bool {
    attrs.iter().any(|attr| is_attr(attr, name))
}
pub fn extract_attr_val(attr: &Attribute) -> Option<String> {
    if let Meta::NameValue(name_value) = &attr.meta {
        if let Expr::Lit(s) = &name_value.value {
            if let Lit::Str(s) = &s.lit {
                return Some(s.value());
            }
        }
    }
    None
}

pub fn extract_named_attr_val(attr: &Attribute, name: &str) -> Option<String> {
    let path: &Path = attr.path();
    if !path.is_ident(name) {
        return None;
    }
    extract_attr_val(attr)
}

pub fn extract_named_attrs_val(attrs: &[Attribute], name: &str) -> Vec<String> {
    attrs
        .iter()
        .map(|attr| extract_named_attr_val(attr, name))
        .filter(Option::is_some)
        .map(Option::unwrap)
        .collect()
}

pub fn extract_named_attr_vals(attr: &Attribute, name: &str) -> Vec<String> {
    let mut fields = Vec::new();
    match &attr.meta {
        Meta::List(meta_list) => {
            let mut token_stream = meta_list.clone().tokens;
            let mut tokens = token_stream.into_iter();
            while let Some(token) = tokens.next() {
                match token {
                    TokenTree::Ident(ident) if ident == &name => {
                        if let Some(TokenTree::Group(group)) = tokens.next() {
                            for field in group.stream() {
                                if let TokenTree::Literal(lit) = field {
                                    fields.push(lit.to_string().trim_matches('"').to_string());
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        _ => {}
    }
    fields
}

#[derive(Debug, Clone)]
pub struct IndexAttribute {
    pub name: String,
    pub fields: Vec<String>,
}
pub fn extract_index_attr(attr: &Attribute, name: &str) -> Option<IndexAttribute> {
    let path: &Path = attr.path();
    if !path.is_ident(name) {
        return None;
    }
    let name_values: Vec<String> = extract_named_attr_vals(attr, "name");
    let fields_values: Vec<String> = extract_named_attr_vals(attr, "fields");
    if name_values.len() != 1 || fields_values.is_empty() {
        return None;
    }
    let name = name_values.first().unwrap();
    Some(IndexAttribute {
        name: name.to_string(),
        fields: fields_values,
    })
}
