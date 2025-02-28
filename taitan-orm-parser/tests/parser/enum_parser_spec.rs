use std::borrow::{Borrow, Cow};
use syn::{parse_quote, DeriveInput, Field};
use taitan_orm_parser::{FieldName, FieldParser, InputParser, NamedVariant, StructFieldDef, TableColumnDef};



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
    let fields = InputParser::get_enum_variant(&input.data).unwrap();

    let expect_struct_field = StructFieldDef {
        name: FieldName::unnamed(0),
        rust_type: Cow::Borrowed("& 'a str"),
        is_optional: false,
        is_location_expr: false,
        is_enum_variant: true,
        lifetime: Some(Cow::Borrowed("'a")),
    };

    let field = fields.get(0).unwrap();
    let actual_field_def = FieldParser::parse(field.fields.first().unwrap(), true, Some(0), None);
    assert_eq!(field.name, "A");
    assert_eq!(actual_field_def.struct_field, expect_struct_field);

    let expect_struct_field = StructFieldDef {
        name: FieldName::unnamed(1),
        rust_type: Cow::Borrowed("Cow < 'b , str >"),
        is_optional: false,
        is_location_expr: false,
        is_enum_variant: true,
        lifetime: Some(Cow::Borrowed("'b")),
    };
    let field = fields.get(1).unwrap();
    let actual_field_def = FieldParser::parse(field.fields.first().unwrap(), true, Some(1), None);
    assert_eq!(field.name, "B");
    assert_eq!(actual_field_def.struct_field, expect_struct_field);

    let field = fields.get(2).unwrap();
    let actual_field_def = FieldParser::parse(field.fields.first().unwrap(), true, Some(2), None);
    let expect_struct_field = StructFieldDef {
        name: FieldName::unnamed(2),
        rust_type: Cow::Borrowed("Expr < Cow < 'b , str > >"),
        is_optional: true,
        is_location_expr: false,
        is_enum_variant: true,
        lifetime: Some(Cow::Borrowed("'b")),
    };
    assert_eq!(field.name, "C");
    assert_eq!(actual_field_def.struct_field, expect_struct_field);

    let field = fields.get(3).unwrap();
    let actual_field_def = FieldParser::parse(field.fields.first().unwrap(), true, Some(3), None);
    let expect_struct_field = StructFieldDef {
        name: FieldName::unnamed(3),
        rust_type: Cow::Borrowed("Uuid"),
        is_optional: false,
        is_location_expr: false,
        is_enum_variant: true,
        lifetime: None,
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
    let fields = InputParser::get_enum_variant(&input.data).unwrap();

    let field = fields.get(0).unwrap();
    assert_eq!(field.name, "A");

    let actual_field_def = FieldParser::parse(field.fields.get(0).unwrap(), true, None, None);
    let expect_struct_field = StructFieldDef {
        name: FieldName::named("f1"),
        rust_type: Cow::Borrowed("& 'a str"),
        is_optional: false,
        is_location_expr: false,
        is_enum_variant: true,
        lifetime: Some(Cow::Borrowed("'a")),
    };
    assert_eq!(actual_field_def.struct_field, expect_struct_field);

    let actual_field_def = FieldParser::parse(field.fields.get(1).unwrap(), true, None, None);
    let expect_struct_field = StructFieldDef {
        name: FieldName::named("f2"),
        rust_type: Cow::Borrowed("Cow < 'b , str >"),
        is_optional: false,
        is_location_expr: false,
        is_enum_variant: true,
        lifetime: Some(Cow::Borrowed("'b")),
    };
    assert_eq!(actual_field_def.struct_field, expect_struct_field);

    let actual_field_def = FieldParser::parse(field.fields.get(2).unwrap(), true, None, None);
    let expect_struct_field = StructFieldDef {
        name: FieldName::named("f3"),
        rust_type: Cow::Borrowed("String"),
        is_optional: false,
        is_location_expr: false,
        is_enum_variant: true,
        lifetime: None,
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
    let fields = InputParser::get_enum_variant(&input.data).unwrap();

    let field = fields.get(0).unwrap();
    assert_eq!(field.name, "A");

    let actual_field_def = FieldParser::parse(field.fields.get(0).unwrap(), true, None, None);
    let expect_struct_field = StructFieldDef {
        name: FieldName::named("f1"),
        rust_type: Cow::Borrowed("& 'a str"),
        is_optional: false,
        is_location_expr: false,
        is_enum_variant: true,
        lifetime: Some(Cow::Borrowed("'a")),
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

    let actual_field_def = FieldParser::parse(field.fields.get(1).unwrap(), true, None, None);
    let expect_struct_field = StructFieldDef {
        name: FieldName::named("f2"),
        rust_type: Cow::Borrowed("Cow < 'b , str >"),
        is_optional: false,
        is_location_expr: false,
        is_enum_variant: true,
        lifetime: Some(Cow::Borrowed("'b")),
    };
    assert_eq!(actual_field_def.struct_field, expect_struct_field);

    let actual_field_def = FieldParser::parse(field.fields.get(2).unwrap(), true, None, None);
    let expect_struct_field = StructFieldDef {
        name: FieldName::named("f3"),
        rust_type: Cow::Borrowed("String"),
        is_optional: false,
        is_location_expr: false,
        is_enum_variant: true,
        lifetime: None,
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
    let fields = InputParser::get_enum_variant(&input.data).unwrap();

    let field = fields.get(0).unwrap();
    assert_eq!(field.name, "A");

    let actual_field_def = FieldParser::parse(field.fields.get(0).unwrap(), true, Some(0), None);
    let expect_struct_field = StructFieldDef {
        name: FieldName::default(),
        rust_type: Cow::Borrowed("& 'a str"),
        is_optional: false,
        is_location_expr: false,
        is_enum_variant: true,
        lifetime: Some(Cow::Borrowed("'a")),
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