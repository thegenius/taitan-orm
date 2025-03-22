use super::base::{
    KeywordsEscaper, MySqlKeywordEscaper, PostgresKeywordEscaper, SqliteKeywordEscaper,
};
use super::mappers::{ArgsMapper, ConditionsMapper, MarksMapper, NamesMapper, RowMapper, SetsMapper, UpsertSetsMapper};
use crate::field_mapper::base::Connector2;
use crate::{DatabaseType, FieldName, FieldTokenType};
use crate::FieldDef;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use std::borrow::Cow;
use case::CaseExt;
use crate::FieldTokenType::{InnerMostType, InnerVariantExpr};

#[derive(Clone, Debug, Default)]
pub struct FieldMapper {
    names_mapper: NamesMapper,
    marks_mapper: MarksMapper,
    sets_mapper: SetsMapper,
    conditions_mapper: ConditionsMapper,
    upsert_sets_mapper: UpsertSetsMapper,
    args_mapper: ArgsMapper,
    row_mapper: RowMapper,
    mysql_escaper: MySqlKeywordEscaper,
    postgres_escaper: PostgresKeywordEscaper,
    sqlite_escaper: SqliteKeywordEscaper,
}

fn generate_nested_vec<'a>(input: Vec<&'a FieldDef<'a>>) -> Vec<Vec<&'a FieldDef<'a>>> {
    input.iter().enumerate().fold(Vec::new(), |mut acc, (i, _)| {
        acc.push(input[..=i].to_vec());
        acc
    })
}

impl FieldMapper {
    pub fn new() -> Self {
        Self::default()
    }

    fn get_escaper(&self, db_type: &DatabaseType) -> &dyn KeywordsEscaper {
        match db_type {
            DatabaseType::MySql => &self.mysql_escaper,
            DatabaseType::Postgres => &self.postgres_escaper,
            DatabaseType::Sqlite => &self.sqlite_escaper,
        }
    }

    pub fn escape<'a>(&'a self, word: &'a str, db_type: &DatabaseType) -> Cow<'a, str> {
        self.get_escaper(db_type).escape(word)
    }

    pub fn gen_add_to_args<'a, T>(&self, fields: T) -> TokenStream
    where
        T: IntoIterator<Item = &'a FieldDef<'a>> + Clone,
    {
        let streams = fields
            .into_iter()
            .map(|def| self.args_mapper.map_add_to_args(def))
            .collect::<Vec<_>>();
        quote! {
            #( #streams; )*
        }
    }

    pub fn gen_enum_add_to_args<'a, T>(&self, fields: T) -> TokenStream
    where
        T: IntoIterator<Item = &'a FieldDef<'a>> + Clone,
    {
        let streams = fields
            .into_iter()
            .map(|def| self.args_mapper.map_enum_add_to_args(def))
            .collect::<Vec<_>>();
        quote! {
            #( #streams; )*
        }
    }

    pub fn gen_template_add_to_args<'a, T>(&self, fields: T) -> TokenStream
    where
        T: IntoIterator<Item = &'a FieldDef<'a>> + Clone,
    {
        let streams = fields
            .into_iter()
            .map(|def|
                {
                    let field_name = def.struct_field.name.get_name();
                    let s = self.args_mapper.map_add_to_args(def);
                    quote! {
                        #field_name => #s,
                    }
                }
            )
            .collect::<Vec<_>>();
        quote! {
            match name {
                #( #streams )*
                _=> unreachable!(),
            }
        }
    }

    pub fn gen_row_try_get<'a, T>(&self, fields: T) -> TokenStream
    where
        T: IntoIterator<Item = &'a FieldDef<'a>> + Clone,
    {
        let streams = fields
            .into_iter()
            .map(|def| self.row_mapper.map_row_try_get(def))
            .collect::<Vec<_>>();
        quote! {
            #( #streams )*
        }
    }

    pub fn gen_struct_fields<'a, T>(&self, fields: T, force_to_option: bool) -> TokenStream
    where
        T: IntoIterator<Item = &'a FieldDef<'a>> + Clone,
    {
        let field_token_type = if force_to_option {
            FieldTokenType::NestedOptionType
        } else {
            FieldTokenType::InnerMostType
        };
        let streams = fields
            .into_iter()
            .map(|def|

                def.struct_field.to_token_stream(field_token_type))
            .collect::<Vec<_>>();
        quote! {
            #( #streams,)*
        }
    }

    pub fn gen_enum_expr_variants<'a, T>(&self, fields: T) -> TokenStream
    where
        T: IntoIterator<Item = &'a FieldDef<'a>> + Clone,
    {
        let streams = fields
            .into_iter()
            .map(|def| def.struct_field.to_token_stream(FieldTokenType::VariantExpr))
            .collect::<Vec<_>>();
        quote! {
            #( #streams,)*
        }
    }

