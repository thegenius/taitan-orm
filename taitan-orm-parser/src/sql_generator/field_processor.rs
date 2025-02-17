use crate::field_mapper::{CommaType, FieldMapper};
use crate::sql_generator::keywords_escaper::{
    KeywordsEscaper, MySqlKeywordEscaper, PostgresKeywordEscaper, SqliteKeywordEscaper,
};
use crate::FieldDef;
use proc_macro2::TokenStream;
use quote::{quote};
use std::borrow::Cow;

use super::FieldGroup;
use super::FieldGroupList;


pub trait FieldProcessor {
    type Escaper: KeywordsEscaper;
    fn get_escaper(&self) -> &Self::Escaper;

    fn gen_list_stream(&self, fields: &[FieldDef]) -> TokenStream {
        let field_group_list = FieldGroupList::from(fields);
        if field_group_list.is_all_required {
            let list_string = FieldMapper::gen_names_string(fields, self.get_escaper());
            return quote! {
                let fields = String::from(#list_string);
            };
        }
        let mut stream = TokenStream::new();
        let groups = field_group_list.groups;
        let first_required_index = field_group_list.first_required;
        for (index, group) in groups.iter().enumerate() {
            let comma_type = CommaType::parse(index, first_required_index);
            match group {
                FieldGroup::Optional(field) => {
                    stream.extend(FieldMapper::add_name(
                        field,
                        self.get_escaper(),
                        &comma_type,
                        true
                    ));
                }
                FieldGroup::Required(fields) => {
                    stream.extend(FieldMapper::add_names(
                        fields,
                        self.get_escaper(),
                        &comma_type,
                    ));
                }
            }
        }
        quote! {
            let fields = {
                let mut fields = String::default();
                let mut has_prev = false;
                #stream
                fields
            };
        }
    }

    fn gen_marks_stream(&self, fields: &[FieldDef]) -> TokenStream {
        let field_group_list = FieldGroupList::from(fields);
        if field_group_list.is_all_required {
            let list_string = FieldMapper::gen_plain_marks(fields);
            return quote! {
                let marks = String::from(#list_string);
            };
        }
        let mut stream = TokenStream::new();
        let groups = field_group_list.groups;
        let first_required_index = field_group_list.first_required;
        for (index, group) in groups.iter().enumerate() {
            match group {
                FieldGroup::Optional(field) => {
                    let comma_type = CommaType::parse(index, first_required_index);
                    stream.extend(FieldMapper::add_mark(field, &comma_type, true));
                }
                FieldGroup::Required(fields) => {
                    let comma_type = CommaType::parse(index, first_required_index);
                    stream.extend(FieldMapper::add_marks(fields, &comma_type, false));
                }
            }
        }
        quote! {
            let marks = {
                let mut marks = String::default();
                let mut has_prev = false;
                #stream
                marks
            };
        }
    }
}

#[derive(Default)]
pub struct MySqlFieldProcessor {
    escaper: MySqlKeywordEscaper,
}
impl FieldProcessor for MySqlFieldProcessor {
    type Escaper = MySqlKeywordEscaper;
    fn get_escaper(&self) -> &Self::Escaper {
        &self.escaper
    }
}

#[derive(Default)]
pub struct PostgresFieldProcessor {
    escaper: PostgresKeywordEscaper,
}
impl FieldProcessor for PostgresFieldProcessor {
    type Escaper = PostgresKeywordEscaper;
    fn get_escaper(&self) -> &Self::Escaper {
        &self.escaper
    }

    fn gen_marks_stream(&self, fields: &[FieldDef]) -> TokenStream {
        let field_group_list = FieldGroupList::from(fields);
        if field_group_list.is_all_required {
            let list_string = FieldMapper::gen_plain_marks(fields);
            return quote! {
                String::from(#list_string)
            };
        }
        let mut stream = TokenStream::new();
        let groups = field_group_list.groups;
        let first_required_index = field_group_list.first_required;
        for (index, group) in groups.iter().enumerate() {
            match group {
                FieldGroup::Optional(field) => {
                    let comma_type = CommaType::parse(index, first_required_index);
                    stream.extend(FieldMapper::add_indexed_mark(field, &comma_type, true));
                }
                FieldGroup::Required(fields) => {
                    let comma_type = CommaType::parse(index, first_required_index);
                    if index == first_required_index {
                        stream.extend(FieldMapper::add_marks(fields, &comma_type, true));
                    } else {
                        for field in fields {
                            stream.extend(FieldMapper::add_indexed_mark(field, &comma_type, false));
                        }
                    }
                }
            }
        }
        quote! {
            let marks = {
                let mut marks = String::default();
                let mut has_prev = false;
                let mut index: usize = 0;
                #stream
                marks
            };
        }
    }
}

#[derive(Default)]
pub struct SqliteFieldProcessor {
    escaper: SqliteKeywordEscaper,
}
impl FieldProcessor for SqliteFieldProcessor {
    type Escaper = SqliteKeywordEscaper;
    fn get_escaper(&self) -> &Self::Escaper {
        &self.escaper
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use super::*;
    use crate::{StructFieldDef, TableDef};
    use syn::{parse_quote, DeriveInput};
    #[test]
    pub fn test_split_fields() {
        let field_def1 = FieldDef {
            struct_field: StructFieldDef {
                is_optional: false,
                ..StructFieldDef::default()
            },
            ..FieldDef::default()
        };
        let field_def2 = field_def1.clone();
        let field_def3 = field_def1.clone();
        let field_def4 = FieldDef {
            struct_field: StructFieldDef {
                is_optional: true,
                ..StructFieldDef::default()
            },
            ..FieldDef::default()
        };
        let field_def5 = field_def4.clone();
        let field_def6 = field_def4.clone();
        let field_def7 = field_def1.clone();
        let field_def8 = field_def1.clone();

        let fields: Vec<FieldDef> = vec![
            field_def1, field_def2, field_def3, field_def4, field_def5, field_def6, field_def7,
            field_def8,
        ];
        let fields_group = FieldGroupList::from(fields.as_slice()).groups;
        assert_eq!(fields_group.len(), 5);
        assert_eq!(fields_group[0].len(), 3);
        assert_eq!(fields_group[0].is_optional(), false);

        assert_eq!(fields_group[1].len(), 1);
        assert_eq!(fields_group[1].is_optional(), true);

        assert_eq!(fields_group[2].len(), 1);
        assert_eq!(fields_group[2].is_optional(), true);

        assert_eq!(fields_group[3].len(), 1);
        assert_eq!(fields_group[3].is_optional(), true);

        assert_eq!(fields_group[4].len(), 2);
        assert_eq!(fields_group[4].is_optional(), false);
    }

    #[test]
    pub fn test_mysql() {
        let input: DeriveInput = parse_quote! {
            #[primary(a, b)]
            struct Foo<'a, 'b> {
                a: &'a str,
                b: Cow<'b, str>,
                c: String,
                d: Option<Cow<'b, str>>,
                e: Optional<Cow<'b, str>>
            }
        };

        let table_def = TableDef::parse(&input);
        let field_list = FieldMapper::gen_names_string(&table_def.fields, &MySqlKeywordEscaper::default()).to_string();
        assert_eq!(field_list, "a,b,c,d,e");
    }

    #[test]
    pub fn test_gen_stream() {
        let input: DeriveInput = parse_quote! {
            #[primary(a, b)]
            struct Foo<'a, 'b> {
                a: &'a str,
                b: Cow<'b, str>,
                c: String,
                d: Option<Cow<'b, str>>,
                e: Optional<Cow<'b, str>>
            }
        };

        let table_def = TableDef::parse(&input);
        let processor = MySqlFieldProcessor::default();
        let field_list = processor.gen_list_stream(&table_def.fields).to_string();
        assert_eq!(field_list, "let fields = { let mut fields = String :: default () ; let mut has_prev = false ; fields . push_str (\"a,b,c\") ; has_prev = true ; if self . d . is_some () { fields . push_str (\",d\") ; } if self . e . is_some () { fields . push_str (\",e\") ; } fields } ;");

        let marks_list = processor.gen_marks_stream(&table_def.fields).to_string();
        assert_eq!(marks_list, "let marks = { let mut marks = String :: default () ; let mut has_prev = false ; marks . push_str (\"?,?,?\") ; has_prev = true ; if self . d . is_some () { marks . push_str (\",?\") ; } if self . e . is_some () { marks . push_str (\",?\") ; } marks } ;");
    }

    // #[primary(a, b)]
    struct Foo<'a, 'b> {
        a: &'a str,
        b: Cow<'b, str>,
        c: String,
        d: Option<Cow<'b, str>>,
        e: Option<Cow<'b, str>>,
    }

    impl<'a, 'b> Foo<'a, 'b> {
        fn gen_sql(&self) -> String {
            let fields = {
                let mut fields = String::default();
                let mut has_prev = false;
                fields.push_str("a,b,c");
                has_prev = true;
                if self.d.is_some() {
                    fields.push(',');
                    fields.push_str("d");
                    has_prev = true;
                }
                if self.e.is_some() {
                    fields.push(',');
                    fields.push_str("e");
                    has_prev = true;
                }
                fields
            };
            fields;
            let marks = {
                let mut marks = String::default();
                let mut has_prev = false;
                marks.push_str("?,?,?");
                has_prev = true;
                if self.d.is_some() {
                    marks.push(',');
                    marks.push('?');
                }
                if self.e.is_some() {
                    marks.push(',');
                    marks.push('?');
                }
                marks
            };
            marks
        }
    }
}
