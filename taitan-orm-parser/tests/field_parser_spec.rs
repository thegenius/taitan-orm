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
            c: String
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
}