    pub fn gen_enum_variants<'a, T>(&self, fields: T) -> TokenStream
    where
        T: IntoIterator<Item = &'a FieldDef<'a>> + Clone,
    {
        let mut streams :TokenStream = TokenStream::new();
        let fields: Vec<&FieldDef> = fields.into_iter().collect();
        // panic!("{:?}", fields);
        let groups = generate_nested_vec(fields.clone());

        for group in groups {
            let name:String = group.iter().map(|def| {def.struct_field.name.get_name().to_camel()}).collect();
            let variant_name = format_ident!("{}", name);
            let stream = group
                .iter()
                .map(|def| def.struct_field.to_token_stream(InnerVariantExpr))
                .collect::<Vec<_>>();
            streams.extend(quote! {
                 #variant_name{  #( #stream, )* },
            })
        }
        streams
    }



    pub fn gen_selected_default<'a, T>(&self, fields: T) -> TokenStream
    where
        T: IntoIterator<Item = &'a FieldDef<'a>> + Clone,
    {
        let streams = fields
            .into_iter()
            .map(|def| self.row_mapper.map_selected_default(def))
            .collect::<Vec<_>>();
        quote! {
            #( #streams )*
        }
    }

    pub fn gen_names<'a, T>(&self, fields: T, db_type: &DatabaseType) -> TokenStream
    where
        T: IntoIterator<Item = &'a FieldDef<'a>> + Clone,
    {
        self.names_mapper
            ._connect(fields, self.get_escaper(db_type))
    }

    pub fn gen_idents<'a, T>(&self, fields: T) -> TokenStream
    where
        T: IntoIterator<Item = &'a FieldDef<'a>> + Clone,
    {
        let mut idents: Vec<Ident> = Vec::new();
        for field in fields.into_iter() {
            match &field.struct_field.name {
                FieldName::Named(named)=> {
                    idents.push(format_ident!("{}", named));
                }
                FieldName::Unnamed {idx, name}=> {
                    idents.push(format_ident!("e{}", idx));
                }
            }
        }
        quote! {
            #( #idents, )*
        }
    }

    pub fn gen_upsert_sets<'a, T>(&self, fields: T, db_type: &DatabaseType) -> TokenStream
    where
        T: IntoIterator<Item = &'a FieldDef<'a>> + Clone,
    {
        self.upsert_sets_mapper
            ._connect(fields, self.get_escaper(db_type))
    }

    pub fn gen_marks<'a, T>(&self, fields: T, db_type: &DatabaseType) -> TokenStream
    where
        T: IntoIterator<Item = &'a FieldDef<'a>> + Clone,
    {
        match db_type {
            DatabaseType::Postgres => self
                .marks_mapper
                ._connect_indexed(fields, self.get_escaper(db_type)),
            _ => self
                .marks_mapper
                ._connect(fields, self.get_escaper(db_type)),
        }
    }

    // pub fn gen_marks_indexed<'a, T>(&self, fields: T, db_type: &DatabaseType) -> TokenStream
    // where
    //     T: IntoIterator<Item = &'a FieldDef<'a>> + Clone,{
    //     self.marks_mapper
    //         .connect_indexed(fields, self.get_escaper(db_type))
    // }

    pub fn gen_sets<'a, T>(&self, fields: T, db_type: &DatabaseType) -> TokenStream
    where
        T: IntoIterator<Item = &'a FieldDef<'a>> + Clone,
    {
        match db_type {
            DatabaseType::Postgres => self.sets_mapper._connect_indexed(fields, self.get_escaper(db_type)),
            _ => self
                .sets_mapper
                ._connect(fields, self.get_escaper(db_type)),
        }
    }

    // pub fn gen_sets_indexed<'a, T>(&self, fields: T, db_type: &DatabaseType) -> TokenStream
    // where
    //     T: IntoIterator<Item = &'a FieldDef<'a>> + Clone,
    // {
    //     self.sets_mapper
    //         .connect_indexed(fields, self.get_escaper(db_type))
    // }

    pub fn gen_conditions<'a, T>(&self, fields: T, db_type: &DatabaseType, is_enum: bool) -> TokenStream
    where
        T: IntoIterator<Item = &'a FieldDef<'a>> + Clone,
    {
        match db_type {
            DatabaseType::Postgres => {
                self.conditions_mapper
                    ._connect_expr(fields, self.get_escaper(db_type), true, is_enum)
            }
            _ => self
                .conditions_mapper
                ._connect_expr(fields, self.get_escaper(db_type), false, is_enum),
        }
    }

    // pub fn gen_conditions_indexed<'a, T>(&self, fields: T, db_type: &DatabaseType) -> TokenStream
    // where
    //     T: IntoIterator<Item = &'a FieldDef<'a>> + Clone,
    // {
    //     self.conditions_mapper
    //         .connect_dynamic_indexed(fields, self.get_escaper(db_type))
    // }
}
