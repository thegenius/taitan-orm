use syn::{parse_quote, DeriveInput};
use taitan_orm_parser::{NamesMapper, TableDef};
use taitan_orm_parser::MultiFieldMapper;
use taitan_orm_parser::MySqlKeywordEscaper;
#[test]
fn test_name_mapper() {
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
    let names_mapper = NamesMapper::default();
    let escaper = MySqlKeywordEscaper::default();
    let names = names_mapper.map(&table_def.fields, &escaper).to_string();


    assert_eq!(names, "a, b, c, d, e");
}