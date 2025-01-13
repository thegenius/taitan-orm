use crate::fields::mappers::{ArgsAddConstructor, ArgsConstructorMySql, ArgsConstructorPostgres, ArgsConstructorSqlite, NamesAddConstructor, NamesConstructor, RowConstructor, RowGetConstructor, SqlConstructor, SqlConstructors, StructConstructor, StructFieldConstructor};
use crate::fields::{DefaultFieldMapper, FieldMapType, FieldMapper, LocationParser, UniqueParser};
use crate::types::{DefaultTypeChecker, TypeChecker};
use crate::types::{DefaultTypeExtractor, TypeExtractor};
use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::Field;
use syn::FieldsNamed;

pub struct FieldsParser {
    fields: Vec<Field>,
}

impl FieldsParser {
    pub fn from_vec(fields: &Vec<Field>) -> Self {
        Self {
            fields: fields.clone(),
        }
    }
    pub fn from_named(fields: &FieldsNamed) -> Self {
        let fields: Vec<Field> = fields.clone().named.into_iter().collect();
        Self { fields }
    }
}

pub trait FieldsContainer {
    fn get_fields(&self) -> &Vec<Field>;

    fn map_field_vec<F>(&self, wrap_fn: &F) -> Vec<TokenStream>
    where
        F: Fn(Field) -> TokenStream,
    {
        let cloned_names = self.get_fields().clone();
        cloned_names
            .into_iter()
            .map(wrap_fn)
            .collect::<Vec<TokenStream>>()
    }

    fn map_field_vec_with_index<F>(&self, wrap_fn: F) -> Vec<TokenStream>
    where
        F: Fn(&Field, usize) -> TokenStream,
    {
        let cloned_names = self.get_fields().clone();
        cloned_names
            .iter() // 注意这里我们使用 iter() 而不是 into_iter()，因为我们不想消耗 cloned_names
            .enumerate() // 使用 enumerate() 来获取索引
            .map(|(index, field)| wrap_fn(field, index))
            .collect::<Vec<TokenStream>>()
    }
}

impl FieldsContainer for FieldsParser {
    fn get_fields(&self) -> &Vec<Field> {
        &self.fields
    }
}

impl ArgsAddConstructor for FieldsParser {}
impl ArgsConstructorSqlite for FieldsParser {}
impl ArgsConstructorMySql for FieldsParser {}
impl ArgsConstructorPostgres for FieldsParser {}

impl StructFieldConstructor for FieldsParser {}
impl StructConstructor for FieldsParser {}

impl NamesAddConstructor for FieldsParser {}
impl NamesConstructor for FieldsParser {}

impl UniqueParser for FieldsParser {}

impl LocationParser for FieldsParser {}

impl RowGetConstructor for FieldsParser {}

impl RowConstructor for FieldsParser {}

impl SqlConstructor for FieldsParser {}
impl SqlConstructors for FieldsParser {}

impl FieldsParser {
    pub fn map_with<F>(self, map_fn: &F) -> Vec<TokenStream>
    where
        F: Fn(Field) -> TokenStream,
    {
        self.fields
            .into_iter()
            .map(map_fn)
            .collect::<Vec<TokenStream>>()
    }

