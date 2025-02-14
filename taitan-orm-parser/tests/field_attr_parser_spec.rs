use std::borrow::Cow;
use syn::{parse_quote, DeriveInput};
use taitan_orm_parser::{FieldAttrParser, InputParser, TableColumnDef};
use taitan_orm_parser::attr_parser::AttrParser;

#[test]
pub fn field_parser_spec_column() {
    let input: DeriveInput = parse_quote! {
        struct Foo<'a, 'b> {
            #[field(name = user_name, column_type = BIGINT, nullable = true, auto_inc = true, generated = "CONCAT(first_name, ' ', last_name)")]
            e: Optional<Cow<'b, str>>
        }
    };
    let fields = InputParser::get_fields(&input.data);
    let field = fields.first().unwrap();
    let attr = AttrParser::get_attr(&field.attrs, "field").unwrap();
    let column_def = FieldAttrParser::parse_column_def(&attr);
    let expect_column_def = TableColumnDef {
        name: Some(Cow::Borrowed("user_name")),
        column_type: Some(Cow::Borrowed("BIGINT")),
        default_value: None,
        generated: Some(Cow::Borrowed("CONCAT(first_name, ' ', last_name)")),
        nullable: true,
        auto_inc: true,
    };
    assert_eq!(column_def, expect_column_def);

}