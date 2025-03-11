use proc_macro2::TokenStream;
use quote::{format_ident, quote};

#[derive(Debug, Clone, PartialEq)]
pub enum SqlSegment {
    Simple(String),
    Hash(String),
    At(String),
    Dollar(String),
}

impl SqlSegment {
    pub fn to_sql(&self, indexed: bool) -> TokenStream {
        match self {

            SqlSegment::Simple(s) => {
                let content = format!("{} ", s);
                quote! { s.push_str(#content); }
            }
            SqlSegment::Hash(s) => {
                if indexed {
                    quote! {
                        index += 1;
                        sql.push_str(format!("${}", index).as_str());
                    }
                } else {
                    quote! { sql.push('?'); }
                }
            }
            SqlSegment::At(s) => {
                let ident = format_ident!("{}", s);
                if indexed {
                    quote! {
                        if self.#ident.is_some() {
                            index += 1;
                            sql.push_str(format!("${}", index).as_str());
                        }
                    }
                } else {
                    quote! {
                        if self.#ident.is_some() {
                            s.push('?');
                        }
                    }
                }
            }
            SqlSegment::Dollar(s) => {
                let template = format!("{{ {} }}", s);
                quote! { s.push_str(#template); }
            }
        }
    }
}

pub trait ToSqlSegment {
    // ${ name } 在to_sql的时候替换为 {{ name }}
    // #{ name } 替换为 ?，并且variables里面返回名称，保证后续的绑定

    fn gen_sql_segment(&self) -> SqlSegment {
        unimplemented!()
    }

    fn gen_sql_segments(&self) -> Vec<SqlSegment> {
        vec![self.gen_sql_segment()]
    }

    // #{ name } 替换为 ?，并且variables里面返回名称，保证后续的绑定
    // @{ name } 替换为 if self.name.is_some() { v.push("name".to_string()); }
}
