use std::borrow::Cow;
use syn::{parse_quote, Attribute, DeriveInput};
use taitan_orm_parser::attr_parser::{AttrParser, NamedAttribute};

fn check_attr(attrs: &[Attribute], index: usize, name: &str, value_str: &str) {
    let value_opt = AttrParser::parse(&attrs[index]);
    let value = value_opt.unwrap();
    assert_eq!(value.name, Cow::<'static, str>::Owned(name.to_string()));
    let values = value_str
        .split(|c: char| c ==' ' || c == ',')
        .filter(|s| !s.is_empty())
        .map(|s| Cow::Owned(s.to_string()))
        .collect::<Vec<_>>();
    let expected: Vec<Cow<'static, str>> = values;
    assert_eq!(value.values, expected);
}
#[test]
fn attr_parser_struct_spec() {
    let input: DeriveInput = parse_quote! {
        #[table1("user1")]
        #[table2 = "user2"]
        #[table3(user3)]
        #[table4 = user4]
        #[table5 = "user50, user51"]
        #[table6(user60, user61)]
        struct Foo<'a, 'b> {
            a: &'a str,
            b: Cow<'b, str>,
            c: String,
            d: Option<Cow<'b, str>>,
            e: Optional<Cow<'b, str>>
        }
    };

    let attrs = input.attrs;
    check_attr(&attrs, 0, "table1", "user1");
    check_attr(&attrs, 1, "table2", "user2");
    check_attr(&attrs, 2, "table3", "user3");
    check_attr(&attrs, 3, "table4", "user4");
    check_attr(&attrs, 4, "table5", "user50,  user51");
    check_attr(&attrs, 5, "table6", "user60,  user61");
}

#[test]
fn attr_parser_spec_multi() {
    let input: DeriveInput = parse_quote! {
        #[table1(index = user60, fields = (f1, f2, f3))]
        #[table2(index = user60, fields = "f1, f2, f3")]
        struct Foo<'a, 'b> {
            a: &'a str,
            b: Cow<'b, str>,
            c: String,
            d: Option<Cow<'b, str>>,
            e: Optional<Cow<'b, str>>
        }
    };

    let attrs = input.attrs;
    let named_attrs = AttrParser::parse_multi(&attrs[0]);
    let named_attr0 = named_attrs[0].clone();
    let expected_named_attr = NamedAttribute::from_str("index", "user60");
    assert_eq!(named_attr0, expected_named_attr);

    let named_attr1 = named_attrs[1].clone();
    let expected_named_attr = NamedAttribute::from_str("fields", "f1, f2, f3");
    assert_eq!(named_attr1, expected_named_attr);

    let named_attrs = AttrParser::parse_multi(&attrs[1]);
    let named_attr0 = named_attrs[0].clone();
    let expected_named_attr = NamedAttribute::from_str("index", "user60");
    assert_eq!(named_attr0, expected_named_attr);

    let named_attr1 = named_attrs[1].clone();
    let expected_named_attr = NamedAttribute::from_str("fields", "f1, f2,  f3");
    assert_eq!(named_attr1, expected_named_attr);

}