use std::borrow::{Borrow, Cow};
use syn::{parse_quote, DeriveInput, Field};
use taitan_orm_parser::{FieldDef, FieldName, InputParser, NamedVariant, ParsedField, TableColumnDef};


fn check_expected<T>(fields: &[T], index: usize, expected: &ParsedField) where T: Borrow<Field> {
    let field_def = FieldDef::parse(fields.get(index).unwrap().borrow(), false, None, None);
    assert_eq!(&field_def.struct_field, expected);
}

#[test]
pub fn field_parser_spec_struct() {
    let input: DeriveInput = parse_quote! {
        struct Foo<'a, 'b> {
            a: &'a str,
            b: Cow<'b, str>,
            c: String,
            d: Option<Cow<'b, str>>
        }
    };
    let fields = InputParser::get_fields(&input.data);

    let expect_struct_field = ParsedField {
        name: FieldName::named("a"),
        rust_type: Cow::Borrowed("& 'a str"),
        option_nest_level: 0,
        is_location_expr: false,
        is_enum_variant: false,
        lifetime: Some(Cow::Borrowed("'a")),
        origin_field: fields.get(0).map(|f|f.clone().clone()).unwrap(),
    };
    check_expected(&fields, 0, &expect_struct_field);

    let expect_struct_field = ParsedField {
        name: FieldName::named("b"),
        rust_type: Cow::Borrowed("Cow < 'b , str >"),
          option_nest_level: 0,
        is_location_expr: false,
        is_enum_variant: false,
        lifetime: Some(Cow::Borrowed("'b")),
        origin_field: fields.get(1).map(|f|f.clone().clone()).unwrap()
    };
    check_expected(&fields, 1, &expect_struct_field);

    let expect_struct_field = ParsedField {
        name: FieldName::named("c"),
        rust_type: Cow::Borrowed("String"),
          option_nest_level: 0,
        is_enum_variant: false,
        is_location_expr: false,
        lifetime: None,
        origin_field: fields.get(2).map(|f|f.clone().clone()).unwrap()
    };
    check_expected(&fields, 2, &expect_struct_field);

    let expect_struct_field = ParsedField {
        name: FieldName::named("d"),
        rust_type: Cow::Borrowed("Cow < 'b , str >"),
        option_nest_level: 1,
        is_location_expr: false,
        is_enum_variant: false,
        lifetime: Some(Cow::Borrowed("'b")),
        origin_field: fields.get(3).map(|f|f.clone().clone()).unwrap()
    };
    check_expected(&fields, 3, &expect_struct_field);
}

#[test]
pub fn field_parser_spec_with_attr() {
    let input: DeriveInput = parse_quote! {
        struct Foo<'a> {
            #[field(name = user_name, column_type = BIGINT, nullable = true, auto_inc = true, generated = "CONCAT(first_name, ' ', last_name)")]
            e: Option<Cow<'a, str>>
        }
    };
    let fields = InputParser::get_fields(&input.data);
    let field = fields.get(0).unwrap();
    let field_def = FieldDef::parse(field, false, None, None);

    let expect_struct_field = ParsedField {
        name: FieldName::named("e"),
        rust_type: Cow::Borrowed("Cow < 'a , str >"),
        option_nest_level: 1,
        is_location_expr: false,
        is_enum_variant: false,
        lifetime: Some(Cow::Borrowed("'a")),
        origin_field: fields.get(0).map(|f|f.clone().clone()).unwrap(),
    };
    let expect_column_def = TableColumnDef {
        name: Some(Cow::Borrowed("user_name")),
        column_type: Some(Cow::Borrowed("BIGINT")),
        default_value: None,
        generated: Some(Cow::Borrowed("CONCAT(first_name, ' ', last_name)")),
        nullable: true,
        auto_inc: true,
    };

    assert_eq!(field_def.struct_field, expect_struct_field);
    assert_eq!(field_def.table_column, expect_column_def);
}

#[test]
pub fn field_parser_spec_enum() {
    let input: DeriveInput = parse_quote! {
        enum Foo<'a, 'b> {
            A{field1: &'a str, field2: Option<Cow<'b, str>>},
            B{field3: String, field4: Optional<Cow<'b, str>>},
        }
    };
    let variants = InputParser::get_enum_variant(&input.data).unwrap();
    assert_eq!(variants[0].name, Cow::Borrowed("A"));

    let fields = &variants[0].fields;
    let expect_struct_field = ParsedField {
        name: FieldName::named("field1"),
        rust_type: Cow::Borrowed("& 'a str"),
          option_nest_level: 0,
        is_location_expr: false,
        is_enum_variant: false,
        lifetime: Some(Cow::Borrowed("'a")),
        origin_field: variants.first().unwrap().fields.get(0).map(|f|f.clone().clone()).unwrap(),
    };
    check_expected(fields, 0, &expect_struct_field);
    let expect_struct_field = ParsedField {
        name: FieldName::named("field2"),
        rust_type: Cow::Borrowed("Cow < 'b , str >"),
        option_nest_level: 1,
        is_location_expr: false,
        is_enum_variant: false,
        lifetime: Some(Cow::Borrowed("'b")),
        origin_field: variants.first().unwrap().fields.get(1).map(|f|f.clone().clone()).unwrap(),
    };
    check_expected(fields, 1, &expect_struct_field);
}

