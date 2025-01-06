use crate::attrs::{AttrParser, DefaultAttrParser};
use crate::types::{DefaultTypeChecker, TypeChecker};
use case::CaseExt;
use proc_macro2::TokenStream;
use quote::{format_ident, quote_spanned};
use syn::spanned::Spanned;
use syn::{Field, LitStr};

pub trait NamesAddConstructor {
    fn of_str(field: Field) -> TokenStream {
        let span = field.span();
        let field_name = field.ident.unwrap();
        let field_name_string = LitStr::new(&field_name.to_string(), span);
        quote_spanned! { span=> #field_name_string }
    }

    fn of_string(field: Field) -> TokenStream {
        let span = field.span();
        let field_name = field.ident.unwrap();
        let field_name_string = LitStr::new(&field_name.to_string(), span);
        quote_spanned! { span=> #field_name_string.to_string() }
    }

    fn of_maybe_option(field: Field) -> TokenStream {
        let field_alias = DefaultAttrParser::extract_field_db_ident(&field);
        let field_type = &field.ty;
        let span = field.span();
        let field_name = field.ident.unwrap();
        let field_name_string = LitStr::new(&field_alias.to_string(), span);
        // if DefaultTypeChecker::type_is_option(field_type) {
        //     quote_spanned! { span=>
        //         if self.#field_name.not_none() {
        //             fields.push(#field_name_string.to_string());
        //         }
        //     }
        // } else {
        //     quote_spanned! { span=>
        //         fields.push(#field_name_string.to_string());
        //     }
        // }

        if DefaultTypeChecker::type_is_option(field_type) {
            quote_spanned! { span=>
                match &self.#field_name {
                    taitan_orm::result::Optional::Some(#field_name) => {
                        fields.push(taitan_orm::prelude::FieldName::from_str(#field_name_string, false));
                    }
                    taitan_orm::result::Optional::Null => {
                        fields.push(taitan_orm::prelude::FieldName::from_str(#field_name_string, true));
                    }
                    _ => {}
                };
            }
        } else {
            quote_spanned! { span=>
                fields.push(taitan_orm::prelude::FieldName::from_str(#field_name_string, false));
            }
        }
    }

    fn of_option(field: Field) -> TokenStream {
        let field_alias = DefaultAttrParser::extract_field_db_ident(&field);
        let span = field.span();
        let field_name = field.ident.unwrap();
        let field_name_string = LitStr::new(&field_alias.to_string(), span);

        quote_spanned! { span=>
            match &self.#field_name {
                taitan_orm::result::Optional::Some(#field_name) => {
                    fields.push(taitan_orm::prelude::FieldName::from_str(#field_name_string, false));
                }
                taitan_orm::result::Optional::Null => {
                    fields.push(taitan_orm::prelude::FieldName::from_str(#field_name_string, true));
                }
                _ => {}
            };
        }
    }

    fn of_enum(field: Field) -> TokenStream {
        let field_alias = DefaultAttrParser::extract_field_db_ident(&field);
        let span = field.span();
        let field_name = field.ident.unwrap().to_string();
        let enum_ident = format_ident!("{}", field_name.to_camel());
        let field_name_string = LitStr::new(&field_alias.to_string(), span);

        quote_spanned! { span=>
            Self::#enum_ident(_) => fields.push(taitan_orm::prelude::FieldName::from_str(#field_name_string, false)),
        }
    }

    fn of_not_option(field: Field) -> TokenStream {
        let field_alias = DefaultAttrParser::extract_field_db_ident(&field);
        let span = field.span();
        // let field_name = field.ident.unwrap();
        let field_name_string = LitStr::new(&field_alias.to_string(), span);
        quote_spanned! { span=>
            fields.push(#field_name_string.to_string());
        }
    }

    fn of_option_bool(field: Field) -> TokenStream {
        let span = field.span();
        let field_name = field.ident.unwrap();
        let field_name_string = LitStr::new(&field_name.to_string(), span);
        quote_spanned! { span=>
            if let taitan_orm::result::Optional::Some(true) = self.#field_name {
                fields.push(#field_name_string.to_string());
            }
        }
    }
    fn of_bool(field: Field) -> TokenStream {
        let field_alias = DefaultAttrParser::extract_field_db_ident(&field);
        let span = field.span();
        let field_name = field.ident.unwrap();
        let field_name_string = LitStr::new(&field_alias.to_string(), span);
        quote_spanned! { span=>
            if self.#field_name {
                fields.push(#field_name_string.to_string());
            }
        }
    }

    fn of_self_optional(field: Field) -> TokenStream {
        let field_alias = DefaultAttrParser::extract_field_db_ident(&field);
        let span = field.span();
        let field_name = field.ident.unwrap();
        let field_name_string = LitStr::new(&field_alias.to_string(), span);
        if DefaultTypeChecker::type_is_option(&field.ty) {
            quote_spanned! { span=>
                if self.#field_name.is_selected() {
                    fields.push(#field_name_string.to_string());
                }
            }
        } else {
            quote_spanned! { span=>
                fields.push(#field_name_string.to_string());
            }
        }
    }

    fn of_bool_bit(field: Field) -> TokenStream {
        // let field_alias = DefaultAttrParser::extract_field_db_ident(&field);
        let span = field.span();
        let field_name = field.ident.unwrap();
        // let field_name_string = LitStr::new(&field_alias.to_string(), span);
        quote_spanned! { span=>
            bits.push(self.#field_name);
        }
    }

    fn of_where_seg(field: Field) -> TokenStream {
        let field_alias = DefaultAttrParser::extract_field_db_ident(&field);
        let field_name = field.ident.unwrap();
        let span = field_name.span();
        let field_name_lit = LitStr::new(&field_alias.to_string(), span);
        quote_spanned! { span =>
            if let taitan_orm::result::Optional::Some(#field_name) = &self.#field_name {
                sql.push(wrap_char);
                sql.push_str(#field_name_lit);
                sql.push(wrap_char);
                sql.push_str(#field_name.cmp.get_sql());
                sql.push(place_holder);
            }
        }
    }
}
