use std::borrow::{Borrow, Cow};
use syn::{parse_quote, DeriveInput, Field};
use taitan_orm_parser::{
    FieldDef, FieldName, InputParser, NamedVariant, ParsedField, TableColumnDef,
};

#[test]
pub fn enum_parser_spec_001() {
    let input: DeriveInput = parse_quote! {
        enum Foo<'a, 'b> {
            A(&'a str),
            B(Cow<'b, str>),
            C(Option<Expr<Cow<'b, str>>>),
            D(Uuid)
        }
    };
    let variants = InputParser::get_enum_variant(&input.data).unwrap();

    let expect_struct_field = ParsedField {
        name: FieldName::unnamed(0),
        rust_type: Cow::Borrowed("& 'a str"),
        option_nest_level: 0,
        is_location_expr: false,
        is_enum_variant: true,
        lifetime: Some(Cow::Borrowed("'a")),
        origin_field: variants.get(0).unwrap().fields.first().unwrap().clone(),
    };

    let field = variants.get(0).unwrap();
    let actual_field_def = FieldDef::parse(field.fields.first().unwrap(), true, Some(0), None);
    assert_eq!(field.name, "A");
    assert_eq!(actual_field_def.struct_field, expect_struct_field);

    let expect_struct_field = ParsedField {
        name: FieldName::unnamed(1),
        rust_type: Cow::Borrowed("Cow < 'b , str >"),
        option_nest_level: 0,
        is_location_expr: false,
        is_enum_variant: true,
        lifetime: Some(Cow::Borrowed("'b")),
        origin_field: variants.get(1).unwrap().fields.first().unwrap().clone(),
    };
    let field = variants.get(1).unwrap();
    let actual_field_def = FieldDef::parse(field.fields.first().unwrap(), true, Some(1), None);
    assert_eq!(field.name, "B");
    assert_eq!(actual_field_def.struct_field, expect_struct_field);

    let field = variants.get(2).unwrap();
    let actual_field_def = FieldDef::parse(field.fields.first().unwrap(), true, Some(2), None);
    let expect_struct_field = ParsedField {
        name: FieldName::unnamed(2),
        rust_type: Cow::Borrowed("Expr < Cow < 'b , str > >"),
        option_nest_level: 1,
        is_location_expr: true,
        is_enum_variant: true,
        lifetime: Some(Cow::Borrowed("'b")),
        origin_field:(variants.get(2).unwrap().fields.first().unwrap().clone()),
    };
    assert_eq!(field.name, "C");
    assert_eq!(actual_field_def.struct_field, expect_struct_field);

    let field = variants.get(3).unwrap();
    let actual_field_def = FieldDef::parse(field.fields.first().unwrap(), true, Some(3), None);
    let expect_struct_field = ParsedField {
        name: FieldName::unnamed(3),
        rust_type: Cow::Borrowed("Uuid"),
        option_nest_level: 0,
        is_location_expr: false,
        is_enum_variant: true,
        lifetime: None,
        origin_field:variants.get(3).unwrap().fields.first().unwrap().clone(),
    };
    assert_eq!(field.name, "D");
    assert_eq!(actual_field_def.struct_field, expect_struct_field);
}

#[test]
pub fn enum_parser_spec_002() {
    let input: DeriveInput = parse_quote! {
        enum Foo<'a, 'b> {
            A { f1: &'a str, f2: Cow<'b, str>, f3: String },
        }
    };
    let variants = InputParser::get_enum_variant(&input.data).unwrap();

    let field = variants.get(0).unwrap();
    assert_eq!(field.name, "A");

    let actual_field_def = FieldDef::parse(field.fields.get(0).unwrap(), true, None, None);
    let expect_struct_field = ParsedField {
        name: FieldName::named("f1"),
        rust_type: Cow::Borrowed("& 'a str"),
        option_nest_level: 0,
        is_location_expr: false,
        is_enum_variant: true,
        lifetime: Some(Cow::Borrowed("'a")),
        origin_field: variants.first().unwrap().fields.get(0).cloned().unwrap(),
    };
    assert_eq!(actual_field_def.struct_field, expect_struct_field);

    let actual_field_def = FieldDef::parse(field.fields.get(1).unwrap(), true, None, None);
    let expect_struct_field = ParsedField {
        name: FieldName::named("f2"),
        rust_type: Cow::Borrowed("Cow < 'b , str >"),
        option_nest_level: 0,
        is_location_expr: false,
        is_enum_variant: true,
        lifetime: Some(Cow::Borrowed("'b")),
        origin_field: variants.first().unwrap().fields.get(1).cloned().unwrap(),
    };
    assert_eq!(actual_field_def.struct_field, expect_struct_field);

    let actual_field_def = FieldDef::parse(field.fields.get(2).unwrap(), true, None, None);
    let expect_struct_field = ParsedField {
        name: FieldName::named("f3"),
        rust_type: Cow::Borrowed("String"),
        option_nest_level: 0,
        is_location_expr: false,
        is_enum_variant: true,
        lifetime: None,
        origin_field: variants.first().unwrap().fields.get(2).cloned().unwrap(),
    };
    assert_eq!(actual_field_def.struct_field, expect_struct_field);
}

#[test]
pub fn enum_parser_spec_003() {
    let input: DeriveInput = parse_quote! {
        enum Foo<'a, 'b> {
            A {
                #[field(name = user_name1, column_type = VARCHAR, nullable = true, auto_inc = true)]
                f1: &'a str,

                #[field(name = user_name2, column_type = VARCHAR, nullable = true, auto_inc = false)]
                f2: Cow<'b, str>,

                f3: String
            },
        }
    };
    let variants = InputParser::get_enum_variant(&input.data).unwrap();

    let field = variants.get(0).unwrap();
    assert_eq!(field.name, "A");

    let actual_field_def = FieldDef::parse(field.fields.get(0).unwrap(), true, None, None);
    let expect_struct_field = ParsedField {
        name: FieldName::named("f1"),
        rust_type: Cow::Borrowed("& 'a str"),
        option_nest_level: 0,
        is_location_expr: false,
        is_enum_variant: true,
        lifetime: Some(Cow::Borrowed("'a")),
        origin_field: variants.first().unwrap().fields.get(0).cloned().unwrap(),
    };
    let expect_column_field = TableColumnDef {
        name: Some(Cow::Borrowed("user_name1")),
        column_type: Some(Cow::Borrowed("VARCHAR")),
        default_value: None,
        generated: None,
        nullable: true,
        auto_inc: true,
    };
    assert_eq!(actual_field_def.struct_field, expect_struct_field);
    assert_eq!(actual_field_def.table_column, expect_column_field);

    let actual_field_def = FieldDef::parse(field.fields.get(1).unwrap(), true, None, None);
    let expect_struct_field = ParsedField {
        name: FieldName::named("f2"),
        rust_type: Cow::Borrowed("Cow < 'b , str >"),
        option_nest_level: 0,
        is_location_expr: false,
        is_enum_variant: true,
        lifetime: Some(Cow::Borrowed("'b")),
        origin_field: variants.first().unwrap().fields.get(1).cloned().unwrap(),
    };
    assert_eq!(actual_field_def.struct_field, expect_struct_field);

    let actual_field_def = FieldDef::parse(field.fields.get(2).unwrap(), true, None, None);
    let expect_struct_field = ParsedField {
        name: FieldName::named("f3"),
        rust_type: Cow::Borrowed("String"),
        option_nest_level: 0,
        is_location_expr: false,
        is_enum_variant: true,
        lifetime: None,
        origin_field: variants.first().unwrap().fields.get(2).cloned().unwrap(),
    };
    assert_eq!(actual_field_def.struct_field, expect_struct_field);
}

#[test]
pub fn enum_parser_spec_004() {
    let input: DeriveInput = parse_quote! {
        enum Foo<'a, 'b> {
            A(
                #[field(name = user_name1, column_type = VARCHAR, nullable = true, auto_inc = true)]
                &'a str
            )
        }
    };
    let variants = InputParser::get_enum_variant(&input.data).unwrap();

    let field = variants.get(0).unwrap();
    assert_eq!(field.name, "A");

    let actual_field_def = FieldDef::parse(field.fields.get(0).unwrap(), true, Some(0), None);
    let expect_struct_field = ParsedField {
        name: FieldName::default(),
        rust_type: Cow::Borrowed("& 'a str"),
        option_nest_level: 0,
        is_location_expr: false,
        is_enum_variant: true,
        lifetime: Some(Cow::Borrowed("'a")),
        origin_field: variants.first().unwrap().fields.get(0).unwrap().clone(),
    };
    let expect_column_field = TableColumnDef {
        name: Some(Cow::Borrowed("user_name1")),
        column_type: Some(Cow::Borrowed("VARCHAR")),
        default_value: None,
        generated: None,
        nullable: true,
        auto_inc: true,
    };
    assert_eq!(actual_field_def.struct_field, expect_struct_field);
    assert_eq!(actual_field_def.table_column, expect_column_field);
}