    // get struct of bool
    pub fn get_bool_fields(&self) -> TokenStream {
        let tokens = <DefaultFieldMapper as FieldMapper>::map_field_vec(&self.fields, &|field| {
            let field_ident = field.ident;
            quote!(#field_ident: bool)
        });
        quote!(#(#tokens, )*)
    }

    // get struct of LocationExpr
    pub fn get_location_expr_fields(&self) -> TokenStream {
        let fields_tokens =
            <DefaultFieldMapper as FieldMapper>::map_field_vec(&self.fields, &|field| {
                let field_ident = field.ident;
                let field_ty = field.ty;
                if <DefaultTypeChecker as TypeChecker>::type_is_option(&field_ty) {
                    let inner_type =
                        <DefaultTypeExtractor as TypeExtractor>::get_option_inner_type(&field_ty);
                    quote!(
                        #field_ident: Option<LocationExpr<#inner_type>>
                    )
                } else {
                    quote!(
                        #field_ident: Option<LocationExpr<#field_ty>>
                    )
                }
            });
        quote!( #(#fields_tokens, )*  )
    }

    // get struct of not option
    pub fn get_not_option_fields(&self) -> TokenStream {
        let fields_tokens =
            <DefaultFieldMapper as FieldMapper>::map_field_vec(&self.fields, &|field| {
                let field_ident = field.ident;
                if <DefaultTypeChecker as TypeChecker>::type_is_option(&field.ty) {
                    let inner_type =
                        <DefaultTypeExtractor as TypeExtractor>::get_option_inner_type(&field.ty)
                            .unwrap();
                    quote!(
                        pub #field_ident: #inner_type
                    )
                } else {
                    let field_ty = field.ty;
                    quote!(
                        pub #field_ident: #field_ty
                    )
                }
            });
        quote!(#(#fields_tokens, )* )
    }

    // get struct of option
    pub fn get_option_fields(&self) -> TokenStream {
        let fields_tokens =
            <DefaultFieldMapper as FieldMapper>::map_field_vec(&self.fields, &|field| {
                let field_ident = field.ident;
                let field_ty = field.ty;
                if <DefaultTypeChecker as TypeChecker>::type_is_option(&field_ty) {
                    quote!(
                        pub #field_ident: #field_ty
                    )
                } else {
                    quote!(
                        pub #field_ident: Option<#field_ty>
                    )
                }
            });
        quote! { #(#fields_tokens, )* }
    }

    pub fn get_construct_fields(&self) -> TokenStream {
        let field_members = self.fields.iter().map(|field| {
            let field_name = field.ident.as_ref().unwrap();
            let span = field_name.span();
            quote_spanned! { span =>
                #field_name
            }
        });
        quote! { #(#field_members, )* }
    }

    // get name of &'static [&'static str]
    pub fn get_name_array(&self) -> TokenStream {
        let tokens =
            <DefaultFieldMapper as FieldMapper>::map_field_vec(&self.fields, &|field: Field| {
                <DefaultFieldMapper as FieldMapper>::map_field(field, FieldMapType::Str)
            });
        quote!(
            &[ #(#tokens, )* ]
        )
    }

    // get name of Vec<String>
    pub fn get_name_vec(&self) -> TokenStream {
        let tokens =
            <DefaultFieldMapper as FieldMapper>::map_field_vec(&self.fields, &|field: Field| {
                <DefaultFieldMapper as FieldMapper>::map_field(field, FieldMapType::String)
            });
        quote!(vec![ #(#tokens, )* ])
    }

    pub fn get_option_name_vec(&self) -> TokenStream {
        let tokens =
            <DefaultFieldMapper as FieldMapper>::map_field_vec(&self.fields, &|field: Field| {
                <DefaultFieldMapper as FieldMapper>::map_field(field, FieldMapType::OptionNamePush)
            });
        quote!(
            let mut fields = Vec::new();
            #(#tokens;)*
            return fields;
        )
    }

    // pub fn get_maybe_option_name_vec(&self) -> TokenStream {
    //     let tokens =
    //         DefaultFieldMapper::map_field_vec(&self.fields, &|field: Field| {
    //             let field_type = &field.ty;
    //             if DefaultTypeChecker::type_is_option(field_type) {
    //                 DefaultFieldMapper::map_field(
    //                     field,
    //                     FieldMapType::OptionNamePush,
    //                 )
    //             } else {
    //                 DefaultFieldMapper::map_field(field, FieldMapType::NamePush)
    //             }
    //         });
    //     quote!(
    //         let mut fields = Vec::new();
    //         #(#tokens;)*
    //         return fields;
    //     )
    // }

    pub fn get_bool_name_vec(&self) -> TokenStream {
        let tokens = DefaultFieldMapper::map_field_vec(&self.fields, &|field: Field| {
            DefaultFieldMapper::map_field(field, FieldMapType::BoolPush)
        });
        quote!(
            let mut fields = Vec::new();
            #(#tokens)*
            return fields;
        )
    }

    pub fn get_option_args(&self) -> TokenStream {
        let args_add_clause =
            <DefaultFieldMapper as FieldMapper>::map_field_vec(&self.fields, &|field: Field| {
                let field_name = field.ident.unwrap();
                let span = field_name.span();
                quote_spanned! { span =>
                    if let Some(#field_name) = &self.#field_name {
                        luna_add_arg(&mut arguments, &#field_name);
                    }
                }
            });

        quote! {
            let mut arguments = AnyArguments::default();
            #(#args_add_clause)*
            arguments
        }
    }

    pub fn get_args(&self) -> TokenStream {
        let args_add_clause =
            <DefaultFieldMapper as FieldMapper>::map_field_vec(&self.fields, &|field: Field| {
                let field_name = field.ident.unwrap();
                let span = field_name.span();
                quote_spanned! { span =>
                    luna_add_arg(&mut arguments, &self.#field_name);
                }
            });
        quote! {
            let mut arguments = AnyArguments::default();
            #(#args_add_clause)*
            arguments
        }
    }

    // pub fn get_maybe_option_args_sqlite(&self) -> TokenStream {
    //     let args_add_clause = DefaultFieldMapper::map_field_vec(
    //         &self.fields,
    //         &DefaultFieldMapper::map_to_maybe_option_args_add,
    //     );
    //     quote! {
    //         let mut args = SqliteArguments::default();
    //         #(#args_add_clause)*
    //         Ok(args)
    //     }
    // }

    // pub fn get_maybe_option_args_mysql(&self) -> TokenStream {
    //     let args_add_clause = <DefaultFieldMapper as FieldMapper>::map_field_vec(
    //         &self.fields,
    //         &<DefaultFieldMapper as FieldMapper>::map_to_maybe_option_args_add,
    //     );
    //     quote! {
    //         let mut args = MySqlArguments::default();
    //         #(#args_add_clause)*
    //         Ok(args)
    //     }
    // }

    // pub fn get_maybe_option_args_postgres(&self) -> TokenStream {
    //     let args_add_clause = <DefaultFieldMapper as FieldMapper>::map_field_vec(
    //         &self.fields,
    //         &<DefaultFieldMapper as FieldMapper>::map_to_maybe_option_args_add,
    //     );
    //     quote! {
    //         let mut args = PgArguments::default();
    //         #(#args_add_clause)*
    //         Ok(args)
    //     }
    // }

    pub fn get_maybe_option_args(&self) -> TokenStream {
        let args_add_clause = <DefaultFieldMapper as FieldMapper>::map_field_vec(
            &self.fields,
            &<DefaultFieldMapper as FieldMapper>::map_to_any_args_add,
        );
        quote! {
            let mut arguments = AnyArguments::default();
            #(#args_add_clause)*
            arguments
        }
    }

    pub fn get_option_location_args(&self) -> TokenStream {
        let args_add_clause =
            <DefaultFieldMapper as FieldMapper>::map_field_vec(&self.fields, &|field: Field| {
                let field_name = field.ident.unwrap();
                let span = field_name.span();
                quote_spanned! { span =>
                    if let Some(#field_name) = &self.#field_name {
                        luna_add_arg(&mut arguments, &#field_name.val);
                    }
                }
            });
        quote! {
            let mut arguments = AnyArguments::default();
            #(#args_add_clause)*
            arguments
        }
    }

    pub fn get_where_clause(&self) -> TokenStream {
        let where_clause_members = <DefaultFieldMapper as FieldMapper>::map_field_vec(
            &self.fields,
            &<DefaultFieldMapper as FieldMapper>::map_to_where_field,
        );
        quote! {
            let mut sql = String::default();
            let connectives = self.mode.as_connective();
            #(#where_clause_members )*

            return sql.strip_suffix(connectives)
            .unwrap_or(sql.as_str())
            .to_string();
        }
    }

    pub fn get_enum_where_clause(&self) -> TokenStream {
        let where_clause_members = <DefaultFieldMapper as FieldMapper>::map_field_vec(
            &self.fields,
            &<DefaultFieldMapper as FieldMapper>::map_to_enum_where_field,
        );
        quote! {
            let mut sql = String::default();
            match self {
                #(#where_clause_members )*
            }
            return sql;
        }
    }

    pub fn get_unique_index_check(&self, unique_indexes: &Vec<Vec<String>>) -> TokenStream {
        if unique_indexes.is_empty() {
            quote!(fields.len() == 0)
        } else {
            let mut check_token_stream = quote!();
            for unique_index in unique_indexes {
                let check_token = quote!(
                    if array_str_equal(&[#(#unique_index,)*], fields) {
                        return true;
                    }
                );
                check_token_stream.extend(check_token);
            }
            check_token_stream.extend(quote!(
                return false;
            ));
            check_token_stream
        }
    }

    pub fn get_row_construct(&self) -> TokenStream {
        // let row_get_stmts = <DefaultFieldMapper as FieldMapper>::map_field_vec(&self.fields, &|field: Field| {
        //     let field_name = field.ident.unwrap();
        //     let span = field_name.span();
        //     let field_name_lit = LitStr::new(&field_name.to_string(), span);
        //     let field_type = field.ty;
        //     if <DefaultTypeChecker as TypeChecker>::type_is_option(&field_type) {
        //         quote_spanned! { span =>
        //             let #field_name: #field_type = row.try_get(#field_name_lit).ok();
        //         }
        //     } else {
        //         quote_spanned! { span =>
        //             let #field_name: Option<#field_type> = row.try_get(#field_name_lit).ok();
        //         }
        //     }
        // });
        let row_get_stmts = <DefaultFieldMapper as FieldMapper>::map_field_vec(
            &self.fields,
            &<DefaultFieldMapper as FieldMapper>::map_to_selected_field,
        );
        let construct_fields = self.get_construct_fields();

        quote! {
            #(#row_get_stmts)*
            let result = Self { #construct_fields  };
            Ok(result)
        }
    }
}
