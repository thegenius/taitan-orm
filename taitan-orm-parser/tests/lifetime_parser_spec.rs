
use syn::{parse_quote, DeriveInput};
use taitan_orm_parser::LifetimeParser;
use taitan_orm_parser::InputParser;
#[test]
fn lifetime_parser_extract_generics() {
    let input: DeriveInput = parse_quote! {
        struct Foo<'a> {}
    };

    let generics = input.generics;
    let lifetimes = LifetimeParser::get_generic_lifetimes(&generics);
    assert_eq!(lifetimes.len(), 1);

    let lifetime = &lifetimes[0];
    let lifetime_str = lifetime.to_string();
    assert_eq!(lifetime_str, "'a");

    let input: DeriveInput = parse_quote! {
        struct Foo<'a, 'b, 'c> {}
    };

    let generics = input.generics;
    let lifetimes = LifetimeParser::get_generic_lifetimes(&generics);
    assert_eq!(lifetimes.len(), 3);

    let lifetime_vec: Vec<String> = lifetimes.iter().map(|l|l.to_string()).collect();
    assert_eq!(lifetime_vec, vec!["'a", "'b", "'c"]);
}

#[test]
fn lifetime_parser_extract_field() {
    let input: DeriveInput = parse_quote! {
        struct Foo<'a, 'b> {
            a: &'a str,
            b: Cow<'b, str>,
            c: String
        }
    };

    let data = input.data;
    let fields = InputParser::get_fields_vec(&data).unwrap();
    let lifetime_a = LifetimeParser::get_lifetime(&fields[0].ty);
    assert!(lifetime_a.is_some());
    let lifetime_a_str = lifetime_a.unwrap().to_string();
    assert_eq!(lifetime_a_str, "'a");

    let lifetime_b = LifetimeParser::get_lifetime(&fields[1].ty);
    assert!(lifetime_b.is_some());
    let lifetime_b_str = lifetime_b.unwrap().to_string();
    assert_eq!(lifetime_b_str, "'b");

    let lifetime_c = LifetimeParser::get_lifetime(&fields[2].ty);
    assert!(lifetime_c.is_none());
}