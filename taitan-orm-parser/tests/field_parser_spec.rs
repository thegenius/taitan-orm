use std::borrow::Cow;
use syn::{parse_quote, DeriveInput, Field};
use taitan_orm_parser::{InputParser, StructFieldDef};
use taitan_orm_parser::FieldParser;

fn check_expected(fields: &Vec<Field>, index: usize, expected: &StructFieldDef) {
    let field_def = FieldParser::parse(fields.get(index).unwrap());
    assert_eq!(&field_def.struct_field, expected);
}

#[test]
pub fn field_parser_spec() {
    let input: DeriveInput = parse_quote! {
        struct Foo<'a, 'b> {
            a: &'a str,
            b: Cow<'b, str>,
            c: String,
            d: Option<Cow<'b, str>>,
            e: Optional<Cow<'b, str>>
        }
    };
    let fields = InputParser::get_fields_vec(&input.data).unwrap();

    let expect_struct_field = StructFieldDef {
        name: Cow::Borrowed("a"),
        rust_type: Cow::Borrowed("& 'a str"),
        is_optional: false,
        lifetime: Some(Cow::Borrowed("'a")),
    };
    check_expected(&fields, 0, &expect_struct_field);

    let expect_struct_field = StructFieldDef {
        name: Cow::Borrowed("b"),
        rust_type: Cow::Borrowed("Cow < 'b , str >"),
        is_optional: false,
        lifetime: Some(Cow::Borrowed("'b")),
    };
    check_expected(&fields, 1, &expect_struct_field);

    let expect_struct_field = StructFieldDef {
        name: Cow::Borrowed("c"),
        rust_type: Cow::Borrowed("String"),
        is_optional: false,
        lifetime: None,
    };
    check_expected(&fields, 2, &expect_struct_field);

    let expect_struct_field = StructFieldDef {
        name: Cow::Borrowed("d"),
        rust_type: Cow::Borrowed("Cow < 'b , str >"),
        is_optional: true,
        lifetime: Some(Cow::Borrowed("'b")),
    };
    check_expected(&fields, 3, &expect_struct_field);

    let expect_struct_field = StructFieldDef {
        name: Cow::Borrowed("e"),
        rust_type: Cow::Borrowed("Cow < 'b , str >"),
        is_optional: true,
        lifetime: Some(Cow::Borrowed("'b")),
    };
    check_expected(&fields, 4, &expect_struct_field);
}