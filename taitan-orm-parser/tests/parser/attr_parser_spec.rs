use std::borrow::Cow;
use syn::{parse_quote, Attribute, DeriveInput};
use syn::Expr::Field;
use taitan_orm_parser::attr_parser::{AttrParser, NamedAttribute};
use taitan_orm_parser::{InputParser};

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

fn check_multi_attrs(attrs: &[Attribute], index: usize, attr0: &NamedAttribute, attr1: &NamedAttribute) {
    let named_attrs = AttrParser::parse_list(&attrs[index]);
    let named_attr0 = named_attrs[0].clone();
    let named_attr1 = named_attrs[1].clone();
    assert_eq!(&named_attr0, attr0);
    assert_eq!(&named_attr1, attr1);
}

#[test]
fn attr_parser_spec_multi() {
    let input: DeriveInput = parse_quote! {
        #[table1(index = user60, fields = (f1, f2, f3))]
        #[table2(index = user60, fields = "f1, f2, f3")]
        #[table3(index(user60), fields(f1, f2, f3))]
        struct Foo<'a, 'b> {
            a: &'a str,
            b: Cow<'b, str>,
            c: String,
            d: Option<Cow<'b, str>>,
            e: Optional<Cow<'b, str>>
        }
    };

    let attrs = input.attrs;
    let expected_attr1 = NamedAttribute::from_str("index", "user60");
    let expected_attr2 = NamedAttribute::from_str("fields", "f1, f2, f3");
    check_multi_attrs(&attrs, 0, &expected_attr1, &expected_attr2);
    check_multi_attrs(&attrs, 1, &expected_attr1, &expected_attr2);
    check_multi_attrs(&attrs, 2, &expected_attr1, &expected_attr2);
}


#[test]
fn attr_parser_spec_field_multi() {
    let input: DeriveInput = parse_quote! {
        struct Foo<'a> {
            #[field(name = user_name, db_type = BIGINT, nullable = true, auto_inc = true)]
            name: &'a str,
        }
    };

    let fields = InputParser::get_fields(&input.data);
    let name_field = &fields[0];
    let field_attrs = &name_field.attrs;
    let name_field_attr = &field_attrs[0];
    let named_attr_list = AttrParser::parse_list(name_field_attr);
    let expected_named_attr = NamedAttribute::from_str("name", "user_name");
    assert_eq!(named_attr_list[0], expected_named_attr);
    let expected_named_attr = NamedAttribute::from_str("db_type", "BIGINT");
    assert_eq!(named_attr_list[1], expected_named_attr);
    let expected_named_attr = NamedAttribute::from_str("nullable", "true");
    assert_eq!(named_attr_list[2], expected_named_attr);
    let expected_named_attr = NamedAttribute::from_str("auto_inc", "true");
    assert_eq!(named_attr_list[3], expected_named_attr);
